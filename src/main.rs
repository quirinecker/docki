mod app;

#[cfg(test)] 
mod test;

use app::App;

#[tokio::main]
async fn main() {
    let app = App::new();
    app.start().await;
}
