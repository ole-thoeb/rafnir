use crate::{PolyMap};
use crate::key::NormalKey;

const INT_KEY: &NormalKey<i32> = &NormalKey::new(6666666661);
const STRING_KEY: &NormalKey<String> = &NormalKey::new(6666666662);

#[test]
fn basic_insert_and_retrieve() {
    let mut map = PolyMap::new();
    map.insert(INT_KEY, 1);
    let hello_world = String::from("Hello, World!");
    map.insert(STRING_KEY, hello_world.clone());

    assert_eq!(map.get(INT_KEY).unwrap(), &1);
    assert_eq!(map.get(STRING_KEY).unwrap(), &hello_world);
}