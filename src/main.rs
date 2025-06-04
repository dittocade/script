use winnow::{combinator::terminated, stream::TokenSlice, Parser};

mod lexer;
mod parser;

fn main() {
    let filename = std::env::args().nth(1).unwrap();
    let binding = std::fs::read_to_string(filename).unwrap();
    let input = binding.as_str();
    let tokens = lexer::tokens.parse(input);
    let Ok(tokens) = tokens else {
        match tokens  {
             Ok(tokens) => println!("{:#?}", tokens),
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
        return
    };
    let grammar = terminated(parser::statements0, lexer::token::Kind::EndOfFile).parse(TokenSlice::new(&tokens));
    println!("{:#?}", grammar);
}
