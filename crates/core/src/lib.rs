//! HealthKeeper Core Library
//!
//! This library provides the core functionality for managing medical records,
//! including data models, storage, AI capabilities, and synchronization.

pub mod ai;
pub mod config;
pub mod models;
pub mod storage;
pub mod sync;

pub use config::AppConfig;
pub use models::*;
pub use storage::{Storage, StorageError};

/// Core error type for the library
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Storage error: {0}")]
    Storage(#[from] StorageError),

    #[error("AI error: {0}")]
    Ai(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Not found: {0}")]
    NotFound(String),
}

pub type Result<T> = std::result::Result<T, Error>;