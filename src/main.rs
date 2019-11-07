use std::fs;

mod html;

fn main() {
    let html = fs::read_to_string("hello_world.html").unwrap();
    let parser = html::Parser { pos: 0, input: html };

    println!("#{:?}", parser);
}
