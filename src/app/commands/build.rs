use crate::app::config::config::Config;

use crate::app::build::DockiBuilder;

pub async fn build(config: &Config) -> () {
    let mut builder = DockiBuilder::new(config);

	builder.prepare().await.expect("could not prepare for build");
    builder.build_docs().expect("build failed")
}
