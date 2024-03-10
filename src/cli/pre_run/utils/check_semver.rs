// Check if the provided version is higher than the required version

pub fn check_semver(version: &str, required_version: &str) -> bool {
    let version = semver::Version::parse(version);
    let required_version = semver::Version::parse(required_version);
    match version {
        Ok(version) => match required_version {
            Ok(required_version) => version >= required_version,
            Err(_) => false,
        },
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_semver() {
        // Test case: version is higher than required version
        assert_eq!(check_semver("1.2.3", "1.0.0"), true);

        // Test case: version is equal to required version
        assert_eq!(check_semver("1.2.3", "1.2.3"), true);

        // Test case: version is lower than required version
        assert_eq!(check_semver("1.0.0", "1.2.3"), false);

        // Test case: invalid version format
        assert_eq!(check_semver("1.2.3.4", "1.0.0"), false);
    }
}
