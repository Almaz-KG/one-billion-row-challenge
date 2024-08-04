#![feature(slice_split_once)]

use std::env;
use std::fmt::Display;
use std::io::Read;
use fxhash::FxHashMap;

pub struct Record {
    pub min: i32,
    pub max: i32,
    pub sum: i32,
    pub count: u32,
}

impl Record {
    pub fn add(&mut self, value: i32) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.sum += value;
        self.count += 1;
    }

    pub fn average(&self) -> i32 {
        self.sum / self.count as i32
    }
}

impl Default for Record {
    fn default() -> Self {
        Record {
            min: i32::MAX,
            max: i32::MIN,
            sum: 0,
            count: 0,
        }
    }
}

impl Display for Record {
    fn fmt(&self,f: &mut std::fmt::Formatter) -> std::fmt::Result{
        write!(f,"{:.1}/{:.1}/{:.1}", self.min as f64 / 10.0, self.average() as f64 / 10.0, self.max as f64 / 10.0)
    }
}


fn parse(raw_value: &[u8]) -> i32 {
    let is_negative = raw_value[0] == b'-';

    let value = if is_negative { &raw_value[1..] } else { raw_value };
    let (a, b, c, d) = match value {
        [c, b'.', d] => (0, 0, c - b'0', d - b'0'),                         // For numbers like \d{1}.\d
        [b, c, b'.', d] => (0, b - b'0', c - b'0', d - b'0'),               // For numbers like \d{2}.\d
        [a, b, c, b'.', d] => (a - b'0', b - b'0', c - b'0', d - b'0'),     // For numbers like \d{3}.\d
        [c] => (0, 0, 0, c - b'0'),                                         // For numbers like \d{1}
        [b, c] => (0, b - b'0', c - b'0', 0),                               // For numbers like \d{2}
        [a, b, c] => (a - b'0', b - b'0', c - b'0', 0),                     // For numbers like \d{3}
        _ => panic!("Unknown patters {:?}", std::str::from_utf8(value).unwrap()),
    };
    let v = (a as u32) * 1000 + b as u32 * 100 + c as u32 * 10 + d as u32;
    if is_negative {
        -(v as i32)
    } else {
        v as i32
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
            hashmap.entry(name).or_insert(Record::default()).add(parse(value));
        }

        let mut v = hashmap.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|p| p.0);

        for (name, record) in v {
            println!("{}={}", std::str::from_utf8(name).unwrap(), record);
        }
    } else {
        println!("No file specified");
    }
    
}