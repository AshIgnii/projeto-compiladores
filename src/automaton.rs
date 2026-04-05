use crate::reader::Reader;
use crate::terminal::Terminal;
use crate::token::Token;

pub struct Automaton<'a> {
    reader: Reader<'a>,
}

impl<'a> Automaton<'a> {
    pub fn new(reader: Reader<'a>) -> Automaton<'a> {
        Automaton { reader }
    }

    pub fn blank_char(&mut self) -> Option<Token> {
        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_whitespace() {
                self.reader.next_char();
            } else {
                break;
            }
        }
        None
    }

    pub fn id_or_keyword(&mut self) -> Option<Token> {
        let mut lexema = String::new();
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(byte) = self.reader.peek() {
            if byte.is_ascii_alphabetic() {
                lexema.push(byte as char);
                self.reader.next_char();
            } else {
                return None;
            }
        } else {
            return None;
        }

        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_alphanumeric() {
                lexema.push(byte as char);
                self.reader.next_char();
            } else {
                break;
            }
        }

        let terminal = Terminal::keyword_to_terminal(&lexema.as_str()).unwrap_or(Terminal::Id);
        Some(Token {
            terminal,
            lexema,
            line: start_line,
            column: start_column,
        })
    }

    pub fn int_or_float(&mut self) -> Option<Token> {
        let mut lexema = String::new();
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        match self.reader.peek() {
            Some(byte) if byte.is_ascii_digit() => {
                lexema.push(byte as char);
                self.reader.next_char();
            }
            _ => return None,
        }

        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_digit() {
                lexema.push(byte as char);
                self.reader.next_char();
            } else {
                break;
            }
        }

        if let Some(b'.') = self.reader.peek() {
            lexema.push('.');
            self.reader.next_char();

            match self.reader.peek() {
                Some(byte) if byte.is_ascii_digit() => {
                    lexema.push(byte as char);
                    self.reader.next_char();
                }
                _ => {
                    return Some(Token {
                        terminal: Terminal::Error,
                        lexema,
                        line: start_line,
                        column: start_column,
                    });
                }
            }

            while let Some(byte) = self.reader.peek() {
                if byte.is_ascii_digit() {
                    lexema.push(byte as char);
                    self.reader.next_char();
                } else {
                    break;
                }
            }
        }

        Some(Token {
            terminal: Terminal::Num,
            lexema,
            line: start_line,
            column: start_column,
        })
    }

    pub fn assign_or_equals(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'=') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::Equals,
                    lexema: "==".to_string(),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::Assign,
                lexema: "=".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn not_equals(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'!') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::NotEquals,
                    lexema: "!=".to_string(),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::Error,
                lexema: "!".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn less_than_or_less_than_or_equal(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'<') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::LessThanOrEqual,
                    lexema: "<=".to_string(),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::LessThan,
                lexema: "<".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn greater_than_or_greater_than_or_equal(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'>') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::GreaterThanOrEqual,
                    lexema: ">=".to_string(),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::GreaterThan,
                lexema: ">".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn plus(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'+') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Plus,
                lexema: "+".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn minus(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'-') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Minus,
                lexema: "-".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn multiply(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'*') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Multiply,
                lexema: "*".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn divide(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'/') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Divide,
                lexema: "/".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn start_paren(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'(') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::StartParen,
                lexema: "(".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn end_paren(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b')') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::EndParen,
                lexema: ")".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn start_brace(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'{') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::StartBrace,
                lexema: "{".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn end_brace(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b'}') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::EndBrace,
                lexema: "}".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn semicolon(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b';') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Semicolon,
                lexema: ";".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn comma(&mut self) -> Option<Token> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;

        if let Some(b',') = self.reader.peek() {
            self.reader.next_char();
            return Some(Token {
                terminal: Terminal::Comma,
                lexema: ",".to_string(),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn next_token(&mut self) -> Option<Token> {
        self.blank_char();

        match self.reader.peek() {
            None => {
                Some(Token {
                    terminal: Terminal::Eof,
                    lexema: String::new(),
                    line: self.reader.line,
                    column: self.reader.column,
                })
            }
            Some(b) => match b {
                b'a'..=b'z' | b'A'..=b'Z' => self.id_or_keyword(),
                b'0'..=b'9' => self.int_or_float(),
                b'=' => self.assign_or_equals(),
                b'!' => self.not_equals(),
                b'<' => self.less_than_or_less_than_or_equal(),
                b'>' => self.greater_than_or_greater_than_or_equal(),
                b'+' => self.plus(),
                b'-' => self.minus(),
                b'*' => self.multiply(),
                b'/' => self.divide(),
                b'(' => self.start_paren(),
                b')' => self.end_paren(),
                b'{' => self.start_brace(),
                b'}' => self.end_brace(),
                b';' => self.semicolon(),
                b',' => self.comma(),
                _ => {
                    self.reader.next_char();
                    Some(Token {
                        terminal: Terminal::Error,
                        lexema: (b as char).to_string(),
                        line: self.reader.line,
                        column: self.reader.column - 1,
                    })
                }
            },
        }
    }
}
