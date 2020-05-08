use crate::config::{ModuleConfig, RootModuleConfig};

use starship_module_config_derive::ModuleConfig;

#[derive(Clone, ModuleConfig)]
pub struct RConfig<'a> {
    pub format: &'a str,
    pub disabled: bool,
}

impl<'a> RootModuleConfig<'a> for RConfig<'a> {
    fn new() -> Self {
        RConfig {
            format: "via [R $version](blue bold) ",
            disabled: false,
        }
    }
}