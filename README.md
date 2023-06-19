# svlint-plugin-sample

This is a sample project of [svlint](https://github.com/dalance/svlint) plugins.


## Overview

An svlint plugin is a shared library, so the crate type of `Cargo.toml` must
be `cdylib`.
Alternatively, `dylib` could be used instead, but it causes the resulting
binary to be very large.

```toml
[lib]
crate-type = ["cdylib"]
```

A plugin project must define a
[`get_plugin`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/lib.rs#L12)
function which returns the list of rules that it implements.

```rust
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_plugin() -> Vec<*mut dyn SyntaxRule> {
    combine_rules!(
        SamplePlugin,
        AnotherPlugin,
    )
}
```

Rules are defined by the `SyntaxRule` trait, see both
[`src/sample_plugin.rs`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/sample_plugin.rs)
and
[`src/another_plugin.rs`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/another_plugin.rs).

```rust
impl SyntaxRule for SamplePlugin {
    fn check(
        &mut self,
        _syntax_tree: &Tree,
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
```

`SyntaxRule` must implement `check`, `name`, `hint` and `reason`.


## Usage

Use svlint's `--plugin` option with the shared object produced by `cargo build`
in this repository (copy from
`target/(debug|release)/libsvlint_plugin_sample.so`).

```
$ svlint --plugin libsvlint_plugin_sample.so test.sv
Fail: sample_plugin
   --> test.sv:2:1
  |
2 | initial begin
  | ^^^^^^^ hint  : `initial` is forbidden
  |         reason: this is a sample plugin
```

The loaded plugin is automatically enabled, may be controlled using [special
comments](https://github.com/dalance/svlint/blob/master/MANUAL.md#textrules-and-syntaxrules-sections),
and has access to values from configuration options.
