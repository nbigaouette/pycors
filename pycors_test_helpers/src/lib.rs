use std::{
    env,
    fs::{self, File},
    io::Write,
    path::Path,
};

pub use anyhow::{Context, Result};

#[cfg(windows)]
const EXEC_EXTENSION: &str = ".exe";
#[cfg(not(windows))]
const EXEC_EXTENSION: &str = "";

pub fn init_logger() {
    env::var("RUST_LOG")
        .or_else(|_| -> Result<String, ()> {
            let rust_log = "debug".to_string();
            println!("Environment variable 'RUST_LOG' not set.");
            println!("Setting to: {}", rust_log);
            env::set_var("RUST_LOG", &rust_log);
            Ok(rust_log)
        })
        .unwrap();
    let _ = env_logger::try_init();
}

pub struct MockedOutput<'a> {
    pub out: Option<&'a str>,
    pub err: Option<&'a str>,
}

pub fn mock_executable<P, S>(
    executable_location: P,
    executable_name: S,
    output: MockedOutput,
) -> crate::Result<()>
where
    P: AsRef<Path>,
    S: AsRef<str>,
{
    _mock_executable(
        executable_location.as_ref(),
        executable_name.as_ref(),
        output,
    )
}

fn _mock_executable(
    executable_location: &Path,
    executable_name: &str,
    output: MockedOutput,
) -> crate::Result<()> {
    let _cargo_output = std::process::Command::new("cargo")
        .args(&["build", "--package", "print_file_to_stdout"])
        .output()
        .with_context(|| "Failed to execute 'cargo build --package print_file_to_stdout")?;

    if !executable_location.exists() {
        fs::create_dir_all(&executable_location)?
    }

    let stdout_filepath = executable_location.join(format!(
        "{}{}_pycors_tests_to_print_stdout.txt",
        executable_name, EXEC_EXTENSION
    ));
    let stderr_filepath = executable_location.join(format!(
        "{}{}_pycors_tests_to_print_stderr.txt",
        executable_name, EXEC_EXTENSION
    ));

    if stdout_filepath.exists() {
        fs::remove_file(&stdout_filepath)
            .with_context(|| format!("Failed to remove file {:?}", stdout_filepath))?;
    }
    if stderr_filepath.exists() {
        fs::remove_file(&stderr_filepath)
            .with_context(|| format!("Failed to remove file {:?}", stderr_filepath))?;
    }

    if let Some(stdout) = output.out {
        let mut f = File::create(&stdout_filepath)
            .with_context(|| format!("Failed to create file {:?}", stdout_filepath))?;
        f.write_all(stdout.as_bytes())
            .with_context(|| format!("Failed to write to file {:?}", stdout_filepath))?;
    }
    if let Some(stderr) = output.err {
        let mut f = File::create(&stderr_filepath)
            .with_context(|| format!("Failed to create file {:?}", stderr_filepath))?;
        f.write_all(stderr.as_bytes())
            .with_context(|| format!("Failed to write to file {:?}", stderr_filepath))?;
    }

    let print_file_to_stdout = {
        let target_dir = match env::var("CARGO_TARGET_DIR") {
            Ok(dir) => dir,
            Err(_) => String::from("target"),
        };

        #[cfg_attr(not(windows), allow(unused_mut))]
        let mut tmp = Path::new(&target_dir)
            .join("debug")
            .join("print_file_to_stdout");

        #[cfg(windows)]
        tmp.set_extension("exe");

        tmp
    };

    fs::copy(
        &print_file_to_stdout,
        executable_location.join(format!("{}{}", executable_name, EXEC_EXTENSION)),
    )
    .with_context(|| {
        format!(
            "Failed to copy {:?} to {:?}",
            print_file_to_stdout,
            executable_location.join(format!("{}{}", executable_name, EXEC_EXTENSION))
        )
    })?;

    Ok(())
}

#[macro_export]
macro_rules! create_test_temp_dir {
    () => {{
        let dir = std::env::temp_dir()
            .join("pycors")
            .join("integration_tests");

        if !dir.exists() {
            std::fs::create_dir_all(&dir).unwrap();
        }
        let mut dir = dir.canonicalize().unwrap();

        // ---------------------------------
        // https://stackoverflow.com/a/40234666
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }
        let name = type_name_of(f);
        let function_path = &name[..name.len() - 3];
        // ---------------------------------

        for component in function_path.split("::").skip(1) {
            dir.push(component);
        }

        if dir.exists() {
            std::fs::remove_dir_all(&dir).unwrap();
        }

        std::fs::create_dir_all(&dir).unwrap();

        dir
    }};
}