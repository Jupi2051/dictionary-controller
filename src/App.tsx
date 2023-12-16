import { createSignal } from "solid-js";
import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";
import ItemList from "./Components/ItemList";
import { itemData } from "./Components/ItemList/ItemsList";
import { fetchAllWords } from "./IPC/fetchAllWords";

function App() {
  const [wordCount, setWordCount] = createSignal<number>(0);
  const [words, setWordsList] = createSignal<itemData[]>([]);
  const [searchFilter, SetSearchFilter] = createSignal<string>("");

  async function greet() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    const wordCount = (await invoke("fetch_word_count")) as number;
    setWordCount(wordCount);
  }

  async function getItemsList() {
    const words: string[] = await fetchAllWords();
    const mappedWords: itemData[] = words.map((word) => {
      return { name: word, value: word };
    });
    setWordsList(mappedWords);
  }

  function onSearchChange(event: Event) {
    const target = event.target as HTMLInputElement;
    SetSearchFilter(target.value);
  }

  async function onInsertNewWord() {
    console.log(searchFilter());
    const success = (await invoke("insert_word", {
      word: searchFilter(),
    })) as boolean;
    console.log(success);
  }

  return (
    <div class="app-container">
      <ItemList list={words()} />
      <button onclick={greet}>Click me for count</button>
      <button onclick={getItemsList}>Click me to see all words</button>
      <div>{wordCount()}</div>
      <input
        placeholder="new word..."
        type="text"
        value={searchFilter()}
        onInput={onSearchChange}
      />
      <button onclick={onInsertNewWord}>Insert word</button>
    </div>
  );
}

export default App;
