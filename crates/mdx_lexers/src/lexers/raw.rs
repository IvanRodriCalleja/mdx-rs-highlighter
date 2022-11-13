use mdxjs::hast;

use crate::{highlighter_tokenizer::HighlighterTokenizer, token::Token};

pub struct Lexer {
    input: Vec<char>,
    pub position: usize,
    pub read_position: usize,
    pub ch: char,
}

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}

impl Lexer {
    pub fn new(input: Vec<char>) -> Self {
        Self {
            input,
            position: 0,
            read_position: 0,
            ch: '\0',
        }
    }

    pub fn read_char(&mut self) {
        if self.read_position >= self.input.len() {
            self.ch = '\0';
        } else {
            self.ch = self.input[self.read_position];
        }
        self.position = self.read_position;
        self.read_position += 1;
    }

    pub fn next_token(&mut self) -> Token {
        let read_identifier = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && is_letter(l.ch) {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let read_number = |l: &mut Lexer| -> Vec<char> {
            let position = l.position;
            while l.position < l.input.len() && l.ch.is_numeric() {
                l.read_char();
            }
            l.input[position..l.position].to_vec()
        };

        let tok: Token;
        match self.ch {
            '\n' => {
                tok = Token::ENDL(self.ch);
            }
            '\0' => {
                tok = Token::EOF;
            }
            _ => {
                return if is_letter(self.ch) {
                    #[allow(unused_variables)]
                    let start_position = self.position;
                    #[allow(unused_mut)]
                    let mut identifier: Vec<char> = read_identifier(self);
                    match get_keyword_token(&identifier) {
                        Ok(keyword_token) => keyword_token,
                        Err(_) => Token::IDENT(identifier),
                    }
                } else if self.ch.is_numeric() {
                    let identifier: Vec<char> = read_number(self);
                    Token::IDENT(identifier)
                } else {
                    Token::ILLEGAL
                }
            }
        }
        self.read_char();
        tok
    }
}

pub fn get_keyword_token(identifier: &Vec<char>) -> Result<Token, String> {
    let id: String = identifier.into_iter().collect();
    match &id[..] {
        _ => Err(String::from("Not a keyword")),
    }
}

pub fn highlight(input: Vec<char>) -> Vec<hast::Node> {
    let mut l = Lexer::new(input);
    l.read_char();
    let mut tokens = HighlighterTokenizer::new();
    loop {
        let token = l.next_token();
        if token == Token::EOF {
            tokens.end_of_line();
            break;
        }

        match token {
            Token::IDENT(value) => {
                tokens.add_text(value.iter().collect::<String>());
            }
            Token::ENDL(_) => {
                tokens.end_of_line();
            }
            _ => {
                if l.ch == '<' {
                    tokens.add_text("&lt;".into());
                    l.read_char();
                    continue;
                }
                if l.ch == '>' {
                    tokens.add_text("&gt;".into());
                    l.read_char();
                    continue;
                }
                tokens.add_text(l.ch.to_string());
                l.read_char();
            }
        }
    }

    tokens.get_highlighted_code()
}
