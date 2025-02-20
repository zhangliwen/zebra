//! Peer handling.

/// Handles outbound requests from our node to the network.
mod client;
/// Wrapper around handshake logic that also opens a TCP connection.
mod connector;
/// Peer-related errors.
mod error;
/// Performs peer handshakes.
mod handshake;
/// Handles inbound requests from the network to our node.
mod server;

use client::ClientRequest;
use error::ErrorSlot;

pub use client::Client;
pub use connector::Connector;
pub use error::{HandshakeError, PeerError, SharedPeerError};
pub use handshake::Handshake;
pub use server::Server;
