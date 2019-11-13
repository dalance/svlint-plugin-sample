use sv_parser::{RefNode, SyntaxTree};
use svlint::linter::{Rule, RuleResult};

#[no_mangle]
pub extern "C" fn get_plugin() -> *mut dyn Rule {
    let boxed = Box::new(SamplePlugin {});
    Box::into_raw(boxed)
}

pub struct SamplePlugin;

impl Rule for SamplePlugin {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> RuleResult {
        match node {
            RefNode::InitialConstruct(_) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("sample_plugin")
    }

    fn hint(&self) -> String {
        String::from("`initial` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this is a sample plugin")
    }
}
