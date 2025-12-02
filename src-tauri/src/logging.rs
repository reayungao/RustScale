use tracing::info;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};

pub fn init_logging(_app_handle: &tauri::AppHandle) {
    // 1. Set up console layer (for dev)
    let console_layer = fmt::layer()
        .with_target(false)
        .with_thread_ids(true)
        .with_level(true);

    // 2. Initialize registry
    tracing_subscriber::registry()
        .with(EnvFilter::new(
            "info,tauri_apprustscale_lib=debug,ort=error",
        )) // Silence ORT logs
        .with(console_layer)
        .init();

    // 4. Panic Hook
    std::panic::set_hook(Box::new(|panic_info| {
        let payload = panic_info.payload();
        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
            *s
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s
        } else {
            "Unknown panic"
        };

        let location = panic_info
            .location()
            .map(|l| format!("{}:{}", l.file(), l.line()))
            .unwrap_or_default();

        tracing::error!(target: "panic", "APPLICATION PANIC: {} at {}", msg, location);
        eprintln!("APPLICATION PANIC: {} at {}", msg, location); // Ensure it hits stderr too
    }));

    info!("Logging initialized (Console Only)");
}
