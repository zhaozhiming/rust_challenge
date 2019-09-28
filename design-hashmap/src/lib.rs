const MAX_HASHMAP_SIZE: usize = 1000001;

pub struct MyHashMap {
    pub containers: Vec<i32>,
}

impl MyHashMap {
    pub fn new() -> Self {
        MyHashMap {
            containers: vec!(-1;MAX_HASHMAP_SIZE),
        }
    }

    // 哈希方法：简单实现，将key对最大长度求模
    fn hash(&self, key: usize) -> usize {
        key % MAX_HASHMAP_SIZE 
    }

    pub fn put(&mut self, key: i32, value: i32) {
        let hash_key = self.hash(key as usize);
        self.containers[hash_key] = value; 
    }

    pub fn get(&self, key: i32) -> i32 {
        let hash_key = self.hash(key as usize);
        self.containers[hash_key] 
    }

    pub fn remove(&mut self, key: i32) {
        let hash_key = self.hash(key as usize);
        self.containers[hash_key] = -1; 
    }
}
