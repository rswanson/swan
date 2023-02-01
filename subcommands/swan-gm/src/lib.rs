use swan_common::{Config, Response};

use git2::{Repository, build::RepoBuilder, RepositoryState};
use std::path::Path;

pub fn run(config: Config) -> Response {
    let repo_url = "https://github.com/rswanson/dotfiles.git";
    let repo_clone_path = "/Users/swanpro/.dotfiles";

    let mut builder = RepoBuilder::new();
    let repo = Repository::open(Path::new(repo_clone_path)).unwrap_or_else(|_| {
        builder.clone(repo_url, Path::new(repo_clone_path)).expect("Could not clone repo")
    });

    let status = repo.state();
    let message = match status {
        RepositoryState::Clean => "clean",
        RepositoryState::Merge => "merge",
        RepositoryState::Revert => "revert",
        RepositoryState::RevertSequence => "revertsequence",
        RepositoryState::CherryPick => "cherrypick",
        RepositoryState::CherryPickSequence => "cherrypicksequence",
        RepositoryState::Bisect => "bisect",
        RepositoryState::Rebase => "rebase",
        RepositoryState::RebaseInteractive => "rebasei",
        RepositoryState::RebaseMerge => "rebasem",
        RepositoryState::ApplyMailbox => "applymail",
        RepositoryState::ApplyMailboxOrRebase => "applemailorrebase",
    };
    Response {
        message: message.to_string(),
        exit_code: 0,
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
