use std::collections::HashMap;
use std::env;

mod app;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        let task = &args[1].clone();
        let parameters = &args[2..];
        dbg!(&task);
        dbg!(&parameters);
    }

    loop {

        let mut input = String::new();
        std::io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        match input.as_str() {
            "list" => {
                println!("Listing collection...");
            }
            "add" => {
                println!("Adding stickers to collection...");
            }
            "remove" => {
                println!("Removing stickers from collection...");
            }
            "trade" => {
                println!("Doing some trading...");
            }
            "exit" => {
                println!("Exiting...");
                break;
            }
            _ => {
                println!("Doing something else...");
            }
        }

    }
}

pub struct Collection {
    pub name: String,
    pub stickers: HashMap<i32, i32>, 
}

impl Collection {
    pub fn new(&mut self, name: String) -> &Collection {
        self.name = name;

        self.stickers = HashMap::new();
        return self;
    }
}