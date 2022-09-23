use std::path::PathBuf;

use color_eyre::eyre::Result;
use git2::{Oid, Repository, Sort};

pub fn process(repo: &Repository, subtree_root_commit: Oid, prefix: PathBuf) -> Result<()> {
    // First list all commits between the start commit and the subtree root
    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    walker.hide(subtree_root_commit)?;
    walker.set_sorting(Sort::TOPOLOGICAL)?;

    let commits = walker.collect::<Result<Vec<_>, _>>()?;

    // Collect all commits that touch the subtree directory.
    let mut relevant_commits = vec![];

    'commits: for (i, &oid) in commits.iter().enumerate() {
        eprint!(
            "\r{i}/{} {}% found {} relevant commits",
            commits.len(),
            i * 100 / commits.len(),
            relevant_commits.len()
        );
        let commit = repo.find_commit(oid)?;
        if commit.parent_count() > 1 {
            // We ignore merge commits, as the rustc repo only has trivial merges.
            trace!(?commit, "skipping merge commit");
            continue;
        }
        trace!(?commit, "found commit in range");
        let parent;
        let diff = repo.diff_tree_to_tree(
            if commit.parent_count() == 1 {
                parent = commit.parent(0)?.tree()?;
                Some(&parent)
            } else {
                None
            },
            Some(&commit.tree()?),
            None,
        )?;
        for delta in diff.deltas() {
            for file in [delta.old_file(), delta.new_file()] {
                let path = file.path().unwrap();
                if path.starts_with(&prefix) {
                    relevant_commits.push(commit.clone());
                    continue 'commits;
                }
            }
        }
    }
    eprintln!("\rdone                                                   ");
    debug!("{:#?}", relevant_commits);

    // repo.treebuilder() for creating a tree in memory before writing it to disk
    Ok(())
}
