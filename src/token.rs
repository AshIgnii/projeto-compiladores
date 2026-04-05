use crate::terminal::Terminal;
use std::fmt;
pub struct Token {
    pub terminal: Terminal,
    pub lexema: String,
    pub line: usize,
    pub column: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "<{:?}, {}, \"{}\", {}:{}>",
            self.terminal,
            self.terminal.to_code(),
            self.lexema,
            self.line,
            self.column
        )
    }
}
