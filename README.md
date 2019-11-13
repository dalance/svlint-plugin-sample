# svlint-plugin-sample

This is a sample project of [svlint](https://github.com/dalance/svlint) plugin.

# Create plugin

svlint plugin is a shared library. So crate-type must be `dylib`.

```
[lib]
crate-type = ["dylib"]
```

All plugin must have `get_plugin` function to generate `Rule`.

```
#[no_mangle]
pub extern "C" fn get_plugin() -> *mut dyn Rule {
    let boxed = Box::new(SamplePlugin {});
    Box::into_raw(boxed)
}
```

The lint rule is defined as `Rule` trait.

```
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
```

`Rule` must implement `check`, `name`, `hint` and `reason`.

# Usage

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
