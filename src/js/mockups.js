function markItemAsSelected(listId, itemClass) {
  var itemsList        = document.getElementById(listId);
  var selectedItemList = itemsList.getElementsByClassName(itemClass);

  for (var i = 0, j = selectedItemList.length; i < j; i++) {
    selectedItemList[i].classList.add('selected');
  }
}
