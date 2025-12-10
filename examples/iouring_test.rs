use io_uring::{opcode, types, IoUring};
use monke::GiB;
use std::fs::OpenOptions;
use std::io::{self};
use std::os::unix::io::AsRawFd;

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(16)?;
    /*
    let mut ring: IoUring = IoUring::builder()
        .setup_coop_taskrun()
        .setup_taskrun_flag()
        .setup_cqsize(64)
        .build(64)?;
    */

    let fd = OpenOptions::new().read(true).open("/data/output")?;

    let mut buf = vec![0; GiB!(16)];

    let mut hasher = blake3::Hasher::new();

    for j in 0..4 {
        for i in 0..16 {
            let read_e = opcode::Read::new(types::Fd(fd.as_raw_fd()), buf[GiB!(1) * i..].as_mut_ptr(), GiB!(1) as _)
                .build()
                .user_data(16 * j + i as u64);

            unsafe {
                ring.submission().push(&read_e).expect("submission queue is full");
            }
        }

        ring.submit_and_wait(16)?;

        for _ in 0..16 {
            let _cqe = ring.completion().next().expect("completion queue is empty");
        }

        hasher.update(&buf);
        eprintln!("{}", monke::fmt_size(hasher.count()));
    }

    let hash = hasher.finalize();
    eprintln!("HASH: {}", hash);

    // let res = unsafe { std::arch::x86_64::_tzcnt_u64(64) };
    assert_eq!(hash.to_string(), "c5885536481fc3c50d5ceb70cc86fd8b3f1bec3740ada9468101f5d4eac348d1");
    Ok(())
}
