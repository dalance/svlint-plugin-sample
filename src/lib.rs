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
