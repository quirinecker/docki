use nu_ansi_term::Color::Green;
use futures::StreamExt;
use live_server::listen;
use notify::{
    event::ModifyKind,
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{env, path::Path};

use crate::app::{ build::{DockiBuildResult, docki_build}, commands::build::build, config::config::Config, log::display_status, watcher::watcher};


pub async fn serve(config: &Config) {
    build(config).await;
    tokio::join!(watch_and_build(&config.input_dir), start_server(config.port));
}

async fn watch_and_build(docs_dir: &str) {
    watch(Path::new(docs_dir), docs_dir)
        .await
        .expect("something went wrong")
}


async fn start_server(port: u16) {
	let link = &format!("http://localhost:{}", port);
	let hyperlink = Green.paint(link).hyperlink(link);

    println!(
        "\nServing at {}",
        hyperlink
    );

    let Ok(()) = listen("localhost", port, "./dist").await else {
        panic!("could not start server")
    };
}

async fn watch(path: &Path, docs_dir: &str) -> notify::Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.next().await {
        let event = res.expect("watching failed");
        file_change(event, docs_dir)
    }

    Ok(())
}

fn file_change(event: Event, docs_dir: &str) {
    match event.kind {
        EventKind::Modify(ModifyKind::Data(_)) => build_file(event.paths, docs_dir),
        _ => (),
    }
}

fn build_file(paths: Vec<std::path::PathBuf>, docs_dir: &str) {
    let invalid_path_message = "changed path is invalid";
    let in_path = paths
        .first()
        .expect(invalid_path_message)
		.strip_prefix(&current_dir())
		.expect(invalid_path_message)
        .to_str()
        .expect(invalid_path_message);

	let in_path = format!("./{}", in_path);
    let result = docki_build(&in_path, false, docs_dir);

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

