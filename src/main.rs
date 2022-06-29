use rocket::{response::Redirect};

#[macro_use] extern crate rocket;

mod utils;

#[get("/")]
fn index() -> &'static str {
    "add https://simplese.herokuapp.com/search?cmd=%s as a search engine in the browser to use it directly from the URL bar"
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

