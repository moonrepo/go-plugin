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
        let id_prefix = format!("{suffix}-{id}");

        if version.contains(id) && !version.contains(&id_prefix) {
            return version.replace(id, &id_prefix);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_from() {
        assert_eq!(from_go_version("1"), "1.0.0");
        assert_eq!(from_go_version("1.2"), "1.2.0");
        assert_eq!(from_go_version("1.2.3"), "1.2.3");

        assert_eq!(from_go_version("1alpha1"), "1.0.0-alpha1");
        assert_eq!(from_go_version("1.2beta2"), "1.2.0-beta2");
        assert_eq!(from_go_version("1.2.3rc3"), "1.2.3-rc3");

        // Shouldn't change
        assert_eq!(from_go_version("1.0.0"), "1.0.0");
        assert_eq!(from_go_version("1.0.0-alpha1"), "1.0.0-alpha1");
    }

    #[test]
    fn formats_to() {
        assert_eq!(to_go_version("1.0.0"), "1");
        assert_eq!(to_go_version("1.2.0"), "1.2");
        assert_eq!(to_go_version("1.2.3"), "1.2.3");

        assert_eq!(to_go_version("1.0.0-alpha1"), "1alpha1");
        assert_eq!(to_go_version("1.2.0-beta2"), "1.2beta2");
        assert_eq!(to_go_version("1.2.3-rc3"), "1.2.3rc3");
    }
}
