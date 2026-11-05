//! Accounts (XMCL: `user/`, `@xmcl/user`). Stage 2.
//!
//! Real Microsoft sign-in is blocked until the user registers an Azure app and Mojang approves it
//! (docs/architecture.md). The client ID is configuration, never code.

/// The Azure client ID of this build, from `ZENITH_MSA_CLIENT_ID` at run time or at build time.
/// `None` when unset or blank: the launcher then says sign-in is not configured.
pub fn msa_client_id() -> Option<String> {
    pick_client_id(
        std::env::var("ZENITH_MSA_CLIENT_ID").ok(),
        option_env!("ZENITH_MSA_CLIENT_ID"),
    )
}

fn pick_client_id(runtime: Option<String>, build: Option<&str>) -> Option<String> {
    let clean = |s: &str| Some(s.trim().to_owned()).filter(|s| !s.is_empty());
    runtime
        .as_deref()
        .and_then(clean)
        .or_else(|| build.and_then(clean))
}

#[cfg(test)]
mod tests {
    use super::pick_client_id;

    #[test]
    fn blank_is_not_configured() {
        assert_eq!(pick_client_id(None, None), None);
        assert_eq!(pick_client_id(Some("  ".into()), Some("")), None);
    }

    #[test]
    fn run_time_wins_over_build_time() {
        assert_eq!(
            pick_client_id(Some(" a ".into()), Some("b")).as_deref(),
            Some("a")
        );
        assert_eq!(
            pick_client_id(Some("".into()), Some("b")).as_deref(),
            Some("b")
        );
    }
}
