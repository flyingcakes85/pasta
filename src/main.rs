#![feature(proc_macro_hygiene, decl_macro)]
use rand::{distributions::Alphanumeric, Rng};
use rocket::config::{Config, Environment, Limits};
use rocket::http::RawStr;
use rocket::request::Form;
use std::fs;
use std::path::Path;
mod vars;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct UserInput<'f> {
    value: &'f RawStr,
}

#[post("/", data = "<user_input>")]
fn submit_task(user_input: Form<UserInput>) -> String {
    // format!("Your value: {}", user_input.into_inner().value)

    let mut file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(vars::URL_LENGTH)
        .map(char::from)
        .collect();

    while Path::new(&(vars::PASTE_ROOT.to_owned() + &file_name)).exists() {
        file_name = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(4)
            .map(char::from)
            .collect();
    }

    fs::write(
        &(vars::PASTE_ROOT.to_owned() + &file_name),
        user_input.into_inner().value,
    )
    .unwrap();

    file_name
}

#[get("/<file_name>")]
fn get_paste(file_name: String) -> String {
    if Path::new(&(vars::PASTE_ROOT.to_owned() + &file_name)).exists() {
        fs::read_to_string(&(vars::PASTE_ROOT.to_owned() + &file_name)).unwrap()
    } else {
        String::from("404")
    }
}

fn main() {
    fs::create_dir_all(vars::PASTE_ROOT).unwrap();

    let limits = Limits::new().limit("forms", 5 * 1024 * 1024);

    let config = Config::build(Environment::Staging)
        .workers(2)
        .limits(limits)
        .unwrap();

    rocket::custom(config)
        .mount("/submit", routes![submit_task])
        .mount("/", routes![get_paste])
        .launch();
}
