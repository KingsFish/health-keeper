//! Sync module - multi-device synchronization

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Sync status for an entity
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Not yet synced
    Pending,
    /// Successfully synced
    Synced,
    /// Conflict detected
    Conflict,
    /// Sync in progress
    InProgress,
}

/// Sync state for tracking changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncState {
    /// Entity type (person, visit, attachment)
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// Version number (incremented on each change)
    pub version: i64,
    /// Current sync status
    pub status: SyncStatus,
    /// Last sync timestamp
    pub last_sync_at: Option<DateTime<Utc>>,
    /// Local modification timestamp
    pub modified_at: DateTime<Utc>,
}

impl SyncState {
    pub fn new(entity_type: &str, entity_id: &str) -> Self {
        Self {
            entity_type: entity_type.to_string(),
            entity_id: entity_id.to_string(),
            version: 1,
            status: SyncStatus::Pending,
            last_sync_at: None,
            modified_at: Utc::now(),
        }
    }

    pub fn bump_version(&mut self) {
        self.version += 1;
        self.modified_at = Utc::now();
        self.status = SyncStatus::Pending;
    }

    pub fn mark_synced(&mut self) {
        self.status = SyncStatus::Synced;
        self.last_sync_at = Some(Utc::now());
    }
}

/// Sync conflict record
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncConflict {
    /// Conflict ID
    pub id: String,
    /// Entity type
    pub entity_type: String,
    /// Entity ID
    pub entity_id: String,
    /// Local version
    pub local_version: i64,
    /// Server version
    pub server_version: i64,
    /// Conflict detected at
    pub detected_at: DateTime<Utc>,
    /// Resolution strategy
    pub resolution: Option<ConflictResolution>,
}

/// Conflict resolution strategy
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConflictResolution {
    /// Keep local version
    KeepLocal,
    /// Use server version
    UseServer,
    /// Manual merge required
    ManualMerge,
}

/// Placeholder for sync service
/// Full implementation will include:
/// - WebSocket or SSE connection
/// - Change detection and batching
/// - Conflict detection and resolution
/// - Offline queue management
/// - Encryption for data in transit

pub struct SyncService {
    _enabled: bool,
    _server_url: Option<String>,
}

impl SyncService {
    pub fn new(enabled: bool, server_url: Option<String>) -> Self {
        Self {
            _enabled: enabled,
            _server_url: server_url,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self._enabled
    }

    // TODO: Implement actual sync logic
    // - connect()
    // - sync()
    // - push_changes()
    // - pull_changes()
    // - resolve_conflict()
}