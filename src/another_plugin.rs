use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use svlint::config::ConfigOption;
use svlint::linter::{Rule, RuleResult};

#[derive(Default)]
pub struct AnotherPlugin;

impl Rule for AnotherPlugin {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _config: &ConfigOption,
    ) -> RuleResult {
        match event {
            NodeEvent::Enter(RefNode::DisableStatementFork(_)) => RuleResult::Fail,
            _ => RuleResult::Pass,
        }
    }

    fn name(&self) -> String {
        String::from("another_plugin")
    }

    fn hint(&self, _config: &ConfigOption) -> String {
        String::from("Do not use `disable fork`.")
    }

    fn reason(&self) -> String {
        String::from("This example dislikes disable-fork statements.")
    }
}
