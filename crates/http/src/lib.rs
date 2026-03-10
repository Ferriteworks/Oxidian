/// The Discord REST HTTP client with built-in rate-limit handling.
pub mod client;
/// Per-bucket and global rate-limit state tracking.
pub mod ratelimit;
/// Typed Discord REST API route definitions.
pub mod routes;

pub use client::HttpClient;
