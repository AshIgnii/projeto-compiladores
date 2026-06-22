use crate::symbols::Symbol;
use crate::token::Token;
use std::io::Write;
use terminal_size::{Width, terminal_size};

const RESET: &str = "\x1b[0m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const BLUE: &str = "\x1b[34m";
const CYAN: &str = "\x1b[36m";
const MAGENTA: &str = "\x1b[35m";

#[derive(Clone, Copy)]
pub enum RowKind {
    Normal,
    New,
    Removed,
}

const ACTION_WIDTH: usize = 42;

pub struct Logger<W: Write> {
    writer: W,
    verbose: bool,
    width: usize,
    errors: Vec<String>,
}

impl<W: Write> Logger<W> {
    pub fn new(writer: W, verbose: bool) -> Logger<W> {
        Logger {
            writer,
            verbose,
            width: detect_width(),
            errors: Vec::new(),
        }
    }

    pub fn token(&mut self, tok: &Token<'_>) {
        if self.verbose {
            writeln!(
                self.writer,
                "\n{CYAN}{BOLD}>> {} {RESET}{DIM}(lexema: {:?}, codigo: {}, linha: {}, coluna: {}){RESET}",
                tok.terminal.name(),
                tok.lexema,
                tok.terminal.to_code(),
                tok.line,
                tok.column
            )
            .unwrap();
        }
    }

    pub fn step_match(&mut self, symbol: Symbol, stack: &[Symbol]) {
        if self.verbose {
            self.step(GREEN, &format!("cortando {}", symbol.name()), stack);
        }
    }

    pub fn step_expand(
        &mut self,
        production: u8,
        nonterminal: Symbol,
        lookahead: Symbol,
        stack: &[Symbol],
    ) {
        if self.verbose {
            self.step(
                BLUE,
                &format!(
                    "M({}, {}) = empilhando p{}",
                    nonterminal.name(),
                    lookahead.name(),
                    production
                ),
                stack,
            );
        }
    }

    pub fn step_action(&mut self, action: Symbol, stack: &[Symbol]) {
        if self.verbose {
            self.step(MAGENTA, &format!("executando {}", action.name()), stack);
        }
    }

    pub fn symbol_table(&mut self, rows: &[([String; 4], RowKind)]) {
        if !self.verbose {
            return;
        }

        const HEADER: [&str; 4] = ["simbolos", "tipo", "valor", "nivel"];
        let mut width = [0usize; 4];
        for (column, label) in HEADER.iter().enumerate() {
            width[column] = label.len();
        }
        for (cells, _) in rows {
            for (column, cell) in cells.iter().enumerate() {
                width[column] = width[column].max(cell.len());
            }
        }

        let mut header = String::from("|");
        for (column, label) in HEADER.iter().enumerate() {
            header.push_str(&format!(" {} |", center(label, width[column])));
        }
        writeln!(self.writer, "\n   {DIM}{header}{RESET}").unwrap();

        if rows.is_empty() {
            writeln!(self.writer, "   {DIM}(vazia){RESET}").unwrap();
        }
        for (cells, kind) in rows {
            let mut line = String::from("|");
            for (column, cell) in cells.iter().enumerate() {
                line.push_str(&format!(" {:<width$} |", cell, width = width[column]));
            }
            match kind {
                RowKind::Normal => writeln!(self.writer, "   {line}").unwrap(),
                RowKind::New => writeln!(self.writer, "   {GREEN}{line}{RESET}").unwrap(),
                RowKind::Removed => writeln!(self.writer, "   {RED}{line}{RESET}").unwrap(),
            }
        }
        writeln!(self.writer).unwrap();
    }

    pub fn recover_drop_terminal(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(
                YELLOW,
                &format!("{RED}{BOLD}PANICO{RESET}{YELLOW} recuperando... terminal descartado"),
                stack,
            );
        }
    }

    pub fn recover_sync(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(
                YELLOW,
                &format!("{RED}{BOLD}PANICO{RESET}{YELLOW} recuperando... sincronizando"),
                stack,
            );
        }
    }

    pub fn recover_drop_token(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(
                YELLOW,
                &format!("{RED}{BOLD}PANICO{RESET}{YELLOW} recuperando... token descartado"),
                stack,
            );
        }
    }

    pub fn lexical_error(&mut self, tok: &Token<'_>) {
        let msg = format!(
            "linha {}, coluna {}: Caractere invalido '{}'",
            tok.line, tok.column, tok.lexema
        );
        self.record("Erro lexico", msg);
    }

    pub fn expected(&mut self, expected: Symbol, found: &Token<'_>) {
        let msg = format!(
            "linha {}, coluna {}: Esperado '{}', Encontrado '{}'",
            found.line,
            found.column,
            expected.name(),
            found_str(found)
        );
        self.record("Erro sintatico", msg);
    }

    pub fn unexpected(&mut self, nonterminal: Symbol, found: &Token<'_>) {
        let msg = format!(
            "linha {}, coluna {}: Token inesperado '{}' em <{}>",
            found.line,
            found.column,
            found_str(found),
            nonterminal.name()
        );
        self.record("Erro sintatico", msg);
    }

    pub fn semantic_error(&mut self, line: usize, column: usize, detail: &str) {
        let msg = format!("linha {}, coluna {}: {}", line, column, detail);
        self.record("Erro semantico", msg);
    }

    pub fn accepted(&mut self) {
        writeln!(self.writer, "\n{GREEN}{BOLD}Entrada aceita.{RESET}").unwrap();
    }

    pub fn summary(&mut self) {
        writeln!(self.writer, "\n{RED}{BOLD}Entrada rejeitada.{RESET}").unwrap();

        let total = self.errors.len();
        writeln!(
            self.writer,
            "\n{RED}{BOLD} {total} erro(s) encontrado(s){RESET}"
        )
        .unwrap();
        for erro in self.errors.iter() {
            writeln!(self.writer, "{RED}   - {}{RESET}", erro).unwrap();
        }
    }

    fn record(&mut self, kind: &str, msg: String) {
        if self.verbose {
            writeln!(self.writer, "{RED}{BOLD}  {kind}{RESET}{RED} {msg}{RESET}").unwrap();
        }
        self.errors.push(format!("{kind} {msg}"));
    }

    pub fn flush(&mut self) {
        self.writer.flush().unwrap();
    }

    fn step(&mut self, color: &str, action: &str, stack: &[Symbol]) {
        let vlen = visible_len(action);
        let pad = if self.width >= 17 + ACTION_WIDTH + 16 {
            ACTION_WIDTH
        } else {
            vlen
        };
        let spaces = " ".repeat(pad.saturating_sub(vlen));
        write!(
            self.writer,
            "   {DIM}acao:{RESET} {color}{action}{spaces}{RESET}  {DIM}pilha:{RESET}"
        )
        .unwrap();

        let mut used = 3 + 6 + vlen.max(pad) + 8;
        let total = stack.len();
        for (shown, &symbol) in stack.iter().rev().enumerate() {
            let name = symbol.name();
            let needed = 1 + name.chars().count();
            if shown > 0 && used + needed > self.width {
                write!(self.writer, " {DIM}...+{}{RESET}", total - shown).unwrap();
                break;
            }
            if symbol.is_nonterminal() {
                write!(self.writer, " {YELLOW}{}{RESET}", name).unwrap();
            } else {
                write!(self.writer, " {}", name).unwrap();
            }
            used += needed;
        }
        writeln!(self.writer).unwrap();
    }
}

fn center(text: &str, width: usize) -> String {
    let total = width.saturating_sub(text.len());
    let left = total / 2;
    let right = total - left;
    format!("{}{}{}", " ".repeat(left), text, " ".repeat(right))
}

fn detect_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse::<usize>().ok())
        .or_else(|| terminal_size().map(|(Width(w), _)| w as usize))
        .unwrap_or(usize::MAX)
}

fn visible_len(s: &str) -> usize {
    let mut len = 0;
    let mut in_escape = false;
    for c in s.chars() {
        if c == '\x1b' {
            in_escape = true;
        } else if in_escape {
            if c == 'm' {
                in_escape = false;
            }
        } else {
            len += 1;
        }
    }
    len
}

fn found_str<'b>(tok: &Token<'b>) -> &'b str {
    if tok.lexema.is_empty() {
        tok.terminal.name()
    } else {
        tok.lexema
    }
}
