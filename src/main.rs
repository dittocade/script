use winnow::Parser;
mod grammar;
mod parser;

fn main() {
    let src = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let output = parser::call().parse_next(&src).unwrap();
    println!("{:#?}", output);
}
