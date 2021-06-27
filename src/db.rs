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

const DATABASE_FILE: &str = "936DA01F9ABD4d9d80C702AF85C822A8.txt";

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: String,
    pub email: String,
    pub name: String,
    #[serde(with = "BigArray")]
    pub pw_hash: [u8; 128],
    pub grades: Vec<f32>,
}

lazy_static! {
    static ref DATABASE: Mutex<Vec<User>> = {
        let data = read_database_from_file(DATABASE_FILE).unwrap_or(Vec::new());
        Mutex::new(data)
    };
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
            break;
        }
        if i == data.len() - 1 {
            println!("No Student found with that email");
            warn!("No Student found with that email")
        }
    }
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
