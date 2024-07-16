use git2::{build::RepoBuilder, Cred, FetchOptions, Progress, RemoteCallbacks, Repository};
use indicatif::{ProgressBar, ProgressStyle};
use std::{
    cell::RefCell,
    path::Path,
    rc::Rc,
};

pub fn clone_repo(url: &str, path: &str) -> Result<Repository, git2::Error> {
    let token = "";
    let pb = Rc::new(RefCell::new(ProgressBar::new(100)));
    pb.borrow_mut().set_style(
        ProgressStyle::default_bar()
            .template(
                "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} {msg}",
            )
            .progress_chars("#>-"),
    );

    let pb_clone = pb.clone();

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(move |_url, _username_from_url, _allowed_types| {
        Cred::userpass_plaintext("git", &token)
    });

    callbacks.transfer_progress(move |stats: Progress| {
        let pb = pb_clone.borrow_mut();
        if stats.received_objects() == stats.total_objects() {
            pb.set_message("Resolving deltas...");
        } else if stats.total_objects() > 0 {
            pb.set_message("Receiving objects...");
        } else {
            pb.set_message("Preparing...");
        }

        let progress = if stats.total_objects() > 0 {
            (100 * stats.received_objects() / stats.total_objects()) as u64
        } else {
            0
        };
        pb.set_position(progress);
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(callbacks);

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fetch_options);

    let result = builder.clone(url, Path::new(path));

    pb.borrow_mut().finish_with_message("Done!");

    result
}
