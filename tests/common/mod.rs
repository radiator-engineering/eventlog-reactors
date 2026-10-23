//! Helpers shared by the reactor integration tests.

use std::path::PathBuf;

/// The `eventlog` binary the reactors run beside. It is the first `eventlog`
/// next to the test binary, or else the first one on PATH. Install the
/// `eventlog-cli` crate before you run the tests.
#[allow(dead_code)]
pub fn core_bin() -> PathBuf {
    let name = format!("eventlog{}", std::env::consts::EXE_SUFFIX);
    let reactors = PathBuf::from(env!("CARGO_BIN_EXE_eventlog-reactors"));
    let beside = reactors.with_file_name(&name);
    if beside.is_file() {
        return beside;
    }
    let on_path = std::env::var_os("PATH").and_then(|paths| {
        std::env::split_paths(&paths)
            .map(|dir| dir.join(&name))
            .find(|candidate| candidate.is_file())
    });
    on_path.unwrap_or_else(|| {
        panic!("`eventlog` is not on PATH; run `cargo install eventlog-cli --locked` first")
    })
}

/// The `eventlog-reactors` binary under test.
#[allow(dead_code)]
pub fn reactors_bin() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_eventlog-reactors"))
}
