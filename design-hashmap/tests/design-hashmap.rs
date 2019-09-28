use design_hashmap::MyHashMap;

#[test]
fn test_design_hashmap_put() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    assert_eq!(1, obj.get(1));
}

#[test]
fn test_design_hashmap_get() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    assert_eq!(1, obj.get(1));
    assert_eq!(-1, obj.get(2));
}

#[test]
fn test_design_hashmap_remove() {
    let mut obj = MyHashMap::new();
    obj.put(1, 1);
    obj.put(2, 1);
    assert_eq!(1, obj.get(1));
    obj.remove(1);
    assert_eq!(-1, obj.get(1));
}

#[test]
fn test_design_hashmap_fail() {
    let mut obj = MyHashMap::new();
    obj.put(21690, 43370);
    assert_eq!(43370, obj.get(21690));
    obj.put(61690, 3642);
    assert_eq!(3642, obj.get(61690));
}
