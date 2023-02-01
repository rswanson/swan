use swan_common::{Config, Response};

use git2::{build::RepoBuilder, Repository, RepositoryState};

use std::io::{self, Write};
use std::str;
use std::{path::Path};

pub fn run(_config: Config) -> Response {
    let repo_url = "https://github.com/rswanson/dotfiles.git";
    let repo_clone_path = "/Users/swanpro/.dotfiles";

    let mut builder = RepoBuilder::new();
    let repo = Repository::open(Path::new(repo_clone_path))
        .unwrap_or_else(|_| builder.clone(repo_url, Path::new(repo_clone_path)).unwrap());
    let remote_name = "origin";
    let remote_branch = "main";
    let mut remote = repo.find_remote(remote_name).unwrap();
    let fetch_commit = do_fetch(&repo, &[remote_branch], &mut remote).unwrap();
    let analysis = repo.merge_analysis(&[&fetch_commit]).unwrap();
    if analysis.0.is_fast_forward() {
        println!("Doing a fast forward");
        // do a fast forward
        let refname = format!("refs/heads/{}", remote_branch);
        match repo.find_reference(&refname) {
            Ok(mut r) => {
                fast_forward(&repo, &mut r, &fetch_commit).unwrap();
            }
            Err(_) => {
                // The branch doesn't exist so just set the reference to the
                // commit directly. Usually this is because you are pulling
                // into an empty repository.
                repo.reference(
                    &refname,
                    fetch_commit.id(),
                    true,
                    &format!("Setting {} to {}", remote_branch, fetch_commit.id()),
                ).unwrap();
                repo.set_head(&refname).unwrap();
                repo.checkout_head(Some(
                    git2::build::CheckoutBuilder::default()
                        .allow_conflicts(true)
                        .conflict_style_merge(true)
                        .force(),
                )).unwrap();
            }
        };
    }
    // let message = match status {
    //     RepositoryState::Clean => "clean",
    //     RepositoryState::Merge => "merge",
    //     RepositoryState::Revert => "revert",
    //     RepositoryState::RevertSequence => "revertsequence",
    //     RepositoryState::CherryPick => "cherrypick",
    //     RepositoryState::CherryPickSequence => "cherrypicksequence",
    //     RepositoryState::Bisect => "bisect",
    //     RepositoryState::Rebase => "rebase",
    //     RepositoryState::RebaseInteractive => "rebasei",
    //     RepositoryState::RebaseMerge => "rebasem",
    //     RepositoryState::ApplyMailbox => "applymail",
    //     RepositoryState::ApplyMailboxOrRebase => "applemailorrebase",
    // };
    Response {
        message: "".to_string(),
        exit_code: 0,
    }
}


fn fast_forward(
    repo: &Repository,
    lb: &mut git2::Reference,
    rc: &git2::AnnotatedCommit,
) -> Result<(), git2::Error> {
    let name = match lb.name() {
        Some(s) => s.to_string(),
        None => String::from_utf8_lossy(lb.name_bytes()).to_string(),
    };
    let msg = format!("Fast-Forward: Setting {} to id: {}", name, rc.id());
    println!("{}", msg);
    lb.set_target(rc.id(), &msg)?;
    repo.set_head(&name)?;
    repo.checkout_head(Some(
        git2::build::CheckoutBuilder::default()
            // For some reason the force is required to make the working directory actually get updated
            // I suspect we should be adding some logic to handle dirty working directory states
            // but this is just an example so maybe not.
            .force(),
    ))?;
    Ok(())
}

fn do_fetch<'a>(
    repo: &'a git2::Repository,
    refs: &[&str],
    remote: &'a mut git2::Remote,
) -> Result<git2::AnnotatedCommit<'a>, git2::Error> {
    let mut cb = git2::RemoteCallbacks::new();

    // Print out our transfer progress.
    cb.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            print!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            print!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        io::stdout().flush().unwrap();
        true
    });

    let mut fo = git2::FetchOptions::new();
    fo.remote_callbacks(cb);
    // Always fetch all tags.
    // Perform a download and also update tips
    fo.download_tags(git2::AutotagOption::All);
    println!("Fetching {} for repo", remote.name().unwrap());
    remote.fetch(refs, Some(&mut fo), None)?;

    // If there are local objects (we got a thin pack), then tell the user
    // how many objects we saved from having to cross the network.
    let stats = remote.stats();
    if stats.local_objects() > 0 {
        println!(
            "\rReceived {}/{} objects in {} bytes (used {} local \
             objects)",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes(),
            stats.local_objects()
        );
    } else {
        println!(
            "\rReceived {}/{} objects in {} bytes",
            stats.indexed_objects(),
            stats.total_objects(),
            stats.received_bytes()
        );
    }

    let fetch_head = repo.find_reference("FETCH_HEAD")?;
    Ok(repo.reference_to_annotated_commit(&fetch_head)?)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {
        assert_eq!(4, 4);
    }
}
