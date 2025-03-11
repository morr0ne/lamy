use std::{fs, path::Path};
// use tracing::warn;
use anyhow::{Result, anyhow};
use heck::ToSnekCase;
use tracing::info;
use xshell::{Shell, cmd};

#[derive(Debug)]
struct YamlTest {
    name: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let sh = Shell::new()?;
    sh.change_dir("yaml-test-suite");
    cmd!(sh, "make data").run()?;

    let tests = resolve_tests()?;

    for YamlTest { name } in tests {
        info!("Generating test {name}");

        fs::write(
            format!("tests/{name}.rs"),
            format!(
                r#"
            #[test]
            fn {name}() {{ unimplemented!() }}
        "#
            ),
        )?;
    }

    Ok(())
}

fn resolve_tests() -> Result<Vec<YamlTest>> {
    let mut test_folders = Vec::new();

    for entry in fs::read_dir("yaml-test-suite/data")? {
        let path = entry?.path();

        if path.ends_with(".git") || path.ends_with("name") || path.ends_with("tags") {
            continue;
        }

        let mut is_nested = false;

        for entry in fs::read_dir(&path)? {
            let path = entry?.path();

            if path.is_dir() {
                test_folders.push(path);
                is_nested = true
            }
        }

        if !is_nested {
            test_folders.push(path);
        }
    }

    let mut tests = Vec::new();

    for folder in test_folders {
        let name = fs::read_to_string(folder.join("==="))?.to_snek_case();

        tests.push(YamlTest { name });
    }

    Ok(tests)
}
