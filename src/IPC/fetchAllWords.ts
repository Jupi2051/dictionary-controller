import { invoke } from "@tauri-apps/api/tauri";

export async function fetchAllWords() : Promise<string[]> {
    return await invoke("fetch_all_words") as string[]
}