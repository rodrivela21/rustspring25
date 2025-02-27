use std::io::{self, Write};
use std::fs::File;
use std::io::prelude::*;

fn reading_from_console() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("How old are you? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let age: u32 = buffer.trim().parse().unwrap();

    println!("Hi {}, you are {} years old!", name, age);
}

fn write_to_file(name: &str, student_id: u32) {
    let mut file = File::create("config.txt").unwrap();
    writeln!(file, "{}", name).unwrap();
    writeln!(file, "{}", student_id).unwrap();
}

struct Config {
    name: String,
    student_id: u32,
}

impl Config {
    fn from_file(path: &str) -> Config {
        let mut file = File::open(path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        let mut lines = contents.lines();
        let name = lines.next().unwrap().to_string();
        let student_id = lines.next().unwrap().parse().unwrap();

        Config { name, student_id }
    }
}

fn reading_from_file() {
    let config = Config::from_file("config.txt");
    println!("Name: {}", config.name);
    println!("Student ID: {}", config.student_id);
}

fn main() {
    let name = "Rodrigo Velazquez";
    let student_id = 20557647;

    write_to_file(name, student_id);
    reading_from_console();
    reading_from_file();
}