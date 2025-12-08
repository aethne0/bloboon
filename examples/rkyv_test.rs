use std::{fs::OpenOptions, io::Write, path::Path};

use rkyv::{Archive, Deserialize, Serialize, rancor::Error};

#[derive(Archive, Deserialize, Serialize, Debug, PartialEq)]
struct Test {
    index: u64,
    value: u64,
    stuff: String,
}

fn main() {
    let val = Test {
        index: 256,
        value: 1,
        stuff:
            "Serializability is used to keep the data in the data item in a consistent state. It is the major criterion for the correctness of concurrent transactions' schedule, and thus supported in all general purpose database systems. Schedules that are not serializable are likely to generate erroneous outcomes; which can be extremely harmful (e.g., when dealing with money within banks).[1][2][3]"
                .to_string(),
    };

    let bytes = rkyv::to_bytes::<Error>(&val).unwrap();
    let path = Path::new("test.bin");
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    f.write(&bytes).unwrap();

    let cmp = lz4_flex::compress(&bytes);

    let path = Path::new("test.lz4");
    let mut f = OpenOptions::new()
        .write(true)
        .create(true)
        .open(&path)
        .unwrap();
    f.write(&cmp).unwrap();
}
