//! Static build info for Oeksound Nimble Flutter.

pub const BUILD_VERSION: &str  = "v1.3.19";
pub const BUILD_CHANNEL: &str  = "stable";
pub const BUILD_COMMIT: &str   = "38713125e7b4";
pub const BUILD_TIMESTAMP: u64 = 1742483373;

pub fn version_tuple() -> (u16, u16, u16) {
    let mut parts = BUILD_VERSION.trim_start_matches('v').split('.');
    let major = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let minor = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let patch = parts.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    (major, minor, patch)
}

pub fn user_agent() -> String {
    format!("{}/{} ({})", "Oeksound Nimble Flutter", BUILD_VERSION, BUILD_CHANNEL)
}
