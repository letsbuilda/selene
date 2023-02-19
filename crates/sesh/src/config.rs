use std::collections::HashMap;

#[allow(unused_imports)]
use dirs::config_dir;

const SESH_CONFIG_FILE: &str = "config.sesh";

#[derive(Debug, Clone)]
pub struct Config {
    /// A mapping of the alias name to what the alias will execute.
    pub aliases: HashMap<String, String>,
}

#[allow(clippy::derivable_impls)]
impl Default for Config {
    fn default() -> Self {
        Self { aliases: HashMap::new() }
    }
}
