mod highlighter_tokenizer;
mod token;
mod lexers;
pub mod constants;

use mdxjs::hast;

use crate::lexers::{javascript, raw};

pub fn highlight(input: Vec<char>, lang: &str) -> Vec<hast::Node> {
    match lang {
        "javascript" | "js" => javascript::highlight(input),
        _ => raw::highlight(input),
    }
}
