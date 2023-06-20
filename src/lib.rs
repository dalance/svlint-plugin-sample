mod sample_plugin;
mod another_plugin;

use svlint::pluginrule;
use svlint::linter::Rule;
use crate::{
    sample_plugin::SamplePlugin,
    another_plugin::AnotherPlugin,
};

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_plugin() -> Vec<Rule> {
    let mut ret: Vec<Rule> = Vec::new();

    ret.push(pluginrule!(Syntax, SamplePlugin));
    ret.push(pluginrule!(Syntax, AnotherPlugin));

    ret
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::env;
    use std::fs::read_to_string;
    use std::path::{Path, PathBuf};
    use svlint::config::Config;
    use svlint::linter::Linter;
    use sv_parser::parse_sv_str;

    fn so_path() -> String {
        let cargo_manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let cargo_pkg_name = env::var("CARGO_PKG_NAME").unwrap();
        let cargo_pkg_name = cargo_pkg_name.replace("-", "_");

        let path = if cfg!(target_os = "windows") {
            Path::new(cargo_manifest_dir.as_str())
                .join("target")
                .join("debug")
                .join(format!("lib{}.dll", cargo_pkg_name))
        } else if cfg!(target_os = "macos") {
            Path::new(cargo_manifest_dir.as_str())
                .join("target")
                .join("debug")
                .join(format!("lib{}.dylib", cargo_pkg_name))
        } else {
            Path::new(cargo_manifest_dir.as_str())
                .join("target")
                .join("debug")
                .join(format!("lib{}.so", cargo_pkg_name))
        };

        String::from(path.to_str().unwrap())
    }

    // Run the linter with plugin loaded, similar to how svlint works.
    // No messages are printed, only pass/fail returned.
    fn execute_linter(sv: &Path) -> bool {
        // Create a linter object, with blank config, plugin loaded.
        // Run all plugin's rules on a single file.

        let config = Config::new();
        let mut linter = Linter::new(config);
        let plugin_path = so_path();
        linter.load(&Path::new(&plugin_path));

        let mut pass = true;

        // Iterate over lines in the file, applying each textrule to each
        // line in turn.
        let text: String = read_to_string(&sv).unwrap();
        let mut beg: usize = 0;
        for line in text.lines() {
            for _failed in linter.textrules_check(&line, &sv, &beg) {
                pass = false;
            }

            // Newlines are not included in each line and `text` does not
            // contain CRLF because `read_to_string` convents CRLF to LF.
            beg += line.len() + 1; // Track the beginning byte index.
        }

        let defines = HashMap::new();
        let includes: Vec<PathBuf> = Vec::new();
        match parse_sv_str(text.as_str(), &sv, &defines, &includes, false, false) {
            Ok((syntax_tree, _)) => {
                // Iterate over nodes in the concrete syntax tree, applying
                // each syntaxrule to each node in turn.

                for node in syntax_tree.into_iter().event() {
                    for _failed in linter.syntaxrules_check(&syntax_tree, &node) {
                        pass = false;
                    }
                }
            }
            Err(_) => {
                pass = false;
            }
        }

        pass
    }

    fn plugin_test(filename: &str, pass_not_fail: bool) {
        let sv: &Path = Path::new(filename);
        assert_eq!(execute_linter(sv), pass_not_fail);
    }

    include!(concat!(env!("OUT_DIR"), "/test.rs"));
}
