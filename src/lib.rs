mod sample_plugin;
mod another_plugin;

use svlint::linter::SyntaxRule;
use crate::{
    sample_plugin::SamplePlugin,
    another_plugin::AnotherPlugin,
};

#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_plugin() -> Vec<*mut dyn SyntaxRule> {
    combine_rules!(
        SamplePlugin,
        AnotherPlugin,
    )
}

#[macro_export]
macro_rules! combine_rules {
    ( $( $x:ty ),* $(,)? ) => {
        {
            let mut vec: Vec<*mut dyn SyntaxRule> = Vec::new();
            $(
                let boxed = Box::<$x>::default();
                vec.push(Box::into_raw(boxed));
            )*
            vec
        }
    };
}
