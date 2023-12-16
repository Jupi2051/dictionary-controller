// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod items_list;
use items_list::ItemsList;
use std::sync::Mutex;
use tauri::State;

struct Items(Mutex<ItemsList>);

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn fetch_word_count(items: State<Items>) -> usize {
    let mut itm = items.0.lock().unwrap();
    let count = itm.get_word_count();
    return count;
}

#[tauri::command]
fn fetch_all_words(items: State<Items>) -> Vec<String> {
    let mut itm = items.0.lock().unwrap();
    return itm.get_all_words();
}

#[tauri::command]
fn delete_word(id: &str, items: State<Items>) -> bool {
    let mut itm = items.0.lock().unwrap();
    let success: bool = itm.remove_word(&id);
    success
}

fn main() {
    tauri::Builder::default()
        .manage::<Items>(Items(Mutex::new(ItemsList::new(
            "C:/Users/Jupi/Desktop/wbDictionary.json",
        ))))
        .invoke_handler(tauri::generate_handler![
            fetch_word_count,
            fetch_all_words,
            delete_word
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
