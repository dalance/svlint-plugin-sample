use std::env;
use std::fs::{File, read_dir};
use std::io::{BufRead, BufReader, Write};
use std::path::Path;

fn write_test_rs(testcases: &Vec<(String, bool)>) -> () {
    let out_dir = env::var("OUT_DIR").unwrap();
    let o = Path::new(&out_dir).join("test.rs");
    let mut o = File::create(&o).unwrap();

    for (path, pass_not_fail) in testcases {
        let passfail = if *pass_not_fail { "pass" } else { "fail" };

        let lines = BufReader::new(File::open(path).unwrap())
            .lines()
            .collect::<Result<Vec<_>, _>>()
            .unwrap();

        let sep = "/".repeat(80);
        let subtests: Vec<&[String]> = lines
            .as_slice()
            .split(|l| l.contains(sep.as_str()))
            .collect();
        let n_subtests: usize = subtests.len();

        let testname = Path::new(path).file_stem().unwrap().to_str().unwrap();

        for (t, subtest) in subtests.into_iter().enumerate().map(|(i, x)| (i + 1, x)) {
            // Write subtest to its own file.
            let subtest_path = Path::new(&out_dir)
                .join(format!("subtest.{testname}.{passfail}.{t}of{n_subtests}.sv"));
            let mut out_subtest = File::create(&subtest_path).unwrap();
            for line in subtest {
                let _ = writeln!(out_subtest, "{}", line);
            }

            // Create call to `lib.rs::tests::plugin_test()` via `tests.rs`.
            let subtest_name = format!("{testname}_{passfail}_{t}of{n_subtests}");
            let _ = writeln!(o, "#[test]");
            let _ = writeln!(o, "fn {}() {{", subtest_name);
            if *pass_not_fail {
                let _ = writeln!(
                    o,
                    "    plugin_test({subtest_path:?}, true);"
                );
            } else {
                let _ = writeln!(
                    o,
                    "    plugin_test({subtest_path:?}, false);"
                );
            }
            let _ = writeln!(o, "}}");
        }
    }
}

fn main() {

    let mut testcases: Vec<(String, bool)> = Vec::new();

    if let Ok(entries) = read_dir("testcases/fail") {
        for entry in entries {
            if let Ok(entry) = entry {
                let p = String::from(entry.path().to_string_lossy());
                testcases.push((p, false));
            }
        }
    }

    if let Ok(entries) = read_dir("testcases/pass") {
        for entry in entries {
            if let Ok(entry) = entry {
                let p = String::from(entry.path().to_string_lossy());
                testcases.push((p, true));
            }
        }
    }

    testcases.sort_by(|a, b| a.0.cmp(&b.0));

    write_test_rs(&testcases);
}
