use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

// Currently times out on Windows!
#[cfg(not(windows))]
generate_globals_test!("go-test", "golang.org/x/tools/cmd/stringer@latest", "GOBIN");
