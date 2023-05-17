# svlint-plugin-sample

This is a sample project of [svlint](https://github.com/dalance/svlint) plugin.

## Create plugin

svlint plugin is a shared library. So crate-type of `Cargo.toml` must be `cdylib`.
`dylib` can be used also, but it causes too large binary size.

```
[lib]
crate-type = ["cdylib"]
```

All plugin must have `get_plugin` function to generate `SyntaxRule`.

```
#[no_mangle]
pub extern "C" fn get_plugin() -> *mut dyn SyntaxRule {
    let boxed = Box::new(SamplePlugin {});
    Box::into_raw(boxed)
}
```

The lint rule is defined as `SyntaxRule` trait.

```
pub struct SamplePlugin;

impl SyntaxRule for SamplePlugin {
    fn check(&self, _syntax_tree: &SyntaxTree, node: &RefNode) -> SyntaxRuleResult {
        match node {
            RefNode::InitialConstruct(_) => SyntaxRuleResult::Fail,
            _ => SyntaxRuleResult::Pass,
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
```

`SyntaxRule` must implement `check`, `name`, `hint` and `reason`.

## Usage

svlint can load plugin by `--plugin` option.

```
$ svlint --plugin libsvlint_plugin_sample.so test.sv
Fail: sample_plugin
   --> test.sv:2:1
  |
2 | initial begin
  | ^^^^^^^ hint  : `initial` is forbidden
  |         reason: this is a sample plugin
```

The loaded plugin is automatically enabled.
