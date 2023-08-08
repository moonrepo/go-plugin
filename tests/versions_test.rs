use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("go-test", {
    "1.19" => "1.19.11",
    "1.11" => "1.11.13",
    "1.9.0-rc2" => "1.9.0-rc2",
});

#[test]
fn loads_versions_from_git() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[test]
fn parse_gomod_file() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.parse_version_file(ParseVersionFileInput {
        content: r#"
module github.com/moonrepo/go-plugin

go 1.20

require (
    github.com/99designs/gqlgen v0.17.25
)"#
        .into(),
        file: "go.mod".into(),
        ..Default::default()
    });

    assert_eq!(output.version.unwrap(), "1.20.0");
}

#[test]
fn returns_no_version_from_gomod() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.parse_version_file(ParseVersionFileInput {
        content: r#"
module github.com/moonrepo/go-plugin

require (
    github.com/99designs/gqlgen v0.17.25
)"#
        .into(),
        file: "go.mod".into(),
        ..Default::default()
    });

    assert_eq!(output.version, None);
}

#[test]
fn parse_gowork_file() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.parse_version_file(ParseVersionFileInput {
        content: r#"
go 1.18

use (
    ./hello
    ./example
)"#
        .into(),
        file: "go.work".into(),
        ..Default::default()
    });

    assert_eq!(output.version.unwrap(), "1.18.0");
}

#[test]
fn returns_no_version_from_gowork() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("go-test", sandbox.path());

    let output = plugin.parse_version_file(ParseVersionFileInput {
        content: r#"
use (
    ./hello
    ./example
)"#
        .into(),
        file: "go.work".into(),
        ..Default::default()
    });

    assert_eq!(output.version, None);
}
