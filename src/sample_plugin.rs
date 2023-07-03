use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use svlint::config::ConfigOption;
use svlint::linter::{SyntaxRule, SyntaxRuleResult};

#[derive(Default)]
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
        String::from("Remove the `initial` process.")
    }

    fn reason(&self) -> String {
        String::from("This example doesn't like `initial` processes.")
    }
}
