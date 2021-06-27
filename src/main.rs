mod access_control;
mod actions;
mod db;

extern crate strum;
#[macro_use]
extern crate strum_macros;

use actions::student_action;
use actions::teacher_action;

use db::save_database_to_file;

use log::{error, info, trace, warn};

fn welcome() {
    trace!("Welcome");
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(teacher: &mut bool) {
    trace!("Menu");
    if *teacher {
        teacher_action();
    } else {
        student_action(teacher);
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
    let mut teacher = false;
    loop {
        menu(&mut teacher);
    }
}
