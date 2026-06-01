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

    pub fn token(&mut self, tok: &Token) {
        if self.verbose {
            writeln!(
                self.writer,
                "\n{CYAN}{BOLD}>> {} {:?}{RESET}{DIM}  (codigo {}, linha {}){RESET}",
                tok.terminal.name(),
                tok.lexema,
                tok.terminal.to_code(),
                tok.line
            )
            .unwrap();
        }
    }

    pub fn step_match(&mut self, symbol: Symbol, stack: &[Symbol]) {
        if self.verbose {
            self.step(GREEN, &format!("corta {}", symbol.name()), stack);
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
                    "M({}, {}) = empilha p{}",
                    nonterminal.name(),
                    lookahead.name(),
                    production
                ),
                stack,
            );
        }
    }

    pub fn recover_drop_terminal(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(YELLOW, "recupera: descarta terminal", stack);
        }
    }

    pub fn recover_sync(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(YELLOW, "recupera: sincroniza", stack);
        }
    }

    pub fn recover_drop_token(&mut self, stack: &[Symbol]) {
        if self.verbose {
            self.step(YELLOW, "recupera: descarta token", stack);
        }
    }

    pub fn lexical_error(&mut self, tok: &Token) {
        let msg = format!("linha {}: Caractere invalido '{}'", tok.line, tok.lexema);
        self.record("Erro lexico", msg);
    }

    pub fn expected(&mut self, expected: Symbol, found: &Token) {
        let msg = format!(
            "linha {}: Esperado '{}', Encontrado '{}'",
            found.line,
            expected.name(),
            found_str(found)
        );
        self.record("Erro sintatico", msg);
    }

    pub fn unexpected(&mut self, nonterminal: Symbol, found: &Token) {
        let msg = format!(
            "linha {}: Token inesperado '{}' em <{}>",
            found.line,
            found_str(found),
            nonterminal.name()
        );
        self.record("Erro sintatico", msg);
    }

    pub fn accepted(&mut self) {
        writeln!(self.writer, "\n{GREEN}{BOLD}Entrada aceita.{RESET}").unwrap();
    }

    pub fn summary(&mut self) {
        let total = self.errors.len();
        writeln!(
            self.writer,
            "\n{RED}{BOLD} {total} erro(s) encontrado(s){RESET}"
        )
        .unwrap();
        for (i, erro) in self.errors.iter().enumerate() {
            writeln!(self.writer, "{RED}  {:>2}. {}{RESET}", i + 1, erro).unwrap();
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
        let pad = if self.width >= 17 + ACTION_WIDTH + 16 {
            ACTION_WIDTH
        } else {
            action.chars().count()
        };
        write!(
            self.writer,
            "   {DIM}acao:{RESET} {color}{action:<pad$}{RESET}  {DIM}pilha:{RESET}"
        )
        .unwrap();

        let mut used = 3 + 6 + action.chars().count().max(pad) + 8;
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

fn detect_width() -> usize {
    std::env::var("COLUMNS")
        .ok()
        .and_then(|cols| cols.parse::<usize>().ok())
        .or_else(|| terminal_size().map(|(Width(w), _)| w as usize))
        .unwrap_or(usize::MAX)
}

fn found_str(tok: &Token) -> &str {
    if tok.lexema.is_empty() {
        tok.terminal.name()
    } else {
        tok.lexema.as_str()
    }
}
