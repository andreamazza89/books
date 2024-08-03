use std::process::Stdio;
use std::sync::mpsc::channel;
use std::thread;

fn main() {
    let asdf = thread::spawn(|| {});

    let (tx, rx) = channel();

    tx.send(32);

    let s2 = tx.clone();

    let r2 = rx.recv();

    println!("{:?}", 22);
}

