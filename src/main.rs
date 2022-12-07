use serde::{Serialize};
use failure::Error;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

#[derive(Serialize, Debug)]
struct Person {
    name: String,
    attestation: u32,
}

fn reading_line(x: &mut String) -> &str{
    io::stdin()
            .read_line(x)
            .expect("Failed to read line");
    let new = x.trim();
    return new;
}

fn main() -> Result<(), Error> {
    let student_Ivanov_I = Person {
        name: "Ivan Ivanov".to_string(),
        attestation: 95,
    };
    serde_any::to_file("student.json", &student_Ivanov_I);
    println!("Please input your old file.");
    let mut basic_file = String::new();
    let first_file = reading_line(&mut basic_file);
    let old_path = Path::new(&first_file);
    let mut old_file = File::open(old_path)?;
    let mut information = String::new();
    old_file.read_to_string(&mut information)?;
    println!("Please input your new file.");
    let mut new_file = String::new();
    let second_file = reading_line(&mut new_file);
    let new_path = Path::new(&second_file);
    serde_any::to_file(new_path, &information);
    Ok(())
}
