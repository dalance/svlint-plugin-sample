# svlint-plugin-sample

This is a sample project of an [svlint](https://github.com/dalance/svlint)
plugin.
An svlint plugin implements one or more rules (either `TextRule` or
`SyntaxRule`) in an externally developed project, compiling a shared object
file which is dynamically loaded at svlint runtime.


## Usage

Use svlint's `--plugin` option with the shared object produced by `cargo build`
in this repository.
The shared object can be copied from `target/(debug|release)/`, but the
filename will be platform-dependent.

- Linux: `lib<name>.so`
- MacOS: `lib<name>.dylib`
- Windows: `<name>.dll`

```
$ svlint --plugin libsvlint_plugin_sample.so test.sv
Fail: sample_plugin
   --> test.sv:2:1
  |
2 | initial begin
  | ^^^^^^^ hint  : Remove the `initial` process.
  |         reason: This example doesn't like `initial` processes.
```

The loaded plugin is automatically enabled, has access to values from svlint's
TOML configuration, and syntax rules may be controlled using
[special comments](https://github.com/dalance/svlint/blob/master/MANUAL.md#textrules-and-syntaxrules-sections).


## Implementation

As a plugin must create a shared object, the crate type of `Cargo.toml` should
be `cdylib`.
Alternatively, `dylib` could be used, but the resulting binary may be very
large.

```toml
[lib]
crate-type = ["cdylib"]
```

A plugin project must define a
[`get_plugin`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/lib.rs#L13-L21)
function (in `src/lib.rs`) which returns the list of rules that it implements.
Svlint provides a macro [`pluginrules`](https://github.com/dalance/svlint/blob/master/src/linter.rs#L15-L33)
which makes this quite simple.

```rust
#[allow(improper_ctypes_definitions)]
#[no_mangle]
pub extern "C" fn get_plugin() -> Vec<Rule> {
    pluginrules!(
        SamplePlugin,
        AnotherPlugin,
        ForbiddenRegex
    )
}
```

Rules are defined by the `TextRule` or `SyntaxRule` traits, see
[`src/forbidden_regex.rs`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/forbidden_regex.rs)
and
[`src/another_plugin.rs`](https://github.com/dalance/svlint-plugin-sample/blob/master/src/sample_plugin.rs)
for examples of each.

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

`TextRule` must implement `check`, `name`, `hint` and `reason`.
`SyntaxRule` must implement `check`, `name`, `hint` and `reason`.


## Testing

This sample project includes a basic test infrastructure to test its rules.
To run the tests, first build the shared object (`cargo build`), then run
`cargo test`.
If you wish to debug via `println`, run `cargo test -- --show-output`.

The test infrastructure has 3 main parts:

1. The `tests` module in `src/lib.rs`.
  - `so_path()`: Return a string with the expected filesystem path of the
    shared object.
    If your plugin has an unusual name (specified in `Cargo.toml`), then this
    may require modification.
  - `execute_linter()`: Attempts to perform in the same way as svlint does.
    If svlint is modified, then this may require modification.
  - `plugin_test()`: Called by the functions written by `build.rs`.
    Should not normally require modification.
2. A collection of SystemVerilog testcase files in `testcases/(fail|pass)/`.
  Naturally, you must create your own testcases for your own plugin rules.
  To add a SystemVerilog test file, simply copy it to `testcases/pass/` if it
  must pass *all* of the plugin's rules, or to `testcases/fail/` if it
  must fail *any* of the plugin's rules.
3. The build script (`build.rs`) which uses the testcase files to produce
  test functions just before the main compilation.
