use {
    std::{
        fmt,
        path::Path,
        str::FromStr,
    },
    git2::Repository,
    lazy_regex::regex_captures,
    serde::Deserialize,
};

struct Version {
    base: ootr_utils::Version,
    subsupplementary: Option<u8>,
}

impl FromStr for Version {
    type Err = <ootr_utils::Version as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            base: s.parse()?,
            subsupplementary: None,
        })
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.base.fmt(f)?;
        if let Some(subsupplementary) = self.subsupplementary {
            write!(f, " riir-{subsupplementary}")?;
        }
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Git(#[from] git2::Error),
    #[error(transparent)] ParseInt(#[from] std::num::ParseIntError),
    #[error(transparent)] SemVer(#[from] semver::Error),
    #[error(transparent)] Toml(#[from] toml::de::Error),
    #[error(transparent)] TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)] Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)] VersionParse(#[from] ootr_utils::VersionParseError),
    #[error("failed to find base version line")]
    Base,
    #[error("failed to find branch identifier line")]
    Branch,
    #[error("failed to convert git object")]
    GitObject,
    #[error("unknown branch name in Cargo.toml version")]
    RustBranch,
    #[error("failed to parse prerelease segment of Cargo.toml version")]
    RustParse,
    #[error("failed to find supplementary version line")]
    Supplementary,
}

impl<'a> From<git2::Object<'a>> for Error {
    fn from(_: git2::Object<'a>) -> Self {
        Self::GitObject
    }
}

#[wheel::main]
fn main() -> Result<(), Error> {
    let repo = Repository::open("C:/Users/fenhl/git/github.com/fenhl/OoT-Randomizer/stage")?;
    let mut commit = repo.find_branch("riir", git2::BranchType::Local)?.into_reference().peel_to_commit()?;
    loop {
        let version = if commit.id() == git2::Oid::from_str("c435f0af131492386709700eaefc7840c2b992a1")? {
            Version {
                base: ootr_utils::Version::from_dev(0, 0, 0),
                subsupplementary: None,
            }
        } else {
            let tree = commit.tree()?;
            if let Ok(cargo_toml) = tree.get_path(Path::new("Cargo.toml")) {
                #[derive(Deserialize)]
                struct CargoManifest {
                    workspace: CargoWorkspace,
                }

                #[derive(Deserialize)]
                struct CargoWorkspace {
                    package: CargoPackage,
                }

                #[derive(Deserialize)]
                struct CargoPackage {
                    version: semver::Version,
                }

                let cargo_toml = cargo_toml.to_object(&repo)?.into_blob()?;
                let cargo_toml = std::str::from_utf8(cargo_toml.content())?;
                let version = if let Ok(manifest) = toml::from_str::<CargoManifest>(cargo_toml) {
                    manifest.workspace.package.version
                } else {
                    toml::from_str::<CargoWorkspace>(cargo_toml)?.package.version
                };
                let [branch, supplementary, subbranch, subsupplementary] = version.pre.split('.').collect::<Vec<_>>().try_into().map_err(|_| Error::RustParse)?;
                if subbranch != "riir" { return Err(Error::RustBranch) }
                Version {
                    base: ootr_utils::Version::from_branch(match branch {
                        "fenhl" => ootr_utils::Branch::DevFenhl,
                        _ => return Err(Error::RustBranch),
                    }, version.major.try_into()?, version.minor.try_into()?, version.patch.try_into()?, supplementary.parse()?),
                    subsupplementary: Some(subsupplementary.parse()?),
                }
            } else {
                let base_version_line = ["version.py", "Settings.py", "Main.py"].into_iter()
                    .filter_map(|path| {
                        let blob = tree.get_path(Path::new(path)).ok()?.to_object(&repo).ok()?.into_blob().ok()?;
                        let version_py = std::str::from_utf8(blob.content()).ok()?;
                        Some(version_py.lines().find_map(|line| line.strip_prefix("__version__ = "))?.to_owned())
                    })
                    .next().ok_or(Error::Base)?;
                if let Some((_, base_version)) = regex_captures!("^'([0-9.]+) (?:Release|f\\.LUM|f\\.Lum)'$", &base_version_line) {
                    base_version.parse()?
                } else if let Some((_, version)) = regex_captures!("^'([0-9.]+ .+)'$", &base_version_line) {
                    version.parse()?
                } else {
                    let (_, base_version) = regex_captures!("^'([0-9.]+)'$", &base_version_line).ok_or(Error::Base)?;
                    if let Ok(path) = tree.get_path(Path::new("version.py")) {
                        let blob = path.to_object(&repo)?.into_blob()?;
                        let version_py = std::str::from_utf8(blob.content())?;
                        let supplementary = version_py.lines()
                            .filter_map(|line| regex_captures!("^supplementary_version = ([0-9]+)$", line))
                            .find_map(|(_, supplementary_version)| supplementary_version.parse::<u8>().ok())
                            .ok_or(Error::Supplementary)?;
                        let base_version = semver::Version::parse(base_version)?;
                        if supplementary == 0 {
                            Version {
                                base: ootr_utils::Version::from_dev(base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?),
                                subsupplementary: None,
                            }
                        } else {
                            let branch = version_py.lines()
                                .find_map(|line| match line {
                                    "branch_identifier = 0xfe" => Some(ootr_utils::Branch::DevFenhl),
                                    _ => None,
                                })
                                .ok_or(Error::Branch)?;
                            Version {
                                base: ootr_utils::Version::from_branch(branch, base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?, supplementary),
                                subsupplementary: None,
                            }
                        }
                    } else {
                        let base_version = semver::Version::parse(base_version)?;
                        Version {
                            base: ootr_utils::Version::from_dev(base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?),
                            subsupplementary: None,
                        }
                    }
                }
            }
        };
        println!("{version} {}", commit.id());
        commit = if commit.id() == git2::Oid::from_str("83d39ffa9d8fb0af5465d9eb4d66660dea5b27f8")? {
            // fix discontinuity from dev-fenhl-rebase
            repo.find_commit(git2::Oid::from_str("73d3791e9cb335a64489304ea4be6be489d5f29d")?)?
        } else if let Some(parent) = commit.parents().next() {
            parent
        } else {
            break
        };
    }
    Ok(())
}
