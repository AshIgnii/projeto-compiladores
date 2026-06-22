use crate::reader::Reader;
use crate::symbols::Terminal;
use crate::token::Token;

pub struct Automaton<'a> {
    reader: Reader<'a>,
}

impl<'a> Automaton<'a> {
    pub fn new(reader: Reader<'a>) -> Automaton<'a> {
        Automaton { reader }
    }

    pub fn blank_char(&mut self) -> Option<Token<'a>> {
        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_whitespace() {
                self.reader.next_char();
            } else {
                break;
            }
        }
        None
    }

    pub fn id_or_keyword(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        match self.reader.peek() {
            Some(byte) if byte.is_ascii_alphabetic() => {
                self.reader.next_char();
            }
            _ => return None,
        }

        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_alphanumeric() {
                self.reader.next_char();
            } else {
                break;
            }
        }

        let lexema = self.reader.slice_from(start);
        let terminal = Terminal::keyword_to_terminal(lexema).unwrap_or(Terminal::Id);
        Some(Token {
            terminal,
            lexema,
            line: start_line,
            column: start_column,
        })
    }

    pub fn int_or_float(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        match self.reader.peek() {
            Some(byte) if byte.is_ascii_digit() => {
                self.reader.next_char();
            }
            _ => return None,
        }

        while let Some(byte) = self.reader.peek() {
            if byte.is_ascii_digit() {
                self.reader.next_char();
            } else {
                break;
            }
        }

        if let Some(b'.') = self.reader.peek() {
            self.reader.next_char();

            match self.reader.peek() {
                Some(byte) if byte.is_ascii_digit() => {
                    self.reader.next_char();
                }
                _ => {
                    return Some(Token {
                        terminal: Terminal::Error,
                        lexema: self.reader.slice_from(start),
                        line: start_line,
                        column: start_column,
                    });
                }
            }

            while let Some(byte) = self.reader.peek() {
                if byte.is_ascii_digit() {
                    self.reader.next_char();
                } else {
                    break;
                }
            }
        }

        Some(Token {
            terminal: Terminal::Num,
            lexema: self.reader.slice_from(start),
            line: start_line,
            column: start_column,
        })
    }

    pub fn assign_or_equals(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        if let Some(b'=') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::Equals,
                    lexema: self.reader.slice_from(start),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::Assign,
                lexema: self.reader.slice_from(start),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn not_equals(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        if let Some(b'!') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::NotEquals,
                    lexema: self.reader.slice_from(start),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::Error,
                lexema: self.reader.slice_from(start),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn less_than_or_less_than_or_equal(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        if let Some(b'<') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::LessThanOrEqual,
                    lexema: self.reader.slice_from(start),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::LessThan,
                lexema: self.reader.slice_from(start),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn greater_than_or_greater_than_or_equal(&mut self) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        if let Some(b'>') = self.reader.peek() {
            self.reader.next_char();

            if let Some(b'=') = self.reader.peek() {
                self.reader.next_char();
                return Some(Token {
                    terminal: Terminal::GreaterThanOrEqual,
                    lexema: self.reader.slice_from(start),
                    line: start_line,
                    column: start_column,
                });
            }

            return Some(Token {
                terminal: Terminal::GreaterThan,
                lexema: self.reader.slice_from(start),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    fn single(&mut self, expected: u8, terminal: Terminal) -> Option<Token<'a>> {
        let start_line = self.reader.line;
        let start_column = self.reader.column;
        let start = self.reader.offset();

        if self.reader.peek() == Some(expected) {
            self.reader.next_char();
            return Some(Token {
                terminal,
                lexema: self.reader.slice_from(start),
                line: start_line,
                column: start_column,
            });
        }

        None
    }

    pub fn plus(&mut self) -> Option<Token<'a>> {
        self.single(b'+', Terminal::Plus)
    }

    pub fn minus(&mut self) -> Option<Token<'a>> {
        self.single(b'-', Terminal::Minus)
    }

    pub fn multiply(&mut self) -> Option<Token<'a>> {
        self.single(b'*', Terminal::Multiply)
    }

    pub fn divide(&mut self) -> Option<Token<'a>> {
        self.single(b'/', Terminal::Divide)
    }

    pub fn start_paren(&mut self) -> Option<Token<'a>> {
        self.single(b'(', Terminal::StartParen)
    }

    pub fn end_paren(&mut self) -> Option<Token<'a>> {
        self.single(b')', Terminal::EndParen)
    }

    pub fn start_brace(&mut self) -> Option<Token<'a>> {
        self.single(b'{', Terminal::StartBrace)
    }

    pub fn end_brace(&mut self) -> Option<Token<'a>> {
        self.single(b'}', Terminal::EndBrace)
    }

    pub fn semicolon(&mut self) -> Option<Token<'a>> {
        self.single(b';', Terminal::Semicolon)
    }

    pub fn comma(&mut self) -> Option<Token<'a>> {
        self.single(b',', Terminal::Comma)
    }

    pub fn next_token(&mut self) -> Option<Token<'a>> {
        self.blank_char();

        match self.reader.peek() {
            None => Some(Token {
                terminal: Terminal::Eof,
                lexema: "",
                line: self.reader.line,
                column: self.reader.column,
            }),
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
                    let start_line = self.reader.line;
                    let start_column = self.reader.column;
                    let start = self.reader.offset();
                    self.reader.next_char();
                    Some(Token {
                        terminal: Terminal::Error,
                        lexema: self.reader.slice_from(start),
                        line: start_line,
                        column: start_column,
                    })
                }
            },
        }
    }
}
