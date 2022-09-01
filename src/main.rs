use rocket::{response::Redirect, fs::FileServer};

#[macro_use] extern crate rocket;

mod utils;

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command(&cmd.trim_end());
    let redirect_url = match command {
        "tw" => {utils::twitter::twitter_url(&cmd.trim_end())}
        "gh" => {utils::github::github_url(&cmd.trim_end())}
        "ttv" => {utils::twitch::twitch_url(&cmd.trim_end())}
        _ => {utils::google::google_search(&cmd.trim_end())}
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![search])
    .mount("/", FileServer::from("static"))
}

