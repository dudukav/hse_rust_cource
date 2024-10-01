#![forbid(unsafe_code)]

use std::hash::{DefaultHasher, Hash, Hasher};

use crate::types::{Data, Key};

pub fn new_hashmap(len: usize) -> Vec<Vec<(Key, Data)>> {
    let mut hashmap = Vec::with_capacity(len);

    for _ in 0..len {
        hashmap.push(Vec::new());
    }

    hashmap
}

pub fn insert(table: &mut Vec<Vec<(Key, Data)>>, key: Key, value: Data) -> &mut Data {
    if table.len() == 0 {
        panic!("insert in empty kolhoz-table");
    }

    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let bucket_index = (hasher.finish() % table.len() as u64) as usize;

    table[bucket_index].push((key, value));
    
    let result = table[bucket_index].last_mut().unwrap();
    &mut result.1
}

fn hash_and_search<'a>(table: &'a Vec<Vec<(Key, Data)>>, key: &Key) -> Vec<&'a Data> {
    if table.is_empty() {
        return Vec::new();
    }

    let mut hasher = DefaultHasher::new();
    key.hash(&mut hasher);
    let bucket_index = (hasher.finish() % table.len() as u64) as usize;

    let mut result = Vec::new();

    for (existing_key, value) in &table[bucket_index] {
        if existing_key == key {
            result.push(value);
        }
    }

    result
}

pub fn get_one_or_default<'a>(table: &'a Vec<Vec<(Key, Data)>>, key: &Key, default_value: &'a Data) -> &'a Data {
    let values = hash_and_search(table, key);

    if values.is_empty() {
        return default_value;
    }

    values.first().unwrap()
}

pub fn multi_get<'a>(table: &'a Vec<Vec<(Key, Data)>>, keys: &'a Vec<Key>) -> Vec<(&'a Key, Vec<&'a Data>)> {
    let mut result = Vec::new();
    for key in keys {
        let values = hash_and_search(table, key);
        result.push((key, values));
    }

    result
}

pub fn find_keys_of_data<'a>(table: &'a Vec<Vec<(Key, Data)>>, value: &Data) -> Vec<&'a Key> {
    let mut keys = Vec::new();

    for bucket in table {
        for pair in bucket {
            if pair.1 == *value {
                keys.push(&pair.0);
            }
        }
    }

    keys
}

pub fn resize(table: &mut Vec<Vec<(Key, Data)>>, new_len: usize) {
    if new_len == 0 {
        *table = Vec::new();
        return
    }
    if table.len() == 0 {
        for _ in 0..new_len {
            table.push(Vec::new());
        }

        return;
    }

    if new_len == table.len() {
        return;
    }

    let mut new_table: Vec<Vec<(Key, Data)>> = Vec::with_capacity(new_len);
    for _ in 0..new_len {
        new_table.push(Vec::new());
    }

    for bucket in table.iter_mut() {
        for (key, value) in bucket.drain(..) {
            let mut hasher = DefaultHasher::new();
            key.hash(&mut hasher);
            let bucket_index = (hasher.finish() % new_len as u64) as usize;

            new_table[bucket_index].push((key, value));
        }
    }

    *table = new_table;
}