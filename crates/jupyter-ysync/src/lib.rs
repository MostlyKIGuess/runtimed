//! # jupyter-ysync
//!
//! Y-sync protocol implementation for Jupyter notebook collaboration.
//!
//! This crate provides CRDT-based document synchronization for Jupyter notebooks
//! using the [yrs](https://docs.rs/yrs) library (Rust port of Y.js). The protocol
//! is compatible with [jupyter-server-documents](https://github.com/jupyter-ai-contrib/jupyter-server-documents)
//! and enables multi-client collaborative editing.
//!
//! ## Features
//!
//! - **NotebookDoc**: CRDT document representation of Jupyter notebooks
//! - **Conversion**: Bidirectional conversion between `nbformat::v4::Notebook` and Y.Doc
//! - **Character-level editing**: Cell sources use Y.Text for fine-grained collaboration
//!
//! ## Example
//!
//! ```rust
//! use jupyter_ysync::{NotebookDoc, notebook_to_ydoc, ydoc_to_notebook};
//! use nbformat::v4::Notebook;
//!
//! // Convert an existing notebook to a collaborative document
//! // let doc = notebook_to_ydoc(&notebook)?;
//!
//! // Make edits via the Y.Doc API
//! // ...
//!
//! // Convert back to nbformat for saving
//! // let updated_notebook = ydoc_to_notebook(&doc)?;
//! ```

pub mod convert;
pub mod doc;
pub mod error;

#[cfg(feature = "python")]
pub mod python;

pub use convert::{notebook_to_ydoc, ydoc_to_notebook};
pub use doc::{cell_types, keys, NotebookDoc};
pub use error::{Result, YSyncError};

// Re-export for Python bindings
#[cfg(feature = "python")]
pub use python::jupyter_ysync;
