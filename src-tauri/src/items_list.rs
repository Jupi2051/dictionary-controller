use std::cmp::Ordering;
use std::fs;

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

    pub fn search_for_word(&self, word: &str) -> Option<usize> {
        let words: &Vec<String> = &self.words;

        let mut low: usize = 0;
        let mut high: usize = words.len();

        loop {
            let mid: usize = (high + low) / 2;
            let mid_word: &String = &words[mid];

            if mid_word.eq(word) {
                return Some(mid);
            }

            let compare_result: Ordering = word.cmp(&mid_word);

            if compare_result == Ordering::Greater {
                low = mid;
            } else if compare_result == Ordering::Less {
                high = mid;
            }

            if high <= low {
                break;
            }
        }

        None
    }

    fn does_word_exist(&self, word: &str) -> bool {
        let words: &Vec<String> = &self.words;

        let mut low: usize = 0;
        let mut high: usize = words.len();

        loop {
            let mid: usize = (high + low) / 2;
            let mid_word: &String = &words[mid];

            if mid_word.eq(word) {
                return true;
            }

            let compare_result: Ordering = word.cmp(&mid_word);

            if compare_result == Ordering::Greater {
                low = mid;
            } else if compare_result == Ordering::Less {
                high = mid;
            }

            if (high <= low) || ((high - low) == 1) {
                break;
            }
        }

        false
    }

    pub fn insert_word(&mut self, word: String) -> bool {
        let word_present: bool = self.does_word_exist(&word);

        if word_present == true {
            return false;
        }

        self.words.insert(0, word);

        self.sort_all_words();

        true
    }

    pub fn remove_word(&mut self, word: &str) -> bool {
        let word_found = self.search_for_word(&word);

        let index_value: usize = if let Some(value) = word_found {
            value
        } else {
            usize::MAX
        };

        if index_value.cmp(&usize::MAX) == Ordering::Equal {
            return false;
        }

        self.words.remove(index_value);
        return true;
    }

    fn sort_all_words(&mut self) {
        self.words.sort();
    }

    pub fn get_word_count(&mut self) -> usize {
        return self.words.iter().count();
    }

    pub fn get_all_words(&mut self) -> Vec<String> {
        let copied_vec: Vec<String> = self.words.iter().cloned().collect();
        return copied_vec;
    }
}
