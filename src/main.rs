extern crate csv;

mod db;
use db::create_account;
use db::enter_grade;
use db::login;
use db::save_database_to_file;
use db::show_grades;
use db::UserDTO;

use read_input::prelude::*;

use log::{error, info, trace, warn};

fn welcome() {
    trace!("Welcome");
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(user: &UserDTO) {
    trace!("Menu");
    println!(
        "*****
        1: See grades
        2: (Teachers): Enter a grade
        3: (Teachers): Create a student
        4: (Admin): Create a teacher
        5: About
        0: Quit"
    );
    let choice = input().inside(0..=5).msg("Enter Your choice: ").get();
    match choice {
        //1 => show_grades(user),
        //2 => enter_grade(user),
        3 => create_account(user, false),
        4 => create_account(user, true),
        5 => about(),
        0 => quit(),
        _ => {
            error!("Impossible choice in menu by {}", user.email);
            panic!("impossible choice")
        }
    }
}

fn about() {
    trace!("About");
    error!("The requested URL was not found on this server.");
    panic!("The requested URL was not found on this server.");
}

fn quit() {
    trace!("Quit");
    println!("Saving database!");
    save_database_to_file();
    std::process::exit(0);
}

fn main() {
    let _ = log4rs::init_file("logConfig.yaml", Default::default()).unwrap();
    welcome();
    let user = login();

    match user {
        Some(u) => {
            info!("Successful login from {}", u.email);
            loop {
                menu(&u);
            }
        }
        None => {
            warn!("Attempted login");
            println!("Wrong username or password")
        }
    }
}
