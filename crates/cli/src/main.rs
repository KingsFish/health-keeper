//! HealthKeeper CLI - Command-line interface for medical records management

use anyhow::Result;
use clap::{Parser, Subcommand};
use health_keeper_core::{
    models::{AttachmentType, Gender, Person, PersonBuilder, Relationship, Visit, VisitBuilder},
    storage::{SqliteStorage, Storage},
    AppConfig,
};
use std::path::PathBuf;

mod commands;

#[derive(Parser)]
#[command(name = "hk")]
#[command(about = "HealthKeeper - Medical records management CLI", long_about = None)]
#[command(version)]
struct Cli {
    /// Configuration file path
    #[arg(short, long, global = true)]
    config: Option<PathBuf>,

    /// Data directory
    #[arg(short, long, global = true)]
    data_dir: Option<PathBuf>,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Manage person records
    #[command(subcommand)]
    Person(PersonCommands),

    /// Manage visit records
    #[command(subcommand)]
    Visit(VisitCommands),

    /// Import and manage attachments
    Import(ImportCommand),

    /// Run OCR on attachments
    Ocr(OcrCommand),

    /// Extract structured data using LLM
    Extract(ExtractCommand),

    /// Search records
    Search(SearchCommand),

    /// Initialize the database
    Init,

    /// Show configuration
    Config,
}

#[derive(Subcommand)]
enum PersonCommands {
    /// Create a new person
    Create {
        /// Person's name
        #[arg(short, long)]
        name: String,

        /// Relationship to user (self, spouse, child, parent, sibling, other)
        #[arg(short = 'r', long, default_value = "self")]
        relationship: String,

        /// Date of birth (YYYY-MM-DD)
        #[arg(short = 'D', long)]
        birth_date: Option<String>,

        /// Gender (male, female, other)
        #[arg(short, long)]
        gender: Option<String>,

        /// Blood type (A, B, AB, O)
        #[arg(short = 'B', long)]
        blood_type: Option<String>,

        /// Notes
        #[arg(short = 'N', long)]
        notes: Option<String>,
    },

    /// List all persons
    List,

    /// Show person details
    Show {
        /// Person ID
        id: String,
    },

    /// Update a person
    Update {
        /// Person ID
        id: String,

        /// Person's name
        #[arg(short, long)]
        name: Option<String>,

        /// Relationship to user
        #[arg(short = 'r', long)]
        relationship: Option<String>,

        /// Date of birth (YYYY-MM-DD)
        #[arg(short = 'D', long)]
        birth_date: Option<String>,

        /// Gender (male, female, other)
        #[arg(short, long)]
        gender: Option<String>,

        /// Blood type (A, B, AB, O)
        #[arg(short = 'B', long)]
        blood_type: Option<String>,

        /// Notes
        #[arg(short = 'N', long)]
        notes: Option<String>,
    },

    /// Delete a person
    Delete {
        /// Person ID
        id: String,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Subcommand)]
enum VisitCommands {
    /// Create a new visit record
    Create {
        /// Person ID
        #[arg(short, long)]
        person: String,

        /// Visit date (YYYY-MM-DD)
        #[arg(short = 'D', long)]
        date: String,

        /// Hospital name
        #[arg(short = 'H', long)]
        hospital: Option<String>,

        /// Department
        #[arg(short = 'e', long)]
        department: Option<String>,

        /// Doctor name
        #[arg(short = 'o', long)]
        doctor: Option<String>,

        /// Chief complaint
        #[arg(short = 'C', long)]
        complaint: Option<String>,

        /// Diagnosis
        #[arg(short = 'i', long)]
        diagnosis: Option<String>,

        /// Treatment
        #[arg(short = 't', long)]
        treatment: Option<String>,

        /// Notes
        #[arg(short = 'N', long)]
        notes: Option<String>,
    },

    /// List visit records
    List {
        /// Filter by person ID
        #[arg(short, long)]
        person: Option<String>,
    },

    /// Show visit details
    Show {
        /// Visit ID
        id: String,
    },

    /// Update a visit record
    Update {
        /// Visit ID
        id: String,

        /// Visit date (YYYY-MM-DD)
        #[arg(short = 'D', long)]
        date: Option<String>,

        /// Hospital name
        #[arg(short = 'H', long)]
        hospital: Option<String>,

        /// Department
        #[arg(short = 'e', long)]
        department: Option<String>,

        /// Doctor name
        #[arg(short = 'o', long)]
        doctor: Option<String>,

        /// Chief complaint
        #[arg(short = 'C', long)]
        complaint: Option<String>,

        /// Diagnosis
        #[arg(short = 'i', long)]
        diagnosis: Option<String>,

        /// Treatment
        #[arg(short = 't', long)]
        treatment: Option<String>,

        /// Notes
        #[arg(short = 'N', long)]
        notes: Option<String>,
    },

    /// Delete a visit record
    Delete {
        /// Visit ID
        id: String,

        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
}

#[derive(Parser)]
struct ImportCommand {
    /// Visit ID to attach the file to
    #[arg(short, long)]
    visit: String,

    /// File path to import
    #[arg(short, long)]
    file: PathBuf,

    /// Attachment type (medical_record, lab_report, prescription, imaging, invoice, other)
    #[arg(short = 't', long, default_value = "other")]
    attachment_type: String,
}

#[derive(Parser)]
struct OcrCommand {
    /// Attachment ID
    #[arg(short, long)]
    attachment: String,

    /// OCR provider to use
    #[arg(short = 'p', long)]
    provider: Option<String>,
}

#[derive(Parser)]
struct ExtractCommand {
    /// Attachment ID
    #[arg(short, long)]
    attachment: String,

    /// LLM provider to use
    #[arg(short = 'p', long)]
    provider: Option<String>,
}

#[derive(Parser)]
struct SearchCommand {
    /// Search query
    query: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();

    // Load configuration
    let config = if let Some(path) = &cli.config {
        AppConfig::from_file(path)?
    } else {
        AppConfig::load()?
    };

    // Override data directory if specified
    let data_dir = cli
        .data_dir
        .unwrap_or_else(|| config.data_dir());

    // Ensure data directory exists
    std::fs::create_dir_all(&data_dir)?;

    // Initialize storage
    let storage = SqliteStorage::new(&config.database_url()).await?;

    // Run migration on init or first use
    storage.migrate().await?;

    // Execute command
    match cli.command {
        Commands::Init => {
            println!("Database initialized at {}", config.database_url());
        }
        Commands::Config => {
            println!("{}", serde_yaml::to_string(&config)?);
        }
        Commands::Person(cmd) => commands::handle_person(&storage, cmd).await?,
        Commands::Visit(cmd) => commands::handle_visit(&storage, cmd).await?,
        Commands::Import(cmd) => commands::handle_import(&storage, cmd).await?,
        Commands::Ocr(cmd) => commands::handle_ocr(&storage, &config, cmd).await?,
        Commands::Extract(cmd) => commands::handle_extract(&storage, &config, cmd).await?,
        Commands::Search(cmd) => commands::handle_search(&storage, cmd).await?,
    }

    storage.close().await;
    Ok(())
}