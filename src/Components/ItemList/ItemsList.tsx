import { For } from "solid-js";
import Item from "./Item";
import "../../Styles/ItemsList.css";
import { invoke } from "@tauri-apps/api";

export type itemData = {
  name: string;
  value: string;
};

type ItemsListProps = {
  list: itemData[];
};

function ItemList(props: ItemsListProps) {
  async function DeleteItem(id: string) {
    const success = await invoke("delete_word", { id });
    console.log(success);
  }

  return (
    <div class="items-list-container">
      <ul class="items-list-ul">
        <For each={props.list}>
          {(item) => (
            <Item
              text={item.name}
              value={item.value}
              onClickItem={DeleteItem}
            />
          )}
        </For>
      </ul>
    </div>
  );
}

export default ItemList;
