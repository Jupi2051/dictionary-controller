import "../../Styles/Item.css";

type listItemProp = {
  text: string;
  value: string;
  onClickItem?: (id: string) => void;
};

function Item(props: listItemProp) {
  function onItemClick() {
    if (props.onClickItem) props.onClickItem(props.value);
  }

  return (
    <li class="list-item-clickable" onclick={onItemClick}>
      {props.text}
    </li>
  );
}

export default Item;
