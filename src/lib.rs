use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use svlint::config::ConfigOption;
use svlint::linter::{SyntaxRule, SyntaxRuleResult};

#[no_mangle]
pub extern "C" fn get_plugin() -> *mut dyn SyntaxRule {
    let boxed = Box::new(SamplePlugin {});
    Box::into_raw(boxed)
}

pub struct SamplePlugin;

impl SyntaxRule for SamplePlugin {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _config: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(RefNode::InitialConstruct(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("sample_plugin")
    }

    fn hint(&self, _config: &ConfigOption) -> String {
        String::from("`initial` is forbidden")
    }

    fn reason(&self) -> String {
        String::from("this is a sample plugin")
    }
}
