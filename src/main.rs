use rocket::{fs::FileServer, response::Redirect, Config};

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

#[rocket::launch]
async fn rocket() -> _ {
    let args = std::env::args().collect::<Vec<String>>();

    let rocket = rocket::build()
        .mount("/", routes![search])
        .mount("/", FileServer::from("static/"));

    match &args.iter().map(String::as_str).collect::<Vec<_>>()[..] {
        [_cmd, "--port", port] => rocket.configure(Config {
            port: port.parse::<u16>().unwrap(),
            ..Default::default()
        }),
        _ => rocket,
    }
}
