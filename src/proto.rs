use crate::version::{from_go_version, to_go_version};
use extism_pdk::*;
use proto_pdk::*;
use std::collections::HashMap;
use std::fs;

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
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_proto_environment()?;

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
        return err!(PluginError::UnsupportedCanary { tool: NAME.into() }.into());
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

    Ok(Json(DownloadPrebuiltOutput {
        archive_prefix: Some("go".into()),
        checksum_url: Some(format!("https://dl.google.com/go/{filename}.sha256")),
        download_url: format!("https://dl.google.com/go/{filename}"),
        download_name: Some(filename),
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_bins(Json(_): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    let env = get_proto_environment()?;

    Ok(Json(LocateBinsOutput {
        bin_path: Some(format_bin_name(format!("bin/{}", BIN), env.os).into()),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec![
            "$GOBIN".into(),
            "$GOROOT/bin".into(),
            "$GOPATH/bin".into(),
            "$HOME/go/bin".into(),
        ],
        ..LocateBinsOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let tags = load_git_tags("https://github.com/golang/go")?;

    let tags = tags
        .iter()
        .filter_map(|t| t.strip_prefix("go"))
        .map(from_go_version)
        .collect::<Vec<_>>();

    Ok(Json(LoadVersionsOutput::from(tags)?))
}

#[plugin_fn]
pub fn detect_version_files(_: ()) -> FnResult<Json<DetectVersionOutput>> {
    Ok(Json(DetectVersionOutput {
        files: vec!["go.mod".into(), "go.work".into()],
    }))
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
pub fn install_global(
    Json(input): Json<InstallGlobalInput>,
) -> FnResult<Json<InstallGlobalOutput>> {
    let result = exec_command!(inherit, BIN, ["install", &input.dependency]);

    Ok(Json(InstallGlobalOutput::from_exec_command(result)))
}

#[plugin_fn]
pub fn uninstall_global(
    Json(input): Json<UninstallGlobalInput>,
) -> FnResult<Json<UninstallGlobalOutput>> {
    let mut output = UninstallGlobalOutput {
        uninstalled: true,
        ..UninstallGlobalOutput::default()
    };
    let global_path = input.globals_dir.join(input.dependency);

    if global_path.exists() {
        if let Err(error) = fs::remove_file(global_path) {
            output.uninstalled = false;
            output.error = Some(error.to_string());
        }
    }

    Ok(Json(output))
}

#[plugin_fn]
pub fn sync_shell_profile(
    Json(input): Json<SyncShellProfileInput>,
) -> FnResult<Json<SyncShellProfileOutput>> {
    Ok(Json(SyncShellProfileOutput {
        check_var: "GOBIN".into(),
        export_vars: Some(HashMap::from_iter([(
            "GOBIN".into(),
            "$HOME/go/bin".into(),
        )])),
        extend_path: Some(vec!["$GOBIN".into()]),
        skip_sync: input.passthrough_args.contains(&"--no-gobin".to_string()),
    }))
}
