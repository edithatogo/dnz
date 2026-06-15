//! CLI integration tests.

use clap::CommandFactory;

#[test]
fn test_cli_metadata_clap() {
    // This parses clap metadata declarations to ensure no option conflicts.
    // Clap debug_assert fails at test run time if clap attributes are misconfigured.
    let _cmd = dnz_cli::Cli::command().debug_assert();
}

#[test]
fn test_cli_ping() {
    assert_eq!(dnz_core::greeting(), "Hello from dnz-core!");
}
