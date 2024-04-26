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

fn to_key(name: &[u8]) -> u64 {
    let mut key = [0u8; 8];
    let l = name.len().min(8);
    key[..l].copy_from_slice(&name[..l]);
    key[0] ^= name.len() as u8;
    u64::from_ne_bytes(key)
}
    

fn parse(raw_value: &[u8]) -> f32 {
    let is_negative = raw_value[0] == b'-';

    let value = if is_negative { &raw_value[1..] } else { raw_value };
    let (a, b, c, d) = match value {
        [c, b'.', d] => (0, 0, c - b'0', d - b'0'),
        [b, c, b'.', d] => (0, b - b'0', c - b'0', d - b'0'),
        [a, b, c, b'.', d] => (a - b'0', b - b'0', c - b'0', d - b'0'),
        [c] => (0, 0, 0, c - b'0'),
        [b, c] => (0, b - b'0', c - b'0', 0),
        [a, b, c] => (a - b'0', b - b'0', c - b'0', 0),
        _ => panic!("Unknown patters {:?}", std::str::from_utf8(value).unwrap()),
    };
    let v = (a as u32) * 1000 + b as u32 * 100 + c as u32 * 10 + d as u32;
    if is_negative {
        -(v as f32)
    } else {
        v as f32
    }
}


fn main() {
    if let Some(path) = env::args().nth(1) {
        let mut data: Vec<u8> = vec![];
        {
            let mut file = std::fs::File::open(&path).unwrap();
            let stat = std::fs::metadata(&path).unwrap();
            data.reserve(stat.len() as usize);
            file.read_to_end(&mut data).unwrap();
            assert!(data.pop() == Some(b'\n'));
        }
        
        let mut hashmap = FxHashMap::default();
        for line in data.split(|&c| c == b'\n') {
            let (name, value) = line.split_once(|&c| c == b';').unwrap();
            hashmap.entry(to_key(name)).or_insert((Record::default(), name)).0.add(parse(value));
        }

        let mut v = hashmap.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|p| p.0);

        for (_k, (record, name)) in v {
            println!("{}={}/{}/{}", std::str::from_utf8(name).unwrap(), record.min, record.average(), record.max);
        }
    } else {
        println!("No file specified");
    }
    
}