use std::fs;

mod grammar;
mod parse;

fn main() -> () {
    let script = match fs::read_to_string("src/test.ds") {
        Ok(v) => v,
        Err(err) => panic!("{err:?}"),
    };
    let (remaining, output): (&str, grammar::Code) = match parse::code(&script) {
        Ok(v) => v,
        Err(err) => panic!("{err:?}"),
    };
    assert_eq!(remaining, "");
    println!("{:#?}", output);
}
