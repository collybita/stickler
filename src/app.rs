use std::collections::HashMap;

pub enum CurrentScreen {
    Main,
    Listing,
    Adding,
    Removing,
    Trading,
    TradeHandling,
    Exiting
}

pub struct App {
    pub number_input: String,              // the current number the user is typing
    pub collection_map: HashMap<u8, u8>,
    pub current_screen: CurrentScreen, // the current screen the user is looking at, and will later determine what is rendered.
}

impl App {
    pub fn new() -> App {
        App {
            number_input: String::new(),
            collection_map: HashMap::new(),
            current_screen: CurrentScreen::Main,
        }
    }

    pub fn add_sticker(&mut self) {
        let num_to_add: Result<u8, _> = self.number_input.parse();

        match num_to_add {
            Ok(n) => {
                let count = self.collection_map.entry(n).or_insert(1);
                *count += 1;
            },
            Err(e) => eprintln!("Failed to parse: {}", e),
        }

        self.number_input = String::new();
    }

    pub fn remove_sticker(&mut self) {
        let num_to_remove: Result<u8, _> = self.number_input.parse();

        match num_to_remove {
            Ok(n) => {
                if let Some(count) = self.collection_map.get_mut(&n) {
                    if *count >= 1 {
                        *count -= 1;
                    }
                    else {
                        self.collection_map.remove(&n);
                    }
                }
                else {
                    eprintln!("Sticker {} not found in collection", n);
                }
            },
            Err(e) => eprintln!("Failed to parse: {}", e),
        }

        self.number_input = String::new();
    }
}