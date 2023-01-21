pub mod asciidoctor;

pub trait Builder {
    fn build(&self, in_path: &str, out_path: &str) -> Result<(), String>;
}
