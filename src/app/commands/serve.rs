use nu_ansi_term::Color::Green;
use futures::StreamExt;
use live_server::listen;
use notify::{
    event::ModifyKind,
    Event, EventKind, RecursiveMode, Watcher,
};
use std::{env, path::Path};

use crate::app::watcher::watcher;
use crate::app::log::display_status;
use crate::app::config::config::Config;
use crate::app::build::{DockiBuildResult, DockiBuilder};

pub async fn serve(config: &Config) {
	let builder = DockiBuilder::new(config);
	let mut server = Server::new(builder, config);
	server.serve().await;
}

struct Server<'a> {
	builder: DockiBuilder<'a>,
	config: &'a Config,
}

impl <'a> Server <'a> {
	fn new(builder: DockiBuilder<'a>, config: &'a Config) -> Self {
		return Self {
			builder,
			config: config
		}
	}

	async fn serve(&mut self) {
		self.builder.prepare().await.expect("could not prepare for build");
		self.builder.build_docs().expect("build failed");
		tokio::join!(self.start_server(), self.watch_and_build());
	}

	async fn start_server(&self) {
		let link = &format!("http://localhost:{}", self.config.port);
		let hyperlink = Green.paint(link).hyperlink(link);

		println!(
			"\nServing at {}",
			hyperlink
		);

		let Ok(()) = listen("localhost", self.config.port, self.config.output_dir.clone()).await else {
			panic!("could not start server")
		};
	}

	async fn watch_and_build(&self) {
		self.watch()
			.await
			.expect("something went wrong")
	}

	async fn watch(&self) -> notify::Result<()> {
		let path = Path::new(&self.config.input_dir);
		let (mut watcher, mut rx) = watcher()?;

		watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

		while let Some(res) = rx.next().await {
			let event = res.expect("watching failed");
			self.file_change(event)
		}

		Ok(())

	}

	fn file_change(&self, event: Event) {
		match event.kind {
			EventKind::Modify(ModifyKind::Data(_)) => self.build_valid_files(event.paths),
			_ => (),
		}
	}

	fn build_valid_files(&self, paths: Vec<std::path::PathBuf>) {
		let invalid_path_message = "changed path is invalid";
		let in_path = paths
			.first()
			.expect(invalid_path_message)
			.strip_prefix(&Self::current_dir())
			.expect(invalid_path_message)
			.to_str()
			.expect(invalid_path_message);

		let in_path = format!("./{}", in_path);
		let result = self.builder.build_file(&in_path);

		match result {
			DockiBuildResult::Slide(out_path) => Self::display_rebuilding_status("Slide", &in_path, &out_path),
			DockiBuildResult::Doc(out_path) => Self::display_rebuilding_status("Doc", &in_path, &out_path),
			DockiBuildResult::Copy(out_path) => Self::display_rebuilding_status("Copy", &in_path, &out_path),
			DockiBuildResult::Err(err) => {
				Self::display_rebuilding_status("Error", &in_path, "");
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
}


