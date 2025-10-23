use std::process;

fn exec_command(command: &mut process::Command) -> Result<(), String> {
    let result = command.output();

    if let Ok(success) = result {
        if success.stderr.len() == 0 {
            return Ok(());
        } else {
            return Err(from_utf8(success.stderr));
        }
    } else {
        println!("{}", result.unwrap_err());
        return Err(
            "asciidoctor not installed. For more information run docki health!".to_string(),
        );
    }
}

fn asciidoctor_docs(in_path: &str, out_path: &str) -> process::Command {
    let mut command = process::Command::new(format!("asciidoctor"));

    command
        .arg(format!("--out-file={out_path}"))
        .arg(format!("{in_path}"));

    return command;
}

fn asciidoctor_slides(in_path: &str, out_path: &str) -> process::Command {
    let mut command = process::Command::new(format!("asciidoctor-revealjs"));
    let revealjs_path = "/slides/revealjs";

    command
        .arg(format!("{in_path}"))
        .arg(format!("-a"))
        .arg(format!("revealjsdir={revealjs_path}"))
        .arg(format!("--out-file={out_path}"));

    return command;
}

pub fn build_doc(in_path: &str, out_path: &str) -> Result<(), String> {
    let mut command = asciidoctor_docs(in_path, out_path);
    return exec_command(&mut command);
}

pub fn build_slide(in_path: &str, out_path: &str) -> Result<(), String> {
    let mut command = asciidoctor_slides(in_path, out_path);
    return exec_command(&mut command);
}

fn from_utf8(input: Vec<u8>) -> String {
    return match String::from_utf8(input) {
        Ok(m) => m,
        Err(e) => panic!("could not print error message: {}", e),
    };
}
