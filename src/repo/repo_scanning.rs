use anyhow::{anyhow, Result};
use git2::Repository;
use std::fs::*;
use std::sync::mpsc::{channel, Receiver};
use std::thread;

pub fn get_repo_receiver(base_dirs: &Vec<String>) -> Result<Receiver<Result<Repository>>> {
    let (sender, receiver) = channel::<Result<Repository>>();

    for base_dir in base_dirs {
        let rdir = read_dir(base_dir)?;
        let thread_sender = sender.clone();

        thread::spawn(move || {
            for dir_entry in rdir {
                match find_repo(dir_entry) {
                    Err(err) => thread_sender.send(Err(anyhow!(err))).unwrap(),
                    Ok(Some(it)) => thread_sender.send(Ok(it)).unwrap(),
                    Ok(None) => {}
                }
            }
        });
    }

    Ok(receiver)
}

fn find_repo(dir_entry: Result<DirEntry, std::io::Error>) -> Result<Option<Repository>> {
    let dir_entry = dir_entry?;
    if dir_entry.file_type()?.is_dir() {
        Ok(Repository::open(dir_entry.path()).ok())
    } else {
        Ok(None)
    }
}
