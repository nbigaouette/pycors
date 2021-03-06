use std::path::PathBuf;

use structopt::{self, StructOpt};

pub mod install;
pub mod list;
pub mod path;
pub mod run;
pub mod select;
pub mod setup;
pub mod version;

#[derive(StructOpt, Debug)]
pub struct VersionOrPath {
    version_or_path: String,
}
#[derive(StructOpt, Debug)]
pub enum Command {
    /// List installed Python versions
    #[structopt(name = "list")]
    List,

    /// Get path to active interpreter
    ///
    /// For example:
    ///     hygeia path
    ///     /usr/local/bin
    #[structopt(name = "path")]
    Path {
        /// Use specified interpreter version
        #[structopt(long = "version", short = "v")]
        version: Option<String>,
    },

    /// Get version of active interpreter
    ///
    /// For example:
    ///     hygeia version
    ///     3.7.2
    #[structopt(name = "version")]
    Version {
        /// Use specified interpreter version
        #[structopt(long = "version", short = "v")]
        version: Option<String>,
    },

    /// Select specified Python versions to use
    ///
    /// The specified Python version will _not_ be installed if not already installed.
    /// Use 'hygeia install' for this.
    ///
    /// For example:
    ///   hygeia select 3.6
    /// will select ~3.6 (the most up to date version of the 3.6 series).
    ///
    ///   hygeia select =3.7.2
    /// will select an exact version.
    #[structopt(name = "select")]
    Select(VersionOrPath),

    /// Install version, either from the provided version or from '.python-version'
    #[structopt(name = "install")]
    Install {
        /// Specified version to install
        from_version: Option<String>,

        /// Build toolchain in release mode, with optimizations (slower)
        #[structopt(short, long)]
        release: bool,

        /// Force installation, even if already installed
        #[structopt(long)]
        force: bool,

        /// Write installed version to '.python-version'
        #[structopt(long = "select", short = "s")]
        select: bool,

        #[structopt(flatten)]
        install_extra_packages: InstallExtraPackagesOptions,
    },

    /// Run a binary from the installed '.python-version'
    ///
    /// For example:
    ///     hygeia run "python -v"
    ///     hygeia run "python -c \"print('string with spaces')\""
    #[structopt(name = "run")]
    Run {
        /// Use specified interpreter version
        #[structopt(long = "version", short = "v")]
        version: Option<String>,

        command: String,
    },

    /// Setup the shim
    ///
    /// This will install pycor's binary to '~/.hygeia/bin' and add the
    /// directory to the '$PATH' environment variable (through '~/.profile').
    ///
    /// Supported shells: Bash, Fish, Zsh, PowerShell and Elvish.
    #[structopt(name = "setup")]
    Setup { shell: structopt::clap::Shell },

    /// Update pycors to latest version
    #[cfg(feature = "self-update")]
    Update,
}

#[derive(StructOpt, Debug)]
pub struct InstallExtraPackagesOptions {
    /// Install extra Python packages from file at default location
    /// ('${HYGEIA_HOME}/extra-packages-to-install.txt')
    #[structopt(long = "extra", short = "e")]
    install_extra_packages: bool,

    /// Install extra Python packages from specific file
    #[structopt(long = "extra-from", short = "f")]
    install_extra_packages_from: Option<PathBuf>,
}
