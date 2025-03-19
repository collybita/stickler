use std::collections::hash_map;
use std::collections::HashMap;
use std::env;
use std::collections;

fn main() {
    let args: Vec<String> = env::args().collect();

    let task = &args[1].clone();
    let parameters = &args[2..];

    match task.as_str() {
        "new" => {
            println!("creating new collection...");
        }
        _ => {
            println!("doing nothing...");
        }
    }

    dbg!(&task);
    dbg!(&parameters);
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