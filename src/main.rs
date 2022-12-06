use rocket::{response::Redirect, fs::FileServer};

#[macro_use] extern crate rocket;

mod utils;

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let cmd = cmd.trim_end();
    let command = utils::get_command(&cmd);
    let redirect_url = match command {
        "tw" => {utils::twitter::twitter_url(&cmd)}
        "gh" => {utils::github::github_url(&cmd)}
        "ttv" => {utils::twitch::twitch_url(&cmd)}
        "vm" => {utils::view_media::video_media_url(&cmd)}
        _ => {utils::google::google_search(&cmd)}
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
    .mount("/", FileServer::from("static"))
}

