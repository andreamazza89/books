use std::{fs, io};
use std::cell::RefCell;

#[derive(Clone,Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn s(shoes: Vec<Shoe>, size: u32) -> Vec<Shoe> {
    shoes
        .into_iter()
        .filter(|s| s.size == size)
        .collect()

}

fn asdf() {
    let asdf: Vec<Shoe> = vec![];

    s(asdf.clone(), 22);

    println!("{:?}", asdf);

}

fn read_and_upcase() -> Result<usize, io::Error> {
    let file_res: io::Result<String> = fs::read_to_string("ciao.txt");

    let arr = [2];

    let iter = arr.iter();

    iter.sum::<i32>();



    return match file_res {
        Ok(string) => { Ok(string.len()) }
        Err(e) => { Err(e) }
    }
}

fn read_and_upcase2() -> Result<usize, io::Error> {
    let file_res: String = fs::read_to_string("ciao.txt")?;

    Ok(file_res.len())
}
