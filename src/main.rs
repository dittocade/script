use winnow::Parser;
mod grammar;
mod parser;

fn main() {
    let binding = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut src = binding.as_str();
    let output = parser::call().parse(&mut src).unwrap();
    println!("{:#?}", output);
}
