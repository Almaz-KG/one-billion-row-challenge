mod common;

use crate::common::Record;
use std::env;
use std::collections::HashMap;
use std::io::Read;

fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut file = std::fs::File::open(path).unwrap();
        let mut data = String::new();
        file.read_to_string(&mut data).unwrap();
        
        let mut hashmap = HashMap::new();
        for line in data.trim().split('\n') {
            let (name, value) = line.split_once(';').unwrap();
            let value = value.parse::<f32>().unwrap();
            hashmap.entry(name).or_insert(Record::default()).add(value);
        }

        let mut v = hashmap.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|p| p.0);

        for (name, record) in v {
            println!("{}={}/{}/{}", name, record.min, record.average(), record.max);
        }
    } else {
        println!("No file specified");
    }
}