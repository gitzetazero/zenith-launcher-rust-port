//! Paths, errors, shared state and tasks (XMCL: `Service`, `SharedState`, `task`, `app/`). Stage 1.

/// Name shown in window titles and the About page.
pub const APP_NAME: &str = "Zenith Launcher";

#[cfg(test)]
mod tests {
    #[test]
    fn app_name_is_zenith_launcher() {
        assert_eq!(super::APP_NAME, "Zenith Launcher");
    }
}
