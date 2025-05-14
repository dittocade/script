use winnow::Parser;
mod grammar;
mod parser;
use parser::*;
fn main() {
    let binding = std::fs::read_to_string(std::env::args().nth(1).unwrap()).unwrap();
    let mut input = binding.as_str();
    let output = file.parse(&mut input);
    match output  {
         Ok(output) => println!("{:#?}", output),
         Err(error) => {
                let message = error.inner().to_string();
                let input = (*error.input()).to_owned();
                let span = error.char_span();
                let message = annotate_snippets::Level::Error.title(&message)
                    .snippet(annotate_snippets::Snippet::source(&input)
                        .fold(true)
                        .annotation(annotate_snippets::Level::Error.span(span.clone()))
                    );
                let renderer = annotate_snippets::Renderer::plain();
                let rendered = renderer.render(message);
                println!("{}", rendered);
                
         }
    };
}
