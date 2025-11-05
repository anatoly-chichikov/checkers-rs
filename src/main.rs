mod ai;
mod application;
mod core;
mod interface;
mod state;
mod utils;

use crate::application::Application;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Load environment variables from .env file ONCE at startup
    // This must be done before any async tasks that might use env vars
    dotenv::dotenv().ok();

    let app = Application::new().await?;
    app.run().await
}
