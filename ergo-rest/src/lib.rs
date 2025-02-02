//! Ergo node REST API

// Coding conventions
#![forbid(unsafe_code)]
#![deny(non_upper_case_globals)]
#![deny(non_camel_case_types)]
#![deny(non_snake_case)]
#![deny(unused_mut)]
// #![deny(dead_code)] // TODO: uncomment
#![allow(dead_code)] // TODO: remove
#![deny(unused_imports)]
#![deny(missing_docs)]
#![deny(rustdoc::broken_intra_doc_links)]
#![deny(clippy::wildcard_enum_match_arm)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
// #![deny(clippy::todo)] // TODO: remove
#![deny(clippy::unimplemented)]
#![deny(clippy::panic)]

mod bulk_req;
mod error;
mod known_nodes;
mod node_conf;
mod node_info;
mod node_response;
mod peer_info;

pub mod api;

pub use error::NodeError;
pub use known_nodes::KnownNodes;
pub use node_conf::NodeConf;
pub use node_info::NodeInfo;
pub use node_response::NodeResponse;
pub use peer_info::PeerInfo;
