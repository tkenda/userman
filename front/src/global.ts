import { Value, Item } from "../entities";

export const clone = (obj: any) => {
  if (null == obj || "object" != typeof obj) return obj;
  var copy = obj.constructor();
  for (var attr in obj) {
    if (obj.hasOwnProperty(attr)) copy[attr] = obj[attr];
  }
  return copy;
};

const clearValues = (values: Value[] | undefined) => {
  if (values) {
    for (let i = 0; i < values.length; i++) {
      if (values[i].boolean) {
        values[i].boolean = false;
      } else if (values[i].float) {
        values[i].float = 0.0;
      } else if (values[i].number) {
        values[i].number = 0;
      } else if (values[i].string) {
        values[i].string = "";
      }
    }
  }
};

export const clearItems = (items: Item[] | undefined) => {
  if (items) {
    for (let i = 0; i < items.length; i++) {
      clearValues(items[i].values);
      clearItems(items[i].items);
    }
  }
};

