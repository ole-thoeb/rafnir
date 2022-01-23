use crate::{Key, PolyMap};

const INT_KEY: Key<i32> = Key::new("i32");
const STRING_KEY: Key<String> = Key::new("String");

#[test]
fn basic_insert_and_retrieve() {
    let mut map = PolyMap::new();
    map.insert(INT_KEY, 1);
    let hello_world = String::from("Hello, World!");
    map.insert(STRING_KEY, hello_world.clone());

    assert_eq!(map.get(&INT_KEY).unwrap(), &1);
    assert_eq!(map.get(&STRING_KEY).unwrap(), &hello_world);
}