use regex::bytes::Regex;

#[derive(Debug)]
pub struct Token {
    value: TokenValue,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub struct LexerError {
    value: LexerErrorValue,
    line: usize,
    column: usize,
}

#[derive(Debug)]
pub enum LexerErrorValue {
    UnrecognizedToken
}

#[derive(Debug)]
pub enum TokenValue {
    Number(i32),
    Boolean(bool),
    Character(u8),
    Cross,
    Dash,
    Star,
    Slash,
    Whitespace,
    OpenRoundBracket,
    CloseRoundBracket,
    OpenCurlyBracket,
    CloseCurlyBracket,
    Equal,
    ExclEqual,
    DoubleEqual,
    DoubleAnd,
    DoublePipe,
    Excl,
}

pub struct Lexer<'a> {
    cstream: &'a[u8],
    line: usize,
    column: usize,
    it: usize
}

type TokenStream = Vec<Token>;
type LexerResult<T> = Result<T, LexerError>;

impl<'a> Lexer<'a> {
    pub fn from_cstream(cstream: &'a[u8]) -> Lexer<'a> {
        Lexer {
            cstream,
            it: 0,
            line: 1,
            column: 1,
        }
    }

    fn get_line(&self) -> usize {
        return self.cstream[0..self.it].iter()
                                        .map(|b| if *b == b'\n' { 1 as usize } else { 0 as usize })
                                        .reduce(|a, b| a + b)
                                        .unwrap_or(0) + 1;
    }

    fn get_column(&self) -> usize {
        return self.cstream[0..self.it].iter()
                    .enumerate()
                    .filter(|n| *(n.1) == b'\n')
                    .map(|e| e.0).last()
                    .map_or(self.it + 1, |n| self.it - n);
    }

    fn get_position(&self) -> (usize, usize) {
        return (self.get_line(), self.get_column());
    }


    fn move_curs(&mut self, offset: usize) {
        self.it += offset;
        self.column = self.get_column();
        self.line = self.get_line();
    }

    fn try_extract_number(&mut self) -> Option<Token> {
        let regex = Regex::new(r"^\d+").unwrap();
        let m = regex.find(&self.cstream[self.it..])?;
        let val: i32 = m.as_bytes()
                        .into_iter()
                        .map(|b| b - b'0')
                        .map(|b| b as i32)
                        .reduce(|a, b| a * 10 + b)
                        .unwrap_or(0);
        let (line, column) = self.get_position();
        self.move_curs(m.end());
        return Some(Token {
            value: TokenValue::Number(val),
            line,
            column
        });

    }

    fn try_extract_whitespace(&mut self) -> Option<Token> {
        let regex = Regex::new(r"^\s+").unwrap();
        match regex.find(&self.cstream[self.it..]) {
            Some(m) => {
                let line = self.line;
                let column = self.column;
                self.move_curs(m.end());
                Some(Token {
                    value: TokenValue::Whitespace,
                    line,
                    column
                })
            },
            None => None
        }
    }

    fn try_extract_singles(&mut self) -> Option<Token> {

        match self.cstream.get(self.it) {
            Some(b) if *b == b'=' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::Equal,
                    line,
                    column
                })
            },
            Some(b) if *b == b'+' => {

                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::Cross,
                    line,
                    column
                })
            },
            Some(b) if *b == b'-' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::Dash,
                    line,
                    column
                })
            },
            Some(b) if *b == b'*' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::Star,
                    line,
                    column
                })
            },
            Some(b) if *b == b'/' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::Slash,
                    line,
                    column
                })
            },
            Some(b) if *b == b'(' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::OpenRoundBracket,
                    line,
                    column
                })
            },
            Some(b) if *b == b')' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::CloseRoundBracket,
                    line,
                    column
                })
            },
            Some(b) if *b == b'{' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::OpenCurlyBracket,
                    line,
                    column
                })
            },
            Some(b) if *b == b'}' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::CloseCurlyBracket,
                    line,
                    column
                })
            },
            Some(b) if *b == b'!' => {
                let line = self.line;
                let column = self.column;
                self.move_curs(1);
                Some(Token{
                    value: TokenValue::CloseCurlyBracket,
                    line,
                    column
                })
            },
            _ => None
        }
    }

    fn try_extract_doubles(&mut self) -> Option<Token> {
        match self.cstream.get(self.it..self.it+2) {
            Some(val) if *val == b"=="[..] => {
                let (line, column) = (self.line, self.column);
                self.move_curs(2);
                return Some(Token{
                    value: TokenValue::DoubleEqual,
                    line, column
                });
            }
            Some(val) if *val == b"!="[..] => {
                let (line, column) = (self.line, self.column);
                self.move_curs(2);
                return Some(Token{
                    value: TokenValue::ExclEqual,
                    line, column
                });
            }
            Some(val) if *val == b"&&"[..] => {
                let (line, column) = (self.line, self.column);
                self.move_curs(2);
                return Some(Token{
                    value: TokenValue::DoubleAnd,
                    line, column
                });
            }
            Some(val) if *val == b"||"[..] => {
                let (line, column) = (self.line, self.column);
                self.move_curs(2);
                return Some(Token{
                    value: TokenValue::DoublePipe,
                    line, column
                });
            }
            _ => ()
        };
        return None;
    }

    fn try_extract_boolean(&mut self) -> Option<Token> {
        let bool_true = b"True";
        let bool_false = b"False";
        match self.cstream.get(self.it..self.it+bool_true.len()) {
            Some(val) if *val == bool_true[..] => {
                let line = self.line;
                let column = self.column;
                self.move_curs(bool_true.len());
                return Some(Token{
                    value: TokenValue::Boolean(true),
                    line, column
                });
            },
            _ => ()
        };

        match self.cstream.get(self.it..self.it+bool_false.len()) {
            Some(val) if *val == bool_false[..] => {
                let (line, column) = (self.get_line(), self.get_column());
                self.move_curs(bool_false.len());
                return Some(Token {
                    value: TokenValue::Boolean(false),
                    line, column
                });
            },
            _ => ()
        };

        return None;
    }

    pub fn execute(&mut self) -> LexerResult<TokenStream> {
        let mut tok_stream: Vec<Token> = Vec::new();

        while self.it < self.cstream.len() {

            match self.try_extract_number() {
                Some(tok) => { 
                    tok_stream.push(tok);
                    continue;
                },
                None => (),
            };

            match self.try_extract_whitespace() {
                Some(tok) => {
                    tok_stream.push(tok);
                    continue;
                },
                None => (),
            };

            match self.try_extract_doubles() {
                Some(tok) => {
                    tok_stream.push(tok);
                    continue;
                }
                None => ()
            };

            match self.try_extract_singles() {
                Some(tok) => {
                    tok_stream.push(tok);
                    continue;
                },
                None => ()
            };

            match self.try_extract_boolean() {
                Some(tok) => {
                    tok_stream.push(tok);
                    continue;
                },
                None => ()
            };

            return Err(LexerError {
                value: LexerErrorValue::UnrecognizedToken,
                line: self.line,
                column: self.column
            });
        }

        return Ok(tok_stream);

    }

    pub fn debug(&mut self) {
        let res: LexerResult<TokenStream> = self.execute();
        match res {
            Ok(tokens) => {
                println!("{:#?}", tokens);
            },
            Err(e) => {
                println!("{:?}", e);
            }
        }
    }
}

pub fn hello() {
    println!("Hello world")
}