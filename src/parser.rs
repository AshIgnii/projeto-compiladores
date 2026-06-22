use crate::automaton::Automaton;
use crate::logger::Logger;
use crate::parsing;
use crate::semantic::SemanticAnalyzer;
use crate::symbols::{Action, EOF_SYMBOL, START_SYMBOL, Symbol, Terminal};
use crate::token::Token;
use std::io::Write;

pub struct Parser<'a> {
    automaton: Automaton<'a>,
    stack: Vec<Symbol>,
    analyzer: SemanticAnalyzer<'a>,
    errors: usize,
}

impl<'a> Parser<'a> {
    pub fn new(automaton: Automaton<'a>) -> Parser<'a> {
        Parser {
            automaton,
            stack: Vec::with_capacity(64),
            analyzer: SemanticAnalyzer::new(),
            errors: 0,
        }
    }

    pub fn parse<W: Write>(&mut self, logger: &mut Logger<W>) -> bool {
        self.stack.clear();
        self.stack.push(EOF_SYMBOL);
        self.stack.push(START_SYMBOL);

        let mut current = self.advance(logger);
        let mut lookahead = current.terminal.as_symbol();

        while let Some(&top) = self.stack.last() {
            let finished = if top.is_terminal() {
                self.match_terminal(top, &mut current, &mut lookahead, logger)
            } else if top.is_action() {
                self.run_action(top, logger);
                false
            } else {
                self.expand(top, &mut current, &mut lookahead, logger)
            };
            if finished {
                break;
            }
        }

        self.errors += self.analyzer.errors;
        self.report_result(logger)
    }

    fn run_action<W: Write>(&mut self, top: Symbol, logger: &mut Logger<W>) {
        self.stack.pop();
        logger.step_action(top, &self.stack);
        self.analyzer.execute(Action::from_code(top.code()), logger);
    }

    fn match_terminal<W: Write>(
        &mut self,
        top: Symbol,
        current: &mut Token<'a>,
        lookahead: &mut Symbol,
        logger: &mut Logger<W>,
    ) -> bool {
        if top == *lookahead {
            self.stack.pop();
            logger.step_match(top, &self.stack);
            self.analyzer.on_match(current);
            if top == EOF_SYMBOL {
                return true;
            }
            *current = self.advance(logger);
            *lookahead = current.terminal.as_symbol();
        } else {
            self.errors += 1;
            logger.expected(top, current);
            self.stack.pop();
            logger.recover_drop_terminal(&self.stack);
        }
        false
    }

    fn expand<W: Write>(
        &mut self,
        top: Symbol,
        current: &mut Token<'a>,
        lookahead: &mut Symbol,
        logger: &mut Logger<W>,
    ) -> bool {
        let production = parsing::entry(top, *lookahead);
        if production != 0 {
            self.stack.pop();
            self.stack
                .extend_from_slice(parsing::production_rhs(production));
            logger.step_expand(production, top, *lookahead, &self.stack);
            return false;
        }

        self.errors += 1;
        logger.unexpected(top, current);
        if parsing::follow(top).contains(lookahead) {
            self.stack.pop();
            logger.recover_sync(&self.stack);
        } else if *lookahead == EOF_SYMBOL {
            return true;
        } else {
            *current = self.advance(logger);
            *lookahead = current.terminal.as_symbol();
            logger.recover_drop_token(&self.stack);
        }
        false
    }

    fn report_result<W: Write>(&mut self, logger: &mut Logger<W>) -> bool {
        if self.errors == 0 {
            logger.accepted();
            true
        } else {
            logger.summary();
            false
        }
    }

    fn advance<W: Write>(&mut self, logger: &mut Logger<W>) -> Token<'a> {
        loop {
            match self.automaton.next_token() {
                Some(token) => {
                    if token.terminal == Terminal::Error {
                        self.errors += 1;
                        logger.lexical_error(&token);
                        continue;
                    }
                    logger.token(&token);
                    return token;
                }
                None => {
                    return Token {
                        terminal: Terminal::Eof,
                        lexema: "",
                        line: 0,
                        column: 0,
                    };
                }
            }
        }
    }
}
