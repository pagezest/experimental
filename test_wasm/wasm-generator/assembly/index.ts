import { JSON } from "assemblyscript-json/assembly";

export function add(a: i32, b: i32): i32 {
  return a + b;
}

export function greet(ptr: usize, len: i32): usize {
  const input = String.UTF8.decodeUnsafe(ptr, len);
  const result = "Hello "+input;
  const encoded = String.UTF8.encode(result, true);
  return changetype<usize>(encoded)
}

export function join_name(ptr: usize, len: i32) : usize {
  // Expecting input as 
  // {
  //    "first_name" : "H",
  //    "last_name" : "J"
  // }
  const raw = String.UTF8.decodeUnsafe(ptr,len);
  const value = JSON.parse(raw);
  if(!value.isObj) {
    return 0;
  }
  const obj = value as JSON.Obj;
  let first_name = obj.getString("first_name");
  let last_name = obj.getString("last_name");
  if (first_name == null || last_name ==  null){
    return 0;
  }

  const full_name = first_name.valueOf()+" "+last_name.valueOf();
  const encode = String.UTF8.encode(full_name, true);
  return changetype<usize>(encode);
}
