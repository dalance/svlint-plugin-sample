use sv_parser::{NodeEvent, RefNode, SyntaxTree};
use svlint::config::ConfigOption;
use svlint::linter::{SyntaxRule, SyntaxRuleResult};

#[derive(Default)]
pub struct AnotherPlugin;

impl SyntaxRule for AnotherPlugin {
    fn check(
        &mut self,
        _syntax_tree: &SyntaxTree,
        event: &NodeEvent,
        _config: &ConfigOption,
    ) -> SyntaxRuleResult {
        match event {
            NodeEvent::Enter(RefNode::DisableStatementFork(_)) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
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
