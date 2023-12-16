type listItemProp = {
  text: string;
  value: string;
};

function Item(props: listItemProp) {
  return <li>{props.text}</li>;
}

export default Item;
