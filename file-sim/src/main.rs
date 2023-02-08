#![allow(unused_variables)]

use std::fmt::{Display};

use rand::prelude::*;

fn one_in(denominator: u32) -> bool {
    thread_rng().gen_ratio(1, denominator)
}


#[derive(Debug, PartialEq)]
pub enum FileState {
    Open,
    Closed
}

impl Display for FileState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            FileState::Open => write!(f, "OPEN"),
            FileState::Closed => write!(f, "CLOSED"),
        }
    }
}

#[derive(Debug)]
struct File {
    pub name: String,
    data: Vec<u8>,
    pub state: FileState
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<{} ({})>", self.name, self.state)
    }
}

impl File {
    pub fn new(name: &str) -> File {
        File {
            name: String::from(name),
            data: Vec::new(),
            state: FileState::Closed
        }
    }

    pub fn new_with_data(name: &str, data: &Vec<u8>) -> File {
        let mut file = File::new(name);
        file.data = data.clone();
        file
    }

    pub fn read(self: &File, save_to: &mut Vec<u8>) -> Result<usize, String> {
        if self.state != FileState::Open {
            return Err(String::from("File must be open for reading"));
        }

        let mut tmp = self.data.clone();
        let read_lenght = tmp.len();
        
        save_to.reserve(read_lenght);
        save_to.append(&mut tmp);
    
        Ok(read_lenght)
    }
}

fn open(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Permission denied");
        return Err(err_msg)
    }
    f.state = FileState::Open;
    Ok(f)
}

fn close(mut f: File) -> Result<File, String> {
    if one_in(10_000) {
        let err_msg = String::from("Interrupted by signal!");
        return Err(err_msg)
    }
    f.state = FileState::Closed;
    Ok(f)
}

fn main() {
    let data: Vec<u8> = vec![114, 117, 115, 116, 33];
    let mut f1 = File::new_with_data("data.txt", &data);
    let mut buffer: Vec<u8> = vec![];

    if f1.read(&mut buffer).is_err() {
        println!("Error checking is working")
    }

    f1 = open(f1).unwrap();
    let f1_length = f1.read(&mut buffer).unwrap();
    f1 = close(f1).unwrap();

    let text = String::from_utf8_lossy(&buffer);
    
    println!("{:?}", f1);
    println!("{}", f1);
    println!("{} is {} bytes long", &f1.name, f1_length);
    print!("{}", text);
}
