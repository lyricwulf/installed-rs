#[cfg(target_os = "macos")]
#[path = "macos.rs"]
mod platform_app;

#[cfg(target_os = "windows")]
#[path = "windows.rs"]
mod platform_app;

pub use platform_app::App;
