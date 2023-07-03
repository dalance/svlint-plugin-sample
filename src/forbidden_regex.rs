use svlint::config::ConfigOption;
use svlint::linter::{TextRule, TextRuleEvent, TextRuleResult};
use regex::Regex;

#[derive(Default)]
pub struct ForbiddenRegex {
    re: Option<Regex>,
}

impl TextRule for ForbiddenRegex {
    fn check(
        &mut self,
        event: TextRuleEvent,
        _option: &ConfigOption,
    ) -> TextRuleResult {
        let line: &str = match event {
            TextRuleEvent::StartOfFile => {
                return TextRuleResult::Pass;
            }
            TextRuleEvent::Line(x) => x,
        };

        if self.re.is_none() {
            let r = format!(r"XXX");
            self.re = Some(Regex::new(&r).unwrap());
        }
        let re = self.re.as_ref().unwrap();

        let is_match: bool = re.is_match(line);
        if is_match {
            TextRuleResult::Fail {
                offset: 0,
                len: line.len(),
            }
        } else {
            TextRuleResult::Pass
        }
    }

    fn name(&self) -> String {
        String::from("forbidden_regex")
    }

    fn hint(&self, _option: &ConfigOption) -> String {
        String::from("Remove the string 'XXX' from all lines.")
    }

    fn reason(&self) -> String {
        String::from("XXX is not meaningful enough.")
    }
}
