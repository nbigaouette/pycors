use std::{env, fs, path::PathBuf};

use assert_cmd::{assert::OutputAssertExt, Command};
use predicates::{boolean::PredicateBooleanExt, prelude::predicate};

pub fn temp_dir(subdir: &str) -> PathBuf {
    let dir = env::temp_dir().join("pycors").join("integration_tests");

    if !dir.exists() {
        fs::create_dir_all(&dir).unwrap();
    }
    let dir = dir.canonicalize().unwrap().join(subdir);

    if dir.exists() {
        fs::remove_dir_all(&dir).unwrap();
    }

    fs::create_dir_all(&dir).unwrap();

    dir
}

mod integration {
    use super::*;

    fn test_version(output: std::process::Output) {
        let assert_output = output.assert();

        assert_output
            .success()
            .stdout(format!(
                "{} {}\n",
                env!("CARGO_PKG_NAME"),
                env!("CARGO_PKG_VERSION")
            ))
            .stderr("");
    }

    #[test]
    fn version_long() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let output = cmd.arg("--version").unwrap();
        test_version(output);
    }

    #[test]
    fn version_short() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let output = cmd.arg("-V").unwrap();
        test_version(output);
    }

    fn test_help(output: std::process::Output) {
        let assert_output = output.assert();

        assert_output
            .success()
            .stdout(
                predicate::str::starts_with(format!(
                    "{} {}\n",
                    env!("CARGO_PKG_NAME"),
                    env!("CARGO_PKG_VERSION")
                ))
                .and(predicate::str::contains("USAGE:"))
                .and(predicate::str::contains("FLAGS:"))
                .and(predicate::str::contains("SUBCOMMANDS:")),
            )
            .stderr("");
    }

    #[test]
    fn help_long() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let output = cmd.arg("--help").unwrap();
        test_help(output);
    }

    #[test]
    fn help_short() {
        let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
        let output = cmd.arg("-h").unwrap();
        test_help(output);
    }
}