use super::repo_scanning::get_repo_receiver;
use anyhow::{anyhow, Result};
use git2::{Branch, BranchType, Repository};
use itertools::Itertools;
use std::thread;
use std::{path::PathBuf, sync::mpsc};

#[derive(Debug, Clone)]
pub struct RepoBranch {
    pub repo_name: PathBuf,
    pub branch_name: String,
    pub latest_commit_sec: Option<i64>,
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

    let mut result = receiver.into_iter().collect_vec();
    result.sort_unstable_by_key(|it| -it.latest_commit_sec.unwrap_or(0));
    Ok(result)
}

fn get_branches_of_repo(repo: Repository) -> Result<Vec<RepoBranch>> {
    let mut result = Vec::new();

    for branch_iter_result in repo.branches(Some(BranchType::Local))? {
        let (branch, _branch_type) = branch_iter_result?;

        let branch_name = branch
            .name()?
            .ok_or(anyhow!("branch does not have name"))?
            .to_owned();

        let latest_commit_sec = get_latest_commit_epoch_sec(branch);

        result.push(RepoBranch {
            repo_name: repo.path().to_owned(),
            branch_name,
            latest_commit_sec,
        })
    }

    Ok(result)
}

fn get_latest_commit_epoch_sec(branch: Branch) -> Option<i64> {
    Some(
        branch
            .into_reference()
            .peel_to_commit()
            .ok()?
            .time()
            .seconds(),
    )
}
