pub fn from_go_version(version: &str) -> String {
    // Zero releases don't end in ".0",
    // so we must fix manually...
    let suffix = match version.matches(".").count() {
        1 => ".0",
        0 => ".0.0",
        _ => "",
    };

    // go1.4rc1, go1.19.1beta, etc
    for id in ["alpha", "beta", "rc"] {
        if version.contains(id) {
            return version.replace(id, format!("{suffix}-{id}").as_str());
        }
    }

    format!("{version}{suffix}")
}

pub fn to_go_version(version: &str) -> String {
    let mut next = version;

    // Remove all trailing ".0"
    while let Some(prefix) = next.strip_suffix(".0") {
        next = prefix;
    }

    // Remove leading ".0" from prereleases
    let mut next = next.to_owned();

    while next.contains(".0-") {
        next = next.replace(".0-", "-");
    }

    next.replace("-", "")
}
