use futures::executor::block_on;
use lazy_static::{__Deref, lazy_static};
use read_input::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{BufReader, BufWriter};
use std::path::Path;
use std::sync::Mutex;

use log::{error, info, trace, warn};

pub mod big_array;
use big_array::BigArray;

pub mod hash;
use hash::padded_hash;
use hash::verify;

pub mod utils;
use utils::ask_for_email;
use utils::ask_for_pw;

pub mod access_control;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    #[serde(with = "BigArray")]
    pub pw_hash: [u8; 128],
    pub grades: Vec<f32>,
}

pub struct UserDTO {
    pub id: String,
    pub email: String,
}

lazy_static! {
    static ref DATABASE: Mutex<Vec<User>> = {
        let data = read_database_from_file(DATABASE_FILE).unwrap_or(Vec::new());
        Mutex::new(data)
    };
}

const ADMIN_HASH: [u8; 128] = [
    36, 97, 114, 103, 111, 110, 50, 105, 100, 36, 118, 61, 49, 57, 36, 109, 61, 54, 53, 53, 51, 54,
    44, 116, 61, 50, 44, 112, 61, 49, 36, 56, 55, 81, 104, 72, 69, 100, 71, 51, 113, 120, 67, 118,
    82, 105, 56, 65, 115, 110, 85, 43, 65, 36, 78, 118, 68, 68, 53, 83, 89, 79, 109, 78, 118, 68,
    66, 74, 71, 88, 70, 109, 73, 87, 114, 98, 83, 99, 118, 69, 56, 115, 110, 75, 116, 106, 104,
    119, 72, 48, 56, 54, 111, 112, 99, 112, 111, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const DATABASE_FILE: &str = "936DA01F9ABD4d9d80C702AF85C822A8.txt";
const NOT_ALLOWED_MSG: &str = "You are not allowed to do that!";

pub fn login() -> Option<UserDTO> {
    trace!("Login");

    let email = ask_for_email();

    let pw = input::<String>().msg("Please enter your password:\n").get();

    if email.eq("admin") {
        if verify(ADMIN_HASH, &pw) {
            return Some(UserDTO {
                email: String::from("admin"),
                id: String::from("admin"),
            });
        }
    } else {
        let data = DATABASE.lock().unwrap();
        for i in 0..(data.len()) {
            let user = &data[i];
            if user.email == email && verify(user.pw_hash, &pw) {
                return Some(UserDTO {
                    email: String::from(&user.email),
                    id: String::from(&user.id),
                });
            }
        }
    }

    return None;
}

pub fn create_account(user: &UserDTO, is_teacher_account: bool) {
    if is_teacher_account {
        if !block_on(access_control::auth(user, access_control::TEACHER_ACC)) {
            println!("{}", NOT_ALLOWED_MSG);
            return;
        }
    } else {
        if !block_on(access_control::auth(user, access_control::STUDENT_ACC)) {
            println!("{}", NOT_ALLOWED_MSG);
            return;
        }
    }
}

pub fn enter_grade() {
    trace!("Enter_grade");
    println!("What is the email of the student?");
    let email: String = input().get();
    println!("What is the new grade of the student?");
    let grade: f32 = input().add_test(|x| *x >= 0.0 && *x <= 6.0).get();
    let mut data = DATABASE.lock().unwrap();
    for i in 0..(data.len()) {
        let user = &mut data[i];
        if user.email.eq(&email) {
            user.grades.push(grade);
            return;
        }
    }
    println!("No Student found with that email");
    warn!("No Student found with that email")
}

pub fn show_grades() {
    trace!("Show_grades");
    let mut data = DATABASE.lock().unwrap();
    let mut res = "".to_owned();
    for i in 0..(data.len()) {
        let user = &mut data[i];
        if !user.grades.is_empty() {
            res.push_str(&format!(
                "{} : {:?} Mean : {}\n",
                user.email,
                user.grades,
                (user.grades.iter().sum::<f32>()) / ((*user.grades).len() as f32)
            ))
        }
    }
}

pub fn read_database_from_file<P: AsRef<Path>>(path: P) -> Result<Vec<User>, Box<dyn Error>> {
    trace!("Read_database");
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let data = serde_json::from_reader(reader)?;
    Ok(data)
}

pub fn save_database_to_file() {
    trace!("Save_database");
    let file = File::create(DATABASE_FILE).unwrap();
    let writer = BufWriter::new(file);
    serde_json::to_writer(writer, DATABASE.lock().unwrap().deref()).unwrap();
}
