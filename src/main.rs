use rofi;
use std::fs::File;
use std::io::{ErrorKind, Read, Write};
use std::{fs, process};

const FILENAME: &str = "~/.rust_todolist";

struct TodoList {
    filename: String,
    list: Vec<String>,
}

impl TodoList {
    // Get a filename and return the words in a vector
    fn file_to_vec(self: &Self, file_name: &String) -> Result<Vec<String>, ()> {
        // Creation of the file descriptor
        let mut file_descriptor: File = match File::open(&file_name) {
            Ok(file) => file,
            Err(ref error) if error.kind() == ErrorKind::NotFound => {
                {
                    let mut file = File::create(&self.filename).unwrap();
                    file.write("".as_bytes()).unwrap();
                }
                File::open(&file_name).unwrap()
            }
            Err(ref error) if error.kind() == ErrorKind::PermissionDenied => {
                println!("You are not authorize to read the file : \"{}\"", file_name);
                std::process::exit(1);
            }
            Err(e) => {
                println!("Unknown error: {}", e);
                std::process::exit(1);
            }
        };

        // Read the file content and transform to a vector of lines
        let mut file_content: String = String::new();
        match file_descriptor.read_to_string(&mut file_content) {
            Err(_) => {
                println!("Could not read the file : \"{}\"", file_name);
                std::process::exit(1);
            }
            Ok(_) => (),
        }

        let file_hashmap: Vec<String> = file_content.split("\n").map(str::to_string).collect();

        Ok(file_hashmap)
    }

    fn vec_to_file(self: &Self) {
        let list = match self.list.len() {
            0 => String::from(""),
            1 => self.list.join(""),
            _ => self.list.join("\n"),
        };

        fs::write(&self.filename, list).expect("Unable to write file");
    }

    fn new(filename: String) -> TodoList {
        let filename_to_struct = shellexpand::tilde(&filename).to_string();
        let mut todolist = TodoList {
            filename: filename_to_struct,
            list: vec![],
        };
        todolist.list = todolist.file_to_vec(&todolist.filename).unwrap();
        todolist
    }

    fn add_element(self: &mut Self, element: &str) {
        self.list.push(element.to_string());
        self.vec_to_file();
    }

    fn rm_element(self: &mut Self, element: &str) {
        self.list
            .remove(self.list.iter().position(|x| *x == element).unwrap());
        self.vec_to_file();
    }
}

fn main() {
    let mut todolist = TodoList::new(FILENAME.to_string());

    let choice: String = match rofi::Rofi::new(&todolist.list).run() {
        Ok(choice) => choice,
        // Err(rofi::Error::Interrupted) => process::exit(1),
        Err(_e) => process::exit(1),
    };

    match choice.chars().next() {
        Some('+') => {
            let mut to_add = choice.chars();
            to_add.next();
            todolist.add_element(to_add.as_str().trim());
        }
        Some(_) => {
            match todolist.list.contains(&choice.trim().to_string()) {
                true => todolist.rm_element(choice.trim()),
                false => (),
            };
        }
        None => (),
    };
}
