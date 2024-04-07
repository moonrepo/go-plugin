use crate::config::GoPluginConfig;
use crate::version::{from_go_version, to_go_version};
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Go";
static BIN: &str = "go";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        plugin_version: Some(env!("CARGO_PKG_VERSION").into()),
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["go.mod".into(), "go.work".into()],
        ignore: vec![],
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/golang/go")?;

    let tags = tags
        .iter()
        .filter_map(|tag| tag.strip_prefix("go"))
        .map(from_go_version)
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn parse_version_file(
    Json(input): Json<ParseVersionFileInput>,
) -> FnResult<Json<ParseVersionFileOutput>> {
    let mut version = None;

    if input.file == "go.mod" || input.file == "go.work" {
        for line in input.content.split('\n') {
            if let Some(v) = line.strip_prefix("go ") {
                version = Some(UnresolvedVersionSpec::parse(v)?);
                break;
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    check_supported_os_and_arch(
        NAME,
        &env,
        permutations! [
            HostOS::Linux => [
                HostArch::X64, HostArch::Arm64, HostArch::X86, HostArch::Arm, HostArch::S390x
            ],
            HostOS::MacOS => [HostArch::X64, HostArch::Arm64],
            HostOS::Windows => [HostArch::X64, HostArch::Arm64, HostArch::X86],
            HostOS::FreeBSD => [HostArch::X64, HostArch::X86],
        ],
    )?;

    let version = &input.context.version;

    if version.is_canary() {
        return Err(plugin_err!(PluginError::UnsupportedCanary {
            tool: NAME.into()
        }));
    }

    let arch = match env.arch {
        HostArch::Arm => "armv6l",
        HostArch::Arm64 => "arm64",
        HostArch::X64 => "amd64",
        HostArch::X86 => "386", // i386
        HostArch::S390x => "s390x",
        _ => unreachable!(),
    };

    let version = to_go_version(version);
    let prefix = match env.os {
        HostOS::Linux => format!("go{version}.linux-{arch}"),
        HostOS::FreeBSD => format!("go{version}.freebsd-{arch}"),
        HostOS::MacOS => format!("go{version}.darwin-{arch}"),
        HostOS::Windows => format!("go{version}.windows-{arch}"),
        _ => unreachable!(),
    };

    let filename = if env.os == HostOS::Windows {
        format!("{prefix}.zip")
    } else {
        format!("{prefix}.tar.gz")
    };

    let host = get_tool_config::<GoPluginConfig>()?.dist_url;

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some("go".into()),
        checksum_url: Some(
            host.replace("{version}", &version)
                .replace("{file}", &format!("{filename}.sha256")),
        ),
        download_url: host
            .replace("{version}", &version)
            .replace("{file}", &filename),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        globals_lookup_dirs: vec![
            "$GOBIN".into(),
            "$GOROOT/bin".into(),
            "$GOPATH/bin".into(),
            "$HOME/go/bin".into(),
        ],
        primary: Some(ExecutableConfig::new(
            env.os.get_exe_name(format!("bin/{}", BIN)),
        )),
        ..LocateExecutablesOutput::default()
    }))
}

#[plugin_fn]
pub fn sync_shell_profile(
    Json(input): Json<SyncShellProfileInput>,
) -> FnResult<Json<SyncShellProfileOutput>> {
    let config = get_tool_config::<GoPluginConfig>()?;

    Ok(Json(SyncShellProfileOutput {
        check_var: "GOBIN".into(),
        export_vars: Some(HashMap::from_iter([(
            "GOBIN".into(),
            "$HOME/go/bin".into(),
        )])),
        extend_path: Some(vec!["$GOBIN".into()]),
        skip_sync: !config.gobin
            || input
                .passthrough_args
                .iter()
                .any(|arg| arg.as_str() == "--no-gobin"),
    }))
}
