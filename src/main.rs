mod app;

#[cfg(test)] 
mod test;

use std::env;

use app::App;

fn main() {
    let app = App::new();
    let args = env::args().collect();
    app.start(args);
}
