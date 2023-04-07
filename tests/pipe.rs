use easy_parallel::Parallel;
use futures_lite::{future, prelude::*};
use piper::pipe;

use std::thread::sleep;
use std::time::Duration;

#[test]
fn smoke() {
    let (mut r, mut w) = pipe(8);
    let mut buf = [0u8; 8];

    future::block_on(w.write_all(&[1, 2, 3, 4, 5, 6, 7, 8])).unwrap();
    future::block_on(r.read_exact(&mut buf)).unwrap();

    assert_eq!(buf, [1, 2, 3, 4, 5, 6, 7, 8]);

    future::block_on(w.write_all(&[9, 10, 11, 12, 13, 14, 15, 16])).unwrap();
    future::block_on(r.read_exact(&mut buf)).unwrap();

    assert_eq!(buf, [9, 10, 11, 12, 13, 14, 15, 16]);

    drop(w);
    assert_eq!(future::block_on(r.read(&mut buf)).ok(), Some(0));
}

#[test]
fn read() {
    let (mut r, mut w) = pipe(100);
    let ms = Duration::from_micros;

    Parallel::new()
        .add(move || {
            let mut buf = [0u8; 3];
            sleep(ms(1000));
            future::block_on(r.read_exact(&mut buf)).unwrap();
            assert_eq!(buf, [1, 2, 3]);

            sleep(ms(1000));
            future::block_on(r.read_exact(&mut buf)).unwrap();
            assert_eq!(buf, [4, 5, 6]);
        })
        .add(move || {
            sleep(ms(1500));
            future::block_on(w.write_all(&[1, 2, 3, 4, 5, 6])).unwrap();
        })
        .run();
}

#[should_panic]
#[test]
fn zero_cap_pipe() {
    let _ = pipe(0);
}

#[should_panic]
#[test]
fn large_pipe() {
    let _ = pipe(core::usize::MAX);
}
