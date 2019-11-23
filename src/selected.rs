use std::{
    env,
    fs::File,
    io::{self, BufRead, BufReader, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

use failure::format_err;
use semver::VersionReq;

use crate::{constants::TOOLCHAIN_FILE, utils, Result};

pub enum VersionOrPath {
    VersionReq(semver::VersionReq),
    Path(PathBuf),
}

impl FromStr for VersionOrPath {
    type Err = std::io::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        // One can use 'latest' to mean '*'
        if s == "latest" {
            "*"
        } else {
            s
        };

        match semver::VersionReq::parse(s) {
            Ok(version_req) => {
                log::info!("Parsed {:?} as semantic version : {}", s, version_req);
                Ok(VersionOrPath::VersionReq(version_req))
            }
            Err(e) => {
                log::debug!("e: {:?}", e);
                let path = Path::new(s);
                log::info!("Parsed {:?} as Path: {:?}", s, path);
                if path.exists() {
                    Ok(VersionOrPath::Path(
                        path.canonicalize().expect("path is expected to exists"),
                    ))
                } else {
                    Err(std::io::Error::new(
                        std::io::ErrorKind::NotFound,
                        format!("Path {:?} not found", s),
                    ))
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct SelectedVersion {
    pub version: VersionReq,
}

pub fn load_selected_toolchain_file() -> Option<Result<SelectedVersion>> {
    match env::current_dir() {
        Ok(mut path) => {
            loop {
                let toolchain_file = path.join(TOOLCHAIN_FILE);
                if utils::path_exists(&toolchain_file) {
                    // We've found the file, stop.
                    log::debug!("Found file {:?}", toolchain_file);
                    break Some(SelectedVersion::from_file(toolchain_file));
                }

                if path.parent().is_none() {
                    // We are at the root directory, we haven't found anything.
                    break None;
                }

                path.pop();
            }
        }
        Err(e) => {
            log::error!("Failed to get current working directory: {:?}", e);
            Some(Err(e.into()))
        }
    }
}

impl SelectedVersion {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<SelectedVersion> {
        log::debug!("Reading configuration from file {:?}", path.as_ref());

        let input = File::open(path)?;
        let buffered = BufReader::new(input);

        // Read first line only
        let line = match buffered.lines().next() {
            None => return Err(format_err!("File does not even contains a line")),
            Some(line_result) => line_result?,
        };
        let version: VersionReq = line.parse()?;
        log::debug!("Found version \"{}\"", version);

        Ok(SelectedVersion { version })
    }

    pub fn save(&self) -> Result<usize> {
        self.save_to(TOOLCHAIN_FILE)
    }

    pub fn save_to<P: AsRef<Path>>(&self, path: P) -> Result<usize> {
        log::debug!("Writing configuration to file {:?}", path.as_ref());

        let version = format!("{}", self.version);
        let mut output = File::create(&path)?;
        let l1 = output.write(version.as_bytes())?;
        let l2 = output.write(b"\n")?;
        Ok(l1 + l2)
    }

    pub fn from_user_input() -> Result<SelectedVersion> {
        log::debug!("Reading configuration from stdin");

        let stdin = io::stdin();
        println!("Please type the Python version to use in this directory:");
        let line = match stdin.lock().lines().next() {
            None => return Err(format_err!("Standard input did not contain a single line")),
            Some(line_result) => line_result?,
        };
        log::debug!("Given: {}", line);

        let version: VersionReq = line.trim().parse()?;

        if line.is_empty() {
            log::error!("Empty line given as input.");
            Err(format_err!("Empty line provided"))
        } else {
            log::debug!("Parsed version: {}", version);
            Ok(SelectedVersion { version })
        }
    }
}
