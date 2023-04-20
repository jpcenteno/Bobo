mod request_logging;

use axum::Router;
use log::{debug, info};
use notify::Watcher;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::path::PathBuf;
use tower_http::services::ServeDir;
use tower_livereload::LiveReloadLayer;

use clap::Parser;

static IP_ADDR: IpAddr = IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0));

/// Simple HTTP server for static files with live reload capabilities.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Arguments {
    /// Path for the directory to serve.
    #[arg(short, long, value_name = "DIRECTORY")]
    directory: PathBuf,

    /// HTTP server port.
    #[arg(short, long, value_name = "PORT", default_value_t = 42069)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    let arguments = Arguments::parse();
    debug!("Initializing server with arguments: {:?}", arguments);

    let live_reload_layer = LiveReloadLayer::new();
    let reloader = live_reload_layer.reloader();

    let app = Router::new()
        .nest_service("/", ServeDir::new(&arguments.directory))
        .layer(request_logging::LogLayer::default())
        .layer(live_reload_layer);

    let mut watcher = notify::recommended_watcher(move |_| reloader.reload())?;
    watcher.watch(&arguments.directory, notify::RecursiveMode::Recursive)?;

    info!(
        "Serving static files from directory: {}",
        &arguments.directory.display()
    );

    let socket_addr = SocketAddr::new(IP_ADDR, arguments.port);
    info!("Listening on {}", socket_addr);

    axum::Server::bind(&socket_addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
