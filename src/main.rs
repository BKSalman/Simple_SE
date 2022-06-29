use rocket::{response::Redirect};

#[macro_use] extern crate rocket;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/search?<cmd>")]
fn search(cmd: String) -> Redirect {
    let command = utils::get_command(&cmd);
    let redirect_url = match command {
        "tw" => {utils::twitter::twitter_url(&cmd)}
        "gh" => {utils::github::github_url(&cmd)}
        _ => {utils::google::google_search(&cmd)}
    };
    Redirect::to(redirect_url)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, search])
}

