use std::{
    ffi::{CString, OsStr},
    path::{Path, PathBuf},
};

use color_eyre::eyre::Result;
use git2::{Oid, Repository, Sort};

pub fn process(
    repo: &Repository,
    subtree_root_commit: Oid,
    msg: &str,
    prefix: String,
) -> Result<()> {
    // First list all commits between the start commit and the subtree root
    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    walker.hide(subtree_root_commit)?;
    walker.set_sorting(Sort::TOPOLOGICAL)?;

    // Collect all commits that touch the subtree directory.
    let mut relevant_commits = vec![];

    for oid in walker {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        trace!(?commit, "found commit in range");
        let tree = commit.tree()?;
        for entry in tree.iter() {
            let path = match entry.name() {
                Some(s) => s,
                None => {
                    warn!(
                        "ignoring path with bytes {:?}, because it cannot be represented as utf8",
                        entry.name_bytes()
                    );
                    continue;
                }
            };
            if path.starts_with(&prefix) {
                relevant_commits.push(commit);
                break;
            }
        }
    }
    debug!("{:#?}", relevant_commits);

    // repo.treebuilder() for creating a tree in memory before writing it to disk
    Ok(())
}
