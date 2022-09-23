## Subtree Management Tool

crab-subtree is a greenfield implementation of subtree management.
It is an attempt at speeding up subtree operations on large repositories significantly.

### Some numbers

collected against rustc commit 1bcdd0c96d6b323a5dc59698cef8d3f8c6bcb20a

#### Finding all relevant commits since git subtree add

* rust-analyzer: 1min (10k commits)
* miri 3s (219 commits)
* clippy 8min (86k commits)
