use crate::version::{from_go_version, to_go_version};
use extism_pdk::*;
use proto_pdk::*;

static NAME: &str = "Go";
static BIN: &str = "go";

#[plugin_fn]
pub fn register_tool(Json(_): Json<ToolMetadataInput>) -> FnResult<Json<ToolMetadataOutput>> {
    Ok(Json(ToolMetadataOutput {
        name: NAME.into(),
        type_of: PluginType::Language,
        ..ToolMetadataOutput::default()
    }))
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let version = to_go_version(&input.env.version);

    let arch = match input.env.arch {
        HostArch::Arm => "armv6l",
        HostArch::Arm64 => "arm64",
        HostArch::X64 => "amd64",
        HostArch::X86 => "386", // i386
        HostArch::S390x => "s390x",
        other => {
            return Err(PluginError::UnsupportedArch {
                tool: NAME.into(),
                arch: other.to_string(),
            })?;
        }
    };

    let prefix = match input.env.os {
        HostOS::Linux => format!("go{version}.linux-{arch}"),
        HostOS::FreeBSD => format!("go{version}.freebsd-{arch}"),
        HostOS::MacOS => format!("go{version}.darwin-{arch}"),
        HostOS::Windows => format!("go{version}.windows-{arch}"),
        other => {
            return Err(PluginError::UnsupportedOS {
                tool: NAME.into(),
                os: other.to_string(),
            })?;
        }
    };

    let filename = if input.env.os == HostOS::Windows {
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
pub fn locate_bins(Json(input): Json<LocateBinsInput>) -> FnResult<Json<LocateBinsOutput>> {
    Ok(Json(LocateBinsOutput {
        bin_path: Some(if input.env.os == HostOS::Windows {
            format!("bin/{}.exe", BIN)
        } else {
            format!("bin/{}", BIN)
        }),
        fallback_last_globals_dir: true,
        globals_lookup_dirs: vec![
            "$GOBIN".into(),
            "$GOROOT/bin".into(),
            "$GOPATH/bin".into(),
            "$HOME/go/bin".into(),
        ],
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

    Ok(Json(LoadVersionsOutput::from_tags(&tags)?))
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
                version = Some(from_go_version(v));
                break;
            }
        }
    }

    Ok(Json(ParseVersionFileOutput { version }))
}
