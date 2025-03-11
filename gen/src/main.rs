use std::{
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

    fs::remove_dir_all("tests")?;
    fs::create_dir("tests")?;

    let tests = resolve_tests()?;

    for YamlTest { name, child_of } in tests {
        info!("Generating test {name}");

        let mut file = if let Some(child_of) = child_of {
            OpenOptions::new()
                .create(true)
                .append(true)
                .open(format!("tests/{child_of}.rs"))?
        } else {
            File::create(format!("tests/{name}.rs"))?
        };

        write!(
            file,
            r#"
            #[test]
            fn {name}() {{ unimplemented!() }}
        "#
        )?;
    }

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
