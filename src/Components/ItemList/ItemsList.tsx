import { For } from "solid-js";
import Item from "./Item";
import "../../Styles/ItemsList.css";

export type itemData = {
  name: string;
  value: string;
};

type ItemsListProps = {
  list: itemData[];
};

function ItemList(props: ItemsListProps) {
  return (
    <div class="items-list-container">
      <ul class="items-list-ul">
        <For each={props.list}>
          {(item) => <Item text={item.name} value={item.value} />}
        </For>
      </ul>
    </div>
  );
}

export default ItemList;
