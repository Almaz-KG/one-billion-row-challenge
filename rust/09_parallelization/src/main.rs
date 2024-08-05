#![feature(slice_split_once, portable_simd, slice_as_chunks, split_array)]

use std::fmt::Display;
use std::thread::JoinHandle;
use std::{env, fs};
use crossbeam_channel::{Receiver, Sender};
use fxhash::FxHashMap;
use std::io::Read;
use memchr::memchr;

const READ_BUF_SIZE: usize=128 * 1024;

#[derive(Clone)]
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

    pub fn merge(&mut self, record: Record) {
        self.min = self.min.min(record.min);
        self.max = self.max.max(record.max);

        self.sum = self.sum + record.sum;
        self.count = self.count + record.count;
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


fn process_chunk(data: &[u8], result: &mut FxHashMap<Box<[u8]>, Record>) -> () {
    let mut buffer = &data[..];

    loop {
        match memchr(b';', &buffer) {
            None => {
                break;
            }
            Some(comma_seperator) => {
                let end = memchr(b'\n', &buffer[comma_seperator..]).unwrap();
                let name = &buffer[..comma_seperator];
                let value = &buffer[comma_seperator + 1..comma_seperator + end];
                // let value: f32 = unsafe { std::str::from_utf8_unchecked(value) }.parse::<f32>().unwrap();

                result.entry(name.into()).or_insert(Record::default()).add(parse(value));
                // result.entry(name.into()).or_insert(Record::default()).merge(record);
                buffer = &buffer[comma_seperator + end + 1..];
            }
        }
    }
}

fn find_new_line_pos(bytes: &[u8]) -> Option<usize> {
    // In this case (position is not far enough),
    // naive version is faster than bstr (memchr)
    bytes.iter().rposition(|&b| b == b'\n')
}

fn start_processing_threads(receiver: &Receiver<Box<[u8]>>) -> Vec<JoinHandle<FxHashMap<Box<[u8]>, Record>>> {
    let n_threads: usize = std::thread::available_parallelism().unwrap().into();

    let mut handles = Vec::with_capacity(n_threads);
    for _ in 0..n_threads {
        let receiver = receiver.clone();
        let handle = std::thread::spawn(move || {
            let mut result = FxHashMap::<Box<[u8]>, Record>::default();
            // wait until the sender sends the chunk
            for buf in receiver {
                process_chunk(&buf, &mut result);
            }
            result
        });
        handles.push(handle);
    }
    handles
}

fn chunk_and_send_data(sender: &Sender<Box<[u8]>>, mut file: fs::File) -> () {
    let mut buf = vec![0; READ_BUF_SIZE];
    let mut bytes_not_processed = 0;
    loop {
        let bytes_read = file.read(&mut buf[bytes_not_processed..]).expect("Failed to read file");
        if bytes_read == 0 {
            break;
        }

        let actual_buf = &mut buf[..bytes_not_processed+bytes_read];
        let last_new_line_index = match find_new_line_pos(&actual_buf) {
            Some(index) => index,
            None => {
                println!("No new line found in the read buffer");
                bytes_not_processed += bytes_read;
                if bytes_not_processed == buf.len(){
                    panic!("No new line found in the read buffer");
                }
                continue; 
            }
        };

        let buf_boxed = Box::<[u8]>::from(&actual_buf[..(last_new_line_index + 1)]);
        sender.send(buf_boxed).expect("Failed to send buffer");

        actual_buf.copy_within(last_new_line_index+1.., 0);
        bytes_not_processed = actual_buf.len() - last_new_line_index - 1;
    }
}


fn run_in_parallel(file: fs::File) -> FxHashMap<Box<[u8]>, Record> {
    // Start the processor threads
    let (sender, receiver) = crossbeam_channel::bounded::<Box<[u8]>>(1_000);
    let handles = start_processing_threads(&receiver);

    // Read the file in chunks and send the chunks to the processor threads
    chunk_and_send_data(&sender, file);
    drop(sender);

    // Combine data from all threads
    let mut result = FxHashMap::<Box<[u8]>, Record>::default();
    for handle in handles {
        let map = handle.join().unwrap();
        for (name, record) in map.into_iter() {
            result.entry(name).or_insert(Record::default()).merge(record);
        }
    }
    result
}


fn main() {
    if let Some(path) = env::args().nth(1) {
        let file = std::fs::File::open(&path).unwrap();
        let mut hashmap_result = run_in_parallel(file);
        let mut v = hashmap_result.iter_mut().collect::<Vec<_>>();

        v.sort_unstable_by_key(|p| p.0);
        for (name, record) in &v {
            println!("{}: {}",std::str::from_utf8(name).unwrap(), record);
        }
    } else {
        println!("No file specified");
    }
}
