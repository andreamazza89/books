fn main() {
    let mut s = String::from("hello");

    change(&mut s);
}

fn change(some_string: &String) {
    println!("{}", some_string)
    // some_string.push_str(", world");
}
