use std::{fs::{File, Permissions}, io::Write, os::unix::prelude::PermissionsExt};

use crate::app::fs_util;

const ASCIIDOC_REVEAL_VERSION: &str= "v4.1.0-rc.5";

pub async fn install_reveal() -> () {
    let result = reqwest::get(url()).await
        .expect("Could not download reveal. Make sure you are connected to the internet");

    let binary = result.bytes().await.expect("could not get binary");

    let home_path = home::home_dir().expect("could not find home dir");
    let save_path = format!("{}/.docki/asciidoctor-revealjs", home_path.display());
    let save_dir = format!("{}/.docki", home_path.display());

    fs_util::create_dir_recursive(save_dir.as_str());

    let mut file = File::create(save_path).expect("could not save binary");
    file.set_permissions(Permissions::from_mode(0o770)).expect("could not set permission");
    file.write_all(&binary).expect("could not save binary");
}

fn url() -> String {
    return format!("https://github.com/asciidoctor/asciidoctor-reveal.js/releases/download/{}/asciidoctor-revealjs-linux", ASCIIDOC_REVEAL_VERSION);
}

