mod app;

#[cfg(test)] 
mod test;

use app::App;

fn main() {
    let app = App::new();
    app.start();
}
