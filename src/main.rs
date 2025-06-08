mod ai;
mod application;
mod core;
mod interface;
mod state;
mod utils;

use crate::application::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = Application::new().await?;
    app.run().await
}
