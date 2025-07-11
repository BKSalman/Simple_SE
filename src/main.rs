use std::net::{IpAddr, Ipv4Addr, SocketAddr};

use axum::{
    Router,
    extract::Query,
    response::Redirect,
    routing::{get, get_service},
};
use serde::{Deserialize, Serialize};
use tower_http::services::ServeDir;

mod utils;

#[derive(Deserialize, Serialize)]
struct QueryParams {
    cmd: String,
}

async fn search(Query(query): Query<QueryParams>) -> Redirect {
    let cmd = query.cmd.trim();
    let command = utils::get_command(cmd);

    let redirect_url = match command {
        "tw" => utils::twitter::twitter_url(cmd),
        "gh" => utils::github::github_url(cmd),
        "ttv" => utils::twitch::twitch_url(cmd),
        "n" => utils::nix::nix_url(cmd),
        "vm" => utils::view_media::video_media_url(cmd),
        _ => utils::google::google_search(cmd),
    };

    Redirect::to(&redirect_url)
}

#[tokio::main]
async fn main() {
    let args = std::env::args().collect::<Vec<String>>();
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/search", get(search))
        .fallback_service(get_service(ServeDir::new("static/")));

    let socket_addr = match &args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [_cmd, "--port", port] => SocketAddr::new(
            IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)),
            port.parse().unwrap(),
        ),
        [_cmd, "--address", address, "--port", port] => {
            SocketAddr::new(address.parse().unwrap(), port.parse().unwrap())
        }
        _ => SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 4433),
    };

    let listener = tokio::net::TcpListener::bind(socket_addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
