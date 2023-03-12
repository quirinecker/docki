use colored::Colorize;
use live_server::listen;
use notify::{
    event::{self, ModifyKind},
    Config, Event, EventKind, RecommendedWatcher, RecursiveMode, Watcher,
};
use std::{env, path::Path};
use tokio::sync::mpsc::{channel, Receiver};

use crate::app::{
    builder::{
        asciidoctor::{AsciiDoctorDocsBuilder, AsciiDoctorSlideBuilder},
        Builder,
    },
    fs_util,
};

pub async fn serve() {
    tokio::join!(watch_and_build(), start_server());

    println!("stopping live server");
}

async fn watch_and_build() {
    watch(Path::new("./docs"))
        .await
        .expect("something went wrong")
}

async fn start_server() {
    println!("Serving at http://localhost:8080");

    let Ok(()) = listen("localhost", 8080, "./dist").await else {
        panic!("could not start server")
    };
}

async fn watch(path: &Path) -> notify::Result<()> {
    let (mut watcher, mut rx) = watcher()?;

    watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

    while let Some(res) = rx.recv().await {
        match res {
            Ok(event) => file_change(event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn file_change(event: Event) {
    match event.kind {
        EventKind::Modify(ModifyKind::Data(_)) => {
            build_file(event.paths).expect("building file failed");
            ()
        }
        _ => (),
    }
}

fn build_file(paths: Vec<std::path::PathBuf>) -> Result<(), String> {
    let invalid_path_message = "changed path is invalid";
    let in_path = paths
        .first()
        .expect(invalid_path_message)
        .to_str()
        .expect(invalid_path_message)
        .replace(&current_dir(), "")
        .replace("/./", "./");
    let out_path = in_path
        .replace("./docs/", "./dist/")
        .replace(".adoc", ".html");

    println!("{} {}", "[Rebuilding]".green(), in_path);

    if in_path.starts_with("./docs/slides") {
        let slide_builder = AsciiDoctorSlideBuilder {};
        slide_builder.build(&in_path, &out_path)
    } else {
        let doc_builder = AsciiDoctorDocsBuilder {};
        doc_builder.build(&in_path, &out_path)
    }
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

fn watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (tx, rx) = channel(1);

    let watcher = RecommendedWatcher::new(
        move |res| {
            futures::executor::block_on(async {
                tx.send(res).await.unwrap();
            });
        },
        Config::default(),
    )?;

    Ok((watcher, rx))
}
