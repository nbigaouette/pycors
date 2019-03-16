#[cfg(not(target_os = "windows"))]
use std::io::{BufRead, BufReader};
use std::{env, fs, io::Write};

use failure::format_err;
use structopt::clap::Shell;
#[cfg(not(target_os = "windows"))]
use structopt::StructOpt;

#[cfg(not(target_os = "windows"))]
use crate::Opt;
use crate::{utils, Result};

pub fn run(shell: Shell) -> Result<()> {
    log::debug!("Setting up the shim...");

    // Copy itself into ~/.pycors/bin
    let pycors_home_dir = utils::pycors_home()?;
    let bin_dir = pycors_home_dir.join("shims");
    if !utils::path_exists(&bin_dir) {
        log::debug!("Directory {:?} does not exists, creating.", bin_dir);
        fs::create_dir_all(&bin_dir)?;
    }
    let copy_from = env::current_exe()?;
    let copy_to = bin_dir.join("pycors");
    log::debug!("Copying {:?} into {:?}...", copy_from, copy_to);
    utils::copy_file(&copy_from, &copy_to)?;

    // Once the shim is in place, create hard links to it.
    let hardlinks_version_suffix = &[
        "python###",
        "idle###",
        "pip###",
        "pydoc###",
        // Internals
        "python###-config",
        "python###dm-config",
        // Extras
        "pipenv###",
        "poetry###",
        "pytest###",
    ];
    let hardlinks_dash_version_suffix = &["2to3###", "easy_install###", "pyvenv###"];

    // Create simple hardlinks: `pycors` --> `bin`
    utils::create_hard_links(&copy_from, hardlinks_version_suffix, &bin_dir, "")?;
    utils::create_hard_links(&copy_from, hardlinks_dash_version_suffix, &bin_dir, "")?;

    // Create major version hardlinks: `pycors` --> `bin3` and `pycors` --> `bin2`
    for major in &["2", "3"] {
        utils::create_hard_links(&copy_from, hardlinks_version_suffix, &bin_dir, major)?;
        utils::create_hard_links(
            &copy_from,
            hardlinks_dash_version_suffix,
            &bin_dir,
            &format!("-{}", major),
        )?;
    }

    // Create an dummy file that will be recognized when searching the PATH for
    // python interpreters. We don't want to "find" the shims we install here.
    let pycors_dummy_file = bin_dir.join("pycors_dummy_file");
    let mut file = fs::File::create(&pycors_dummy_file)?;
    writeln!(file, "This file's job is to tell pycors the directory contains shim, not real Python interpreters.")?;

    // Add ~/.pycors/bin to $PATH in ~/.bash_profile and install autocomplete
    match shell {
        structopt::clap::Shell::Bash => {
            #[cfg(target_os = "windows")]
            {
                let message = "Windows support not yet implemented.";
                log::error!("{}", message);
                Err(format_err!("{}", message))
            }
            #[cfg(not(target_os = "windows"))]
            {
                let home =
                    dirs::home_dir().ok_or_else(|| format_err!("Error getting home directory"))?;
                let bash_profile = home.join(".bash_profile");

                // Add the autocomplete too
                let autocomplete_file = pycors_home_dir.join("pycors.bash-completion");
                let mut f = fs::File::create(&autocomplete_file)?;
                Opt::clap().gen_completions_to("pycors", shell, &mut f);

                log::debug!("Adding {:?} to $PATH in {:?}...", bin_dir, bash_profile);
                let bash_profile_line = format!(r#"export PATH="{}:$PATH""#, bin_dir.display());

                let do_edit_bash_profile = if !bash_profile.exists() {
                    true
                } else {
                    // Verify that file does not contain a line `export PATH=...`

                    let f = fs::File::open(&bash_profile)?;
                    let f = BufReader::new(f);
                    let mut line_found = false;
                    for line in f.lines() {
                        match line {
                            Err(e) => log::error!(
                                "Failed to read line from file {:?}: {:?}",
                                bash_profile,
                                e,
                            ),
                            Ok(line) => {
                                if line == bash_profile_line {
                                    log::debug!(
                                        "File {:?} already contains PATH export. Skipping.",
                                        bash_profile
                                    );
                                    line_found = true;
                                    break;
                                }
                            }
                        }
                    }

                    !line_found
                };

                if do_edit_bash_profile {
                    let bash_profile_existed = bash_profile.exists();
                    let mut file = fs::OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(&bash_profile)?;
                    let lines = &[
                        String::from(""),
                        "#################################################".to_string(),
                        "# These lines were added by pycors.".to_string(),
                        "# See https://github.com/nbigaouette/pycors".to_string(),
                        if !bash_profile_existed {
                            "source ${HOME}/.bashrc".to_string()
                        } else {
                            String::from("")
                        },
                        bash_profile_line,
                        format!(r#"source "{}""#, autocomplete_file.display()),
                        "#################################################".to_string(),
                    ];
                    for line in lines {
                        // debug!("    {}", line);
                        writeln!(file, "{}", line)?;
                    }
                }

                Ok(())
            }
        }
        _ => Err(format_err!("Unsupported shell: {}", shell)),
    }
}
