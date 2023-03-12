use colored::Colorize;
use futures::StreamExt;
use live_server::listen;
use notify::{
    event::ModifyKind,
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{env, path::Path};

use crate::app::{ watcher::watcher, build::{docki_build, DockiBuildResult}, commands::build::build, log::display_status};


pub async fn serve() {
    build().await;
    tokio::join!(watch_and_build(), start_server());
}

async fn watch_and_build() {
    watch(Path::new("./docs"))
        .await
        .expect("something went wrong")
}

async fn start_server() {
    println!("\nServing at {} ", "http://localhost:8080".bold());

    let Ok(()) = listen("localhost", 8080, "./dist").await else {
        panic!("could not start server")
    };
}

async fn watch(path: &Path) -> notify::Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        let event = res.expect("watching failed");
        file_change(event)
    }

    Ok(())
}

fn file_change(event: Event) {
    match event.kind {
        EventKind::Modify(ModifyKind::Data(_)) => build_file(event.paths),
        _ => (),
    }
}

fn build_file(paths: Vec<std::path::PathBuf>) {
    let invalid_path_message = "changed path is invalid";
    let in_path = paths
        .first()
        .expect(invalid_path_message)
        .to_str()
        .expect(invalid_path_message)
        .replace(&current_dir(), "")
        .replace("/./", "./");


    let result = docki_build(&in_path);

    match result {
        DockiBuildResult::Slide(out_path) => display_rebuilding_status("Slide", &in_path, &out_path),
        DockiBuildResult::Doc(out_path) => display_rebuilding_status("Doc", &in_path, &out_path),
        DockiBuildResult::Copy(out_path) => display_rebuilding_status("Copy", &in_path, &out_path),
        DockiBuildResult::Err(err) => {
            display_rebuilding_status("Error", &in_path, "");
            println!("{}", err);
        },
    }
}

fn display_rebuilding_status(context: &str, in_path: &str, out_path: &str) {
    display_status("Rebuildng", context, in_path, out_path)
}

fn current_dir() -> String {
    let err_message = "something went wrong";
    return String::from(
        env::current_dir()
            .expect(err_message)
            .to_str()
            .expect(err_message),
    );
}

