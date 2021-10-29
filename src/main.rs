#![feature(proc_macro_hygiene, decl_macro)]
use rand::{distributions::Alphanumeric, Rng};
use rocket::{
    config::{Config, Environment, Limits},
    request::Form,
};
use std::{fs, path::Path};
mod vars;

#[macro_use]
extern crate rocket;

#[derive(FromForm, Debug)]
struct UserInput {
    value: String,
}

fn gen_random_filename() -> String {
    // generate random srtring while we find
    // a string which doesn't already exist
    let mut file_name: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(vars::URL_LENGTH)
        .map(char::from)
        .collect();

    while Path::new(&(vars::PASTE_ROOT.to_owned() + &file_name)).exists() {
        file_name = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(vars::URL_LENGTH)
            .map(char::from)
            .collect();
    }

    file_name
}

#[post("/", data = "<user_input>")]
fn submit_paste(user_input: Form<UserInput>) -> String {
    let file_name: String = gen_random_filename();

    let write_result = fs::write(
        &(vars::PASTE_ROOT.to_owned() + &file_name),
        user_input.into_inner().value,
    );
    match write_result {
        Ok(_) => (vars::PASTE_ROOT.to_owned() + &file_name),
        Err(_) => String::from("Error saving paste"),
    }
}

#[get("/<file_name>")]
fn get_paste(file_name: String) -> String {
    if Path::new(&(vars::PASTE_ROOT.to_owned() + &file_name)).exists() {
        fs::read_to_string(&(vars::PASTE_ROOT.to_owned() + &file_name)).unwrap()
    } else {
        String::from("404 NOT FOUND <br> <a href=\"/\">HOMEPAGE</a>")
    }
}

#[get("/")]
fn homepage() -> String {
    String::from("Homepage")
}

fn main() {
    fs::create_dir_all(vars::PASTE_ROOT).unwrap();

    let limits = Limits::new().limit("forms", vars::MAX_FILE_SIZE);

    let config = Config::build(Environment::Staging)
        .workers(vars::WORKERS)
        .limits(limits)
        .unwrap();

    rocket::custom(config)
        .mount("/", routes![get_paste, submit_paste, homepage])
        .launch();
}
