use lazy_static::lazy_static;

macro_rules! executable_name_from_env {
    () => {
        env!("CARGO_PKG_NAME")
    };
}

/// Name of the executable, reused across project.
pub const EXECUTABLE_NAME: &str = executable_name_from_env!();

/// Default hidden configuration directory.
pub const DEFAULT_DOT_DIR: &str = concat!(".", executable_name_from_env!());

pub const SHELL_CONFIG_IDENTIFYING_PATTERN_START: &str =
    concat!("Start of ", executable_name_from_env!(), " config block.");

pub const SHELL_CONFIG_IDENTIFYING_PATTERN_END: &str =
    concat!("End of ", executable_name_from_env!(), " config block.");

/// Return the environment variable used to find the project's config home.
pub fn project_home_env_variable() -> &'static str {
    lazy_static! {
        static ref HOME_ENV_VARIABLE: String =
            format!("{}_HOME", executable_name_from_env!().to_uppercase());
    }
    &HOME_ENV_VARIABLE
}

/// Return the environment variable used to overwrite the home directory
pub fn home_overwrite_env_variable() -> &'static str {
    lazy_static! {
        static ref HOME_OVERWRITE_ENV_VARIABLE: String = format!(
            "{}_OVERWRITE_HOME",
            executable_name_from_env!().to_uppercase()
        );
    }
    &HOME_OVERWRITE_ENV_VARIABLE
}

/// Return the environment variable used to overwrite the Document directory
pub fn document_overwrite_env_variable() -> &'static str {
    lazy_static! {
        static ref DOCUMENT_OVERWRITE_ENV_VARIABLE: String = format!(
            "{}_OVERWRITE_DOCUMENT",
            executable_name_from_env!().to_uppercase()
        );
    }
    &DOCUMENT_OVERWRITE_ENV_VARIABLE
}

/// Filename describing which version of this project installed a toolchain.
pub const INFO_FILE: &str = concat!("installed_by_", executable_name_from_env!(), ".txt");

/// Filename containing the list of extra packages to install using `pip`.
pub const EXTRA_PACKAGES_FILENAME: &str = "extra-packages-to-install.txt";

/// Content of file listing extra `pip` packages to install, copied when setting-up shim.
pub const EXTRA_PACKAGES_FILENAME_CONTENT: &str = include_str!("../extra-packages-to-install.txt");

pub const TOOLCHAIN_FILE: &str = ".python-version";

// Note: Trailing '/' is required for proper parsing
pub const PYTHON_SOURCE_INDEX_URL: &str = "https://www.python.org/downloads/source/";

// Note: Trailing '/' is required for proper parsing
pub const PYTHON_WINDOWS_INDEX_URL: &str = "https://www.python.org/downloads/windows/";

pub const AVAILABLE_TOOLCHAIN_CACHE: &str = "available_toolchains.json";

pub const SHIMS_DIRECTORY_IDENTIFIER_FILE: &str = "this_is_a_shims_directory.txt";
