#![feature(slice_split_once)]
use std::fmt::Display;
use std::env;
use fxhash::FxHashMap;
use memmap2::Mmap;
use memchr::arch::aarch64::neon::memchr::One;

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
        let file = std::fs::File::open(&path).unwrap();
        let mmap: Mmap = unsafe { Mmap::map(&file).unwrap() };
        let data: &[u8] = &*mmap;
        
        let mut hashmap = FxHashMap::default();
        let mut data = &data[..];
        let sep = One::new(b';').unwrap();
        let newline = One::new(b'\n').unwrap();
        unsafe {
            while !data.is_empty() {
                let separator = sep.find(data).unwrap();
                let end = newline.find(data.get_unchecked(separator..)).unwrap();
                let name = data.get_unchecked(..separator);
                let value = data.get_unchecked(separator + 1..separator + end);
                hashmap.entry(name).or_insert(Record::default()).add(parse(value));
                data = data.get_unchecked(separator + end + 1..);
            }
        }

        let mut v = hashmap.into_iter().collect::<Vec<_>>();
        v.sort_unstable_by_key(|p| p.0);
        for (name, record) in &v {
            println!("{}: {}",std::str::from_utf8(name).unwrap(), record);
        }
    } else {
        println!("No file specified");
    }
}
