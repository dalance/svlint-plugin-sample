use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use svlint::config::ConfigOption;
use svlint::linter::{Rule, RuleResult};

#[derive(Default)]
pub struct SamplePlugin;

impl Rule for SamplePlugin {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _config: &ConfigOption,
    ) -> RuleResult {
        match event {
            NodeEvent::Enter(RefNode::InitialConstruct(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
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
