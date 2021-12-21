//! Ergo node REST API

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
#![deny(dead_code)]
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// #![deny(clippy::todo)] // TODO: remove
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]

mod node_client;
mod peer_info;

pub use node_client::NodeClient;
pub use node_client::NodeError;
pub use peer_info::PeerInfo;
