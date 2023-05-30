use std::path::PathBuf;

use rocket::{fs::FileServer, response::Redirect};

#[macro_use]
extern crate rocket;

mod utils;

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let cmd = cmd.trim();
    let command = utils::get_command(&cmd);
    let redirect_url = match command {
        "tw" => utils::twitter::twitter_url(&cmd),
        "gh" => utils::github::github_url(&cmd),
        "ttv" => utils::twitch::twitch_url(&cmd),
        "n" => utils::nix::nix_url(&cmd),
        "vm" => utils::view_media::video_media_url(&cmd),
        _ => utils::google::google_search(&cmd),
    };
    Redirect::to(redirect_url)
}

#[shuttle_runtime::main]
async fn rocket(
    #[shuttle_static_folder::StaticFolder(folder = "static")] static_folder: PathBuf,
) -> shuttle_rocket::ShuttleRocket {
    Ok(rocket::build()
        .mount("/", routes![search])
        .mount("/", FileServer::from(static_folder))
        .into())
}
