use regex::Regex;

use super::{Context, Module, RootModuleConfig};

use crate::configs::r::RConfig;
use crate::formatter::StringFormatter;
use crate::utils;

const R_VERSION_PATTERN: &str = r" (?P<rversion>\d+\.\d+\.\d+) ";

/// Creates a module with the current Node.js version
///
/// Will display the Node.js version if any of the following criteria are met:
///     - Current directory contains a `.js` file
///     - Current directory contains a `package.json` or `.node-version` file
///     - Current directory contains a `node_modules` directory
pub fn module<'a>(context: &'a Context) -> Option<Module<'a>> {
    let is_r_project = context.try_begin_scan()?.set_extensions(&["R"]).is_match();
    if !is_r_project {
        log::debug!("r: Not a R project; getting out!");
        return None;
    }

    log::debug!("r: This is a R project; getting in...");

    let r_version = utils::exec_cmd("r", &["--version"])?.stderr;
    log::debug!("r: r_version={}", r_version);

    let formatted_version = parse_version(&r_version)?;
    log::debug!("r: formatted_version={}", formatted_version);

    let mut module = context.new_module("r");
    let config: RConfig = RConfig::try_load(module.config);
    let formatter = if let Ok(formatter) = StringFormatter::new(config.format) {
        formatter.map(|variable| match variable {
            "version" => Some(formatted_version.clone()),
            _ => {
                log::debug!("r: No version for R has been detected.");
                None
            }
        })
    } else {
        log::warn!("Error parsing format string in `r.format`");
        return None;
    };
    module.set_segments(formatter.parse(None));
    module.get_prefix().set_value("");
    module.get_suffix().set_value("");

    Some(module)
}

fn parse_version(version: &str) -> Option<String> {
    let version_regex = Regex::new(R_VERSION_PATTERN).ok()?;
    let captures = version_regex.captures(version)?;
    let r_version = captures["rversion"].to_owned();
    let r_formatted = format!("{}{}", "v", r_version);
    log::debug!("r: r_version = {}", r_version);
    Some(r_formatted.trim().to_owned())
}

#[cfg(test)]
mod tests {
    // use crate::modules::utils::test::render_module;
    // use ansi_term::Color;
    // use std::fs::{self, File};
    // use std::io;
    // use tempfile;
    use super::*;

    #[test]
    fn test_parse_r_version() {
        let r_v3 = r#"r_version=R version 3.6.3 (2020-02-29) -- "Holding the Windsock"
        Copyright (C) 2020 The R Foundation for Statistical Computing
        Platform: x86_64-w64-mingw32/x64 (64-bit)
        
        R is free software and comes with ABSOLUTELY NO WARRANTY.
        You are welcome to redistribute it under the terms of the
        GNU General Public License versions 2 or 3.
        For more information about these matters see
        https://www.gnu.org/licenses/."#;
        assert_eq!(parse_version(r_v3), Some(String::from("v3.6.3")));
    }

    #[test]
    fn test_parse_r_invalid_semantic_version() {
        let r_invalid = r#"r_version=R version 3.6.5.2 (2020-02-29) -- "Holding the Windsock"
        Copyright (C) 2020 The R Foundation for Statistical Computing
        Platform: x86_64-w64-mingw32/x64 (64-bit)
        
        R is free software and comes with ABSOLUTELY NO WARRANTY.
        You are welcome to redistribute it under the terms of the
        GNU General Public License versions 2 or 3.
        For more information about these matters see
        https://www.gnu.org/licenses/."#;
        assert_eq!(parse_version(r_invalid), None);
    }
}
