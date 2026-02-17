//! Y-sync protocol implementation for Jupyter notebook collaboration.
//!
//! This module provides the wire protocol for synchronizing Y.Doc documents
//! between clients. It wraps the yrs sync protocol types and provides
//! jupyter-specific extensions.
//!
//! ## Protocol Overview
//!
//! The y-sync protocol uses a simple message-based format:
//!
//! 1. **SyncStep1**: Client sends its state vector to the server
//! 2. **SyncStep2**: Server responds with missing updates
//! 3. **Update**: Incremental updates are sent as changes occur
//! 4. **Awareness**: User presence information (cursor, selection, etc.)
//!
//! ## Message Format
//!
//! Messages are encoded using lib0 variable-length integers:
//!
//! ```text
//! Message = varUint(message_type) • message_payload
//!
//! message_type:
//!   0 = Sync protocol message
//!   1 = Awareness message
//!   2 = Auth message
//!   3 = AwarenessQuery
//! ```

pub mod awareness;
pub mod message;
pub mod sync;

pub use awareness::{AwarenessState, ClientAwareness};
pub use message::{Message, SyncMessage};
pub use sync::{SyncProtocol, SyncState};
