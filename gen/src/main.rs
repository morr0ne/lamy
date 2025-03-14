use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    io::Write as _,
    path::PathBuf,
};
// use tracing::warn;
use anyhow::Result;
use heck::ToSnekCase;
use tracing::info;
use xshell::{Shell, cmd};

struct YamlTest {
    child_of: Option<String>,
    name: String,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let sh = Shell::new()?;
    sh.change_dir("yaml-test-suite");
    cmd!(sh, "make data").run()?;

    fs::remove_dir_all("tests/test_suite")?;
    fs::create_dir("tests/test_suite")?;

    let tests = resolve_tests()?;

    let mut suite_module = HashSet::new();

    for YamlTest { name, child_of } in tests {
        info!("Generating test {name}");

        let mut file = if let Some(child_of) = child_of {
            suite_module.insert(format!("mod {child_of};"));

            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("tests/test_suite/{child_of}.rs"))?
        } else {
            suite_module.insert(format!("mod {name};"));
            File::create(format!("tests/test_suite/{name}.rs"))?
        };

        write!(
            file,
            r#"
            #[test]
            fn {name}() {{ unimplemented!() }}
        "#
        )?;
    }

    let suite_module: String = suite_module.into_iter().collect();

    fs::write("tests/test_suite/mod.rs", &suite_module)?;
    fs::write("tests/suite.rs", "mod test_suite;")?;

    Ok(())
}

fn resolve_tests() -> Result<Vec<YamlTest>> {
    let mut tests = Vec::new();

    for outer_entry in fs::read_dir("yaml-test-suite/data/name")? {
        let outer_path = outer_entry?.path();

        let mut is_nested = false;

        for entry in fs::read_dir(&outer_path)? {
            let path: PathBuf = entry?.path();

            if path.is_dir() {
                let name = fs::read_to_string(path.join("==="))?.to_snek_case();

                tests.push(YamlTest {
                    name: format!("{name}_{}", path.file_name().unwrap().to_str().unwrap()),
                    child_of: Some(
                        outer_path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .to_snek_case(),
                    ),
                });

                is_nested = true
            }
        }

        if !is_nested {
            let name = fs::read_to_string(outer_path.join("==="))?.to_snek_case();

            tests.push(YamlTest {
                name,
                child_of: None,
            });
        }
    }

    Ok(tests)
}
