use std::{collections::HashMap}

#[derive(Debug)]
/// Represents the store of configurations
pub struct ConfigStore {
    /// Path to config
    location: PathBuf,

    /// Available configurations
    configurations: HashMap<String, Configuration>,

    /// Name of the active configuration
    active: String,
}

fn get_default_config_path() -> String {
    return "~/.config/binswap";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_default_config_path() {
        let expected_default_config_path = "~/.config/binswap";
        let actual_default_config_path = get_default_config_path();

        assert_eq!(expected_default_config_path, actual_default_config_path);
    }
}
