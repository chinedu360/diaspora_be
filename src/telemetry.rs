// Core tracing traits and functions for managing global subscriber state
use tracing::{subscriber::set_global_default, Subscriber};
// Bunyan formatter provides structured JSON logging compatible with Bunyan log viewers
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
// Tracing subscriber utilities for filtering, composing layers, and writing output
use tracing_subscriber::{fmt::MakeWriter, layer::SubscriberExt, EnvFilter, Registry};

/// Compose multiple layers into a `tracing`'s subscriber.
///
/// This function creates a layered logging setup with:
/// - Environment-based log level filtering (RUST_LOG env var)
/// - JSON storage layer for structured log data
/// - Bunyan formatting for JSON output
///
/// # Arguments
///
/// * `name` - Application name that will appear in log entries
/// * `env_filter` - Default log level filter (e.g., "info", "debug")
/// * `sink` - Where to write the logs (e.g., stdout, stderr, file)
///
/// # Implementation Notes
///
/// We are using `impl Subscriber` as return type to avoid having to
/// spell out the actual type of the returned subscriber, which is
/// indeed quite complex (a layered composition of multiple types).
/// We need to explicitly call out that the returned subscriber is
/// `Send` and `Sync` to make it possible to pass it to `init_subscriber`
/// later on, allowing it to work across thread boundaries.
pub fn get_subscriber<Sink>(
    name: String,
    env_filter: String,
    sink: Sink,
) -> impl Subscriber + Send + Sync
where
    // Sink must implement MakeWriter to create writers for log output
    // The for<'a> syntax means it works with any lifetime 'a
    Sink: for<'a> MakeWriter<'a> + Send + Sync + 'static,
{
    // Environment Filter Configuration
    // Try to read log level from RUST_LOG environment variable first,
    // fall back to the provided env_filter if RUST_LOG is not set
    let env_filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));

    // Bunyan Formatting Layer
    // Creates structured JSON logs in Bunyan format, which can be viewed
    // with tools like `bunyan` CLI or other JSON log viewers
    let formatting_layer = BunyanFormattingLayer::new(
        name, // Application name for log identification
        sink, // Output destination (stdout, file, etc.)
    );

    // Layer Composition
    // Start with Registry (the base subscriber) and compose layers:
    // 1. Registry: Base subscriber that coordinates between layers
    // 2. EnvFilter: Filters logs based on environment/level configuration
    // 3. JsonStorageLayer: Stores span data in JSON format for structured logging
    // 4. BunyanFormattingLayer: Formats and outputs logs in Bunyan JSON format
    //
    // The `with` method is provided by `SubscriberExt`, an extension
    // trait for `Subscriber` exposed by `tracing_subscriber`
    Registry::default()
        .with(env_filter) // Apply log level filtering
        .with(JsonStorageLayer) // Enable structured JSON log storage
        .with(formatting_layer) // Apply Bunyan JSON formatting and output
}

/// Initialize the global tracing subscriber.
///
/// This sets the provided subscriber as the global default for the entire application.
/// Once set, all tracing macros (info!, debug!, error!, etc.) will use this subscriber
/// to process and output log data.
///
/// # Arguments
///
/// * `subscriber` - The configured subscriber to use globally
///
/// # Panics
///
/// This function will panic if a global subscriber has already been set,
/// so it should only be called once during application initialization.
pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync) {
    // `set_global_default` registers the subscriber as the global handler
    // for all tracing spans and events in the application
    set_global_default(subscriber)
        .expect("Failed to set subscriber - global subscriber may already be initialized");
}
