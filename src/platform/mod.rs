// Platform-specific implementations
// Each platform module implements the core wipe engine functionality

#[cfg(target_os = "windows")]
pub mod windows;

#[cfg(target_os = "linux")]
pub mod linux;

#[cfg(target_os = "android")]
pub mod android;
