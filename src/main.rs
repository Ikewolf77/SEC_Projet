mod actions;
mod db;

use actions::student_action;
use actions::teacher_action;

use db::save_database_to_file;

fn welcome() {
    println!("Welcome to KING: KING Is Not GAPS");
}

fn menu(teacher: &mut bool) {
    if *teacher {
        teacher_action();
    } else {
        student_action(teacher);
    }
}

fn about() {
    panic!("The requested URL was not found on this server.");
}

fn quit() {
    println!("Saving database!");
    save_database_to_file();
    std::process::exit(0);
}

fn main() {
    welcome();
    let mut teacher = false;
    loop {
        menu(&mut teacher);
    }
}
