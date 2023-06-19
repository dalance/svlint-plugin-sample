mod sample_plugin;
mod another_plugin;

use svlint::linter::Rule;
use crate::{
    sample_plugin::SamplePlugin,
    another_plugin::AnotherPlugin,
};

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_plugin() -> Vec<Rule> {
    let mut ret: Vec<Rule> = Vec::new();

    let s = Box::new(SamplePlugin {});
    ret.push(Rule::Syntax(Box::into_raw(s)));

    let s = Box::new(AnotherPlugin {});
    ret.push(Rule::Syntax(Box::into_raw(s)));

    ret
}
