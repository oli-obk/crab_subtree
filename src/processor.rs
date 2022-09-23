use color_eyre::eyre::Result;
use git2::{Oid, Repository, Sort};

pub fn process(repo: &Repository, subtree_root_commit: Oid, msg: &str) -> Result<()> {
    // First list all commits between the start commit and the subtree root
    let mut walker = repo.revwalk()?;
    walker.push_head()?;
    walker.hide(subtree_root_commit)?;
    walker.set_sorting(Sort::TOPOLOGICAL)?;

    for oid in walker {
        let oid = oid?;
        let commit = repo.find_commit(oid)?;
        trace!(?commit);
    }

    // repo.treebuilder() for creating a tree in memory before writing it to disk
    Ok(())
}
