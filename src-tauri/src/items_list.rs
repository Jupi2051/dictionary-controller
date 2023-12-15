use std::fs;

use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json::{Result, Value};

pub struct ItemsList {
    words: Vec<String>,
}

impl ItemsList {
    pub fn new(file: &str) -> ItemsList {
        let content: String = fs::read_to_string(&file).expect("Failed to load this file.");
        let list_of_words: Vec<String> = serde_json::from_str(&content).unwrap();

        ItemsList {
            words: if content.eq("Failed to load this file.") {
                Vec::new()
            } else {
                list_of_words
            },
        }
    }

    pub fn search_for_word(&self) {
        // TODO: binary search into the list to find a certain word
    }

    pub fn insert_word(&mut self, word: String) {
        self.words.insert(0, word);
        // TODO: insert a word into the list and re-order
        self.sort_all_words();
    }

    pub fn remove_word(&mut self, word: &str) {
        // TODO: remove word from list and ensure order
        self.sort_all_words();
    }

    fn sort_all_words(&mut self) {
        self.words.sort();
    }

    pub fn get_word_count(&mut self) -> usize {
        return self.words.iter().count();
    }

    pub fn get_all_words(&mut self) -> Vec<String> {
        let copied_vec: Vec<String> = self.words.iter().cloned().collect();
        // Return the copied vector
        return copied_vec;
    }
}
