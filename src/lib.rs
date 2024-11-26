#![deny(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![deny(warnings)]
//! # valkyrie_pilot's shutdown signal
//! This crate provides a simple, cross-platform Tokio shutdown waiter.
//!
//! Made because docker sends sigquit, but [tokio's `ctrl_c`](tokio::signal::ctrl_c)
//! does not respond to sigquit, leading to the container being killed 10s later.

/// Completes on sigint, sigquit, or sigterm on Unix or delegates to
/// [tokio's `ctrl_c`](tokio::signal::ctrl_c) on other platforms
///
/// # Panics
/// This function may panic if:
/// - you do not have the `rt` tokio feature flag pulled in somehow
/// - the low-level C signal handler functions fail
pub async fn shutdown_signal() {
    #[cfg(target_family = "unix")]
    {
        use std::pin::pin;
        use tokio::signal::unix::{signal, SignalKind};
        let mut interrupt = signal(SignalKind::interrupt()).expect("Failed to listen to sigint");
        let mut quit = signal(SignalKind::quit()).expect("Failed to listen to sigquit");
        let mut terminate = signal(SignalKind::terminate()).expect("Failed to listen to sigterm");
        futures_util::future::select_all([
            pin!(interrupt.recv()),
            pin!(quit.recv()),
            pin!(terminate.recv()),
        ])
        .await;
    }
    #[cfg(not(target_family = "unix"))]
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen to ctrl+c");
}
