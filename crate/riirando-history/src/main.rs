use {
    std::path::Path,
    git2::Repository,
    lazy_regex::regex_captures,
};

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)] Git(#[from] git2::Error),
    #[error(transparent)] SemVer(#[from] semver::Error),
    #[error(transparent)] TryFromInt(#[from] std::num::TryFromIntError),
    #[error(transparent)] Utf8(#[from] std::str::Utf8Error),
    #[error(transparent)] VersionParse(#[from] ootr_utils::VersionParseError),
    #[error("failed to find base version line")]
    Base,
    #[error("failed to find branch identifier line")]
    Branch,
    #[error("failed to convert git object")]
    GitObject,
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
    let mut commit = repo.find_branch("dev-fenhl", git2::BranchType::Local)?.into_reference().peel_to_commit()?; //TODO implement version parsing from Cargo.toml and include riir branch?
    loop {
        let version = if commit.id() == git2::Oid::from_str("c435f0af131492386709700eaefc7840c2b992a1")? {
            ootr_utils::Version::from_dev(0, 0, 0)
        } else {
            let tree = commit.tree()?;
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
                        ootr_utils::Version::from_dev(base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?)
                    } else {
                        let branch = version_py.lines()
                            .find_map(|line| match line {
                                "branch_identifier = 0xfe" => Some(ootr_utils::Branch::DevFenhl),
                                _ => None,
                            })
                            .ok_or(Error::Branch)?;
                        ootr_utils::Version::from_branch(branch, base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?, supplementary)
                    }
                } else {
                    let base_version = semver::Version::parse(base_version)?;
                    ootr_utils::Version::from_dev(base_version.major.try_into()?, base_version.minor.try_into()?, base_version.patch.try_into()?)
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
