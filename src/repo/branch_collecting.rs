use super::repo_scanning::get_repo_receiver;
use anyhow::{anyhow, Result};
use git2::Repository;
use std::thread;
use std::{path::PathBuf, sync::mpsc};

#[derive(Debug, Clone)]
pub struct RepoBranch {
    pub repo_name: PathBuf,
    pub branch_name: String,
}

impl std::fmt::Display for RepoBranch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({:?})", self.branch_name, self.repo_name)
    }
}

pub fn collect_branches(base_dirs: &Vec<String>) -> Result<Vec<RepoBranch>> {
    let repo_receiver = get_repo_receiver(base_dirs)?;
    let (sender, receiver) = mpsc::channel::<RepoBranch>();

    for repo in repo_receiver {
        let repo = repo?;
        let thread_sender = sender.clone();

        thread::spawn(move || {
            if let Ok(branches) = get_branches_of_repo(repo) {
                branches
                    .into_iter()
                    .for_each(|it| thread_sender.send(it).unwrap())
            }
        });
    }
    drop(sender);

    Ok(receiver.into_iter().collect::<Vec<_>>())
}

fn get_branches_of_repo(repo: Repository) -> Result<Vec<RepoBranch>> {
    let mut result = Vec::new();

    for branch in repo.branches(None)? {
        let branch = branch?;
        result.push(RepoBranch {
            repo_name: repo.path().to_owned(),
            branch_name: branch
                .0
                .name()?
                .ok_or(anyhow!("branch does not have name"))?
                .to_owned(),
        })
    }

    Ok(result)
}
