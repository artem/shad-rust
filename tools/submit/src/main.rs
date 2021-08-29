use anyhow::{bail, Context, Result};
use git2::{Repository, RepositoryOpenFlags, StatusOptions};
use structopt::StructOpt;

use std::{
    ffi::OsStr,
    fs, iter,
    path::{Path, PathBuf},
    process::Command,
};

////////////////////////////////////////////////////////////////////////////////

const REMOTE_NAME: &str = "student";

////////////////////////////////////////////////////////////////////////////////

#[derive(StructOpt, Debug)]
#[structopt()]
struct Opts {
    /// Path to the student repository.
    #[structopt(short = "t", long = "task-path")]
    task_path: PathBuf,
}

////////////////////////////////////////////////////////////////////////////////

fn uncommitted_changes(repo: &Repository, task_name: &str) -> Result<Vec<PathBuf>> {
    let statuses = repo
        .statuses(Some(
            &mut StatusOptions::new()
                .include_untracked(false)
                .include_ignored(false),
        ))
        .with_context(|| format!("failed to get git statuses in {:?}", task_name))?;

    let task_prefix = format!("{}/", task_name);

    let mut paths = vec![];
    for status in statuses.iter() {
        let path = PathBuf::from(
            status
                .path()
                .context("'git diff' contains an entry with non-utf8 path")?,
        );
        if path.starts_with(&task_prefix) {
            paths.push(path.into());
        }
    }

    Ok(paths)
}

fn does_remote_exist(repo: &Repository, remote: &str) -> Result<bool> {
    match repo.find_remote(remote) {
        Ok(_) => Ok(true),
        Err(err) => {
            if err.code() == git2::ErrorCode::NotFound {
                Ok(false)
            } else {
                bail!("failed to find remote '{}': {}", remote, err);
            }
        }
    }
}

fn push_task(path: &Path, task_name: &str, remote_name: &str) -> Result<()> {
    // NB: push using git as a subcommand is way less tedious than using libgit2.
    let status = Command::new("git")
        .args(&[
            "push",
            "--force",
            remote_name,
            &format!("HEAD:submit/{}", task_name),
        ])
        .current_dir(path)
        .spawn()
        .context("failed to call 'git'")?
        .wait()
        .context("failed to wait for 'git'")?;
    if !status.success() {
        bail!("'git push' failed");
    }
    Ok(())
}

fn do_main(opts: Opts) -> Result<()> {
    let task_name = fs::canonicalize(&opts.task_path)
        .context("failed to canonicalize path")?
        .file_name()
        .context("failed to get file name")?
        .to_str()
        .context("task name is not a str")?
        .to_owned();

    let repo = Repository::open_ext(
        &opts.task_path,
        RepositoryOpenFlags::empty(),
        iter::empty::<&OsStr>(),
    )
    .context("failed to open git repository")?;

    let uncommitted_files = uncommitted_changes(&repo, &task_name)
        .context("failed to check for uncommitted changes")?;
    if !uncommitted_files.is_empty() {
        bail!(
            "there are uncommitted changes:\n{}\nPlease either commit or stash them.",
            uncommitted_files
                .iter()
                .map(|p| format!(" * {}", p.display()))
                .collect::<Vec<_>>()
                .join("\n"),
        );
    }

    let remote_exists = does_remote_exist(&repo, REMOTE_NAME)?;
    if !remote_exists {
        bail!(
            "remote '{}' does not exist. Please create it according to the course tutorial.",
            REMOTE_NAME
        );
    }

    push_task(&opts.task_path, &task_name, REMOTE_NAME).context("failed to push task")
}

fn main() {
    let args = Opts::from_args();

    if let Err(err) = do_main(args) {
        eprintln!("Error: {:#}", err);
        std::process::exit(1);
    }
}
