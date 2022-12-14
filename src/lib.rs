//! lib
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    unused_variables,
    unused_imports,
    // missing_docs,
    unused_import_braces,
    rustdoc::broken_intra_doc_links,
    missing_debug_implementations,
    unreachable_pub,
    clippy::all
)]

/// CLI param parsing
pub mod cli;

/// App
pub mod app;

/// Networking capabilities
pub mod network;

/// All UI components
pub mod ui;

/// Input events from the user
pub mod events;
