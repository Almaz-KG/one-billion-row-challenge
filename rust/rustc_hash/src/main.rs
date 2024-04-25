#![feature(slice_split_once)]

use std::env;
use std::io::Read;
use fxhash::FxHashMap;

pub struct Record {
    pub min: f32,
    pub max: f32,
    pub sum: f32,
    pub count: u64,
}

impl Record {
    pub fn add(&mut self, value: f32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value;
        self.count += 1;
    }

    pub fn average(&self) -> f32 {
        self.sum / self.count as f32
    }
}

impl Default for Record {
    fn default() -> Self {
        Record {
            min: f32::MAX,
            max: f32::MIN,
            sum: 0.0,
            count: 0,
        }
    }
}


fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut data: Vec<u8> = vec![];
        {
            let mut file = std::fs::File::open(path).unwrap();
            file.read_to_end(&mut data).unwrap();
            assert!(data.pop() == Some(b'\n'));
        }
        
        let mut hashmap = FxHashMap::default();
        for line in data.split(|&c| c == b'\n') {
            let (name, value) = line.split_once(|&c| c == b';').unwrap();
            let value = unsafe { std::str::from_utf8_unchecked(value) }
                .parse::<f32>()
                .unwrap();
            hashmap.entry(name).or_insert(Record::default()).add(value);
        }

        let mut v = hashmap.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|p| p.0);

        for (name, record) in v {
            println!("{}={}/{}/{}", std::str::from_utf8(name).unwrap(), record.min, record.average(), record.max);
        }
    } else {
        println!("No file specified");
    }
    
}