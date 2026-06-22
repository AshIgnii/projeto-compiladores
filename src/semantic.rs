use crate::logger::{Logger, RowKind};
use crate::symbols::{Action, Terminal};
use crate::token::Token;
use std::collections::HashMap;
use std::io::Write;

#[derive(Debug, Clone, Copy, PartialEq)]
enum DataType {
    Integer,
    Float,
    Boolean,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Variable,
    Parameter,
}

struct FunctionSig {
    params: Vec<DataType>,
    return_type: DataType,
    seq: usize,
}

#[derive(Clone, Copy)]
struct VarInfo {
    data_type: DataType,
    category: Category,
    seq: usize,
}

pub struct SemanticAnalyzer<'a> {
    functions: HashMap<String, FunctionSig>,
    scopes: Vec<HashMap<String, VarInfo>>,
    pending_params: Vec<(&'a str, DataType, usize)>,
    current_function: Option<&'a str>,
    seq_counter: usize,

    type_stack: Vec<DataType>,
    name_stack: Vec<(&'a str, usize, usize)>,
    arg_stack: Vec<usize>,
    return_stack: Vec<bool>,
    block_bases: Vec<usize>,

    pending_id: Option<(&'a str, usize, usize)>,
    pending_type: Option<DataType>,
    pending_num: Option<DataType>,
    last_line: usize,
    last_col: usize,
    last_inserted: Option<(String, usize)>,
    removed: Vec<(usize, String, &'static str, &'static str, usize)>,

    pub errors: usize,
}

impl<'a> SemanticAnalyzer<'a> {
    pub fn new() -> SemanticAnalyzer<'a> {
        SemanticAnalyzer {
            functions: HashMap::new(),
            scopes: Vec::new(),
            pending_params: Vec::new(),
            current_function: None,
            seq_counter: 0,
            type_stack: Vec::new(),
            name_stack: Vec::new(),
            arg_stack: Vec::new(),
            return_stack: Vec::new(),
            block_bases: Vec::new(),
            pending_id: None,
            pending_type: None,
            pending_num: None,
            last_line: 0,
            last_col: 0,
            last_inserted: None,
            removed: Vec::new(),
            errors: 0,
        }
    }

    pub fn on_match(&mut self, token: &Token<'a>) {
        self.last_line = token.line;
        self.last_col = token.column;
        match token.terminal {
            Terminal::Int => self.pending_type = Some(DataType::Integer),
            Terminal::Float => self.pending_type = Some(DataType::Float),
            Terminal::Id => {
                self.pending_id = Some((token.lexema, token.line, token.column));
            }
            Terminal::Num => {
                self.pending_num = Some(if token.lexema.contains('.') {
                    DataType::Float
                } else {
                    DataType::Integer
                });
            }
            _ => {}
        }
    }

    pub fn execute<W: Write>(&mut self, action: Action, logger: &mut Logger<W>) {
        match action {
            Action::OpenScope => {
                let mut scope = HashMap::new();
                for (name, data_type, seq) in self.pending_params.drain(..) {
                    scope.insert(
                        name.to_string(),
                        VarInfo {
                            data_type,
                            category: Category::Parameter,
                            seq,
                        },
                    );
                }
                self.scopes.push(scope);
                self.block_bases.push(self.return_stack.len());
                self.last_inserted = None;
            }
            Action::CloseScope => {
                let level = self.scopes.len();
                if let Some(scope) = self.scopes.pop() {
                    self.removed = scope
                        .into_iter()
                        .map(|(name, info)| {
                            (
                                level,
                                name,
                                category_name(info.category),
                                type_name(info.data_type),
                                info.seq,
                            )
                        })
                        .collect();
                }
                let base = self
                    .block_bases
                    .pop()
                    .unwrap_or(self.return_stack.len())
                    .min(self.return_stack.len());
                let returns = self.return_stack.drain(base..).any(|b| b);
                self.return_stack.push(returns);
                self.last_inserted = None;
            }
            Action::InsertFunction => {
                if let Some((name, line, col)) = self.pending_id.take() {
                    let return_type = self.pending_type.unwrap_or(DataType::Integer);
                    if self.functions.contains_key(name) {
                        self.error(logger, line, col, &format!("funcao '{}' ja declarada", name));
                    }
                    self.seq_counter += 1;
                    let seq = self.seq_counter;
                    self.pending_params.clear();
                    self.functions.insert(
                        name.to_string(),
                        FunctionSig {
                            params: Vec::new(),
                            return_type,
                            seq,
                        },
                    );
                    self.current_function = Some(name);
                    self.last_inserted = Some((name.to_string(), 0));
                }
            }
            Action::InsertParameter => {
                if let Some((name, line, col)) = self.pending_id.take() {
                    let data_type = self.pending_type.unwrap_or(DataType::Integer);
                    if self.pending_params.iter().any(|(n, _, _)| *n == name) {
                        self.error(
                            logger,
                            line,
                            col,
                            &format!("parametro '{}' ja declarado", name),
                        );
                    }
                    self.seq_counter += 1;
                    let seq = self.seq_counter;
                    self.pending_params.push((name, data_type, seq));
                    if let Some(function) = self.current_function {
                        if let Some(sig) = self.functions.get_mut(function) {
                            sig.params.push(data_type);
                        }
                    }
                    self.last_inserted = Some((name.to_string(), self.scopes.len() + 1));
                }
            }
            Action::InsertVariable => {
                if let Some((name, line, col)) = self.pending_id.take() {
                    let data_type = self.pending_type.unwrap_or(DataType::Integer);
                    if self.is_declared_in_scope(name) {
                        self.error(
                            logger,
                            line,
                            col,
                            &format!("identificador '{}' ja declarado", name),
                        );
                    }
                    self.seq_counter += 1;
                    let seq = self.seq_counter;
                    if let Some(scope) = self.scopes.last_mut() {
                        scope.insert(
                            name.to_string(),
                            VarInfo {
                                data_type,
                                category: Category::Variable,
                                seq,
                            },
                        );
                    }
                    self.last_inserted = Some((name.to_string(), self.scopes.len()));
                }
            }
            Action::PushName => {
                if let Some(id) = self.pending_id.take() {
                    self.name_stack.push(id);
                }
            }
            Action::CheckVariableDeclared => {
                if let Some((name, line, col)) = self.name_stack.pop() {
                    match self.lookup_variable(name) {
                        Some(data_type) => self.type_stack.push(data_type),
                        None => {
                            self.error(
                                logger,
                                line,
                                col,
                                &format!("variavel '{}' nao declarada", name),
                            );
                            self.type_stack.push(DataType::Integer);
                        }
                    }
                }
            }
            Action::CheckAssignment => {
                let source = self.type_stack.pop();
                let target = self.type_stack.pop();
                if let (Some(dst), Some(src)) = (target, source) {
                    if !compatible(dst, src) {
                        let (l, c) = (self.last_line, self.last_col);
                        self.error(
                            logger,
                            l,
                            c,
                            &format!(
                                "atribuicao incompativel: '{}' recebe '{}'",
                                type_name(dst),
                                type_name(src)
                            ),
                        );
                    }
                }
            }
            Action::PushNumberType => {
                let data_type = self.pending_num.take().unwrap_or(DataType::Integer);
                self.type_stack.push(data_type);
            }
            Action::CheckSameTypes => {
                let b = self.type_stack.pop();
                let a = self.type_stack.pop();
                match (a, b) {
                    (Some(x), Some(y)) => {
                        if x == DataType::Boolean || y == DataType::Boolean {
                            let (l, c) = (self.last_line, self.last_col);
                            self.error(logger, l, c, "operacao invalida com tipo booleano");
                            self.type_stack.push(DataType::Integer);
                        } else if x == DataType::Float || y == DataType::Float {
                            self.type_stack.push(DataType::Float);
                        } else {
                            self.type_stack.push(DataType::Integer);
                        }
                    }
                    (Some(x), None) => self.type_stack.push(x),
                    _ => self.type_stack.push(DataType::Integer),
                }
            }
            Action::BooleanResult => {
                self.type_stack.pop();
                self.type_stack.push(DataType::Boolean);
            }
            Action::CheckBooleanCondition => match self.type_stack.pop() {
                Some(DataType::Boolean) | None => {}
                Some(_) => {
                    let (l, c) = (self.last_line, self.last_col);
                    self.error(logger, l, c, "condicao de if/while deve ser booleana");
                }
            },
            Action::OpenArgs => {
                self.arg_stack.push(0);
            }
            Action::CountArgument => {
                if let Some(counter) = self.arg_stack.last_mut() {
                    *counter += 1;
                }
            }
            Action::CheckFunctionCall => {
                let arg_count = self.arg_stack.pop().unwrap_or(0);
                let mut args = Vec::with_capacity(arg_count);
                for _ in 0..arg_count {
                    if let Some(data_type) = self.type_stack.pop() {
                        args.push(data_type);
                    }
                }
                args.reverse();

                if let Some((name, line, col)) = self.name_stack.pop() {
                    let signature = self
                        .functions
                        .get(name)
                        .map(|sig| (sig.return_type, sig.params.clone()));
                    match signature {
                        None => {
                            self.error(
                                logger,
                                line,
                                col,
                                &format!("funcao '{}' nao declarada", name),
                            );
                            self.type_stack.push(DataType::Integer);
                        }
                        Some((return_type, params)) => {
                            if params.len() != arg_count {
                                self.error(
                                    logger,
                                    line,
                                    col,
                                    &format!(
                                        "funcao '{}' espera {} argumento(s), recebeu {}",
                                        name,
                                        params.len(),
                                        arg_count
                                    ),
                                );
                            } else {
                                for (i, (expected, received)) in
                                    params.iter().zip(args.iter()).enumerate()
                                {
                                    if !compatible(*expected, *received) {
                                        self.error(
                                            logger,
                                            line,
                                            col,
                                            &format!(
                                                "argumento {} de '{}' incompativel: esperado '{}', recebeu '{}'",
                                                i + 1,
                                                name,
                                                type_name(*expected),
                                                type_name(*received)
                                            ),
                                        );
                                    }
                                }
                            }
                            self.type_stack.push(return_type);
                        }
                    }
                } else {
                    self.type_stack.push(DataType::Integer);
                }
            }
            Action::CheckReturnType => {
                let expr_type = self.type_stack.pop().unwrap_or(DataType::Integer);
                let expected = self
                    .current_function
                    .and_then(|name| self.functions.get(name))
                    .map(|sig| sig.return_type);
                if let Some(expected) = expected {
                    if !compatible(expected, expr_type) {
                        let (l, c) = (self.last_line, self.last_col);
                        self.error(
                            logger,
                            l,
                            c,
                            &format!(
                                "retorno incompativel: funcao retorna '{}', expressao e '{}'",
                                type_name(expected),
                                type_name(expr_type)
                            ),
                        );
                    }
                }
                self.return_stack.push(true);
            }
            Action::CheckReturnRequired => {
                let returns = self.return_stack.pop().unwrap_or(false);
                if !returns {
                    if let Some(name) = self.current_function {
                        let (l, c) = (self.last_line, self.last_col);
                        self.error(
                            logger,
                            l,
                            c,
                            &format!("funcao '{}' pode nao retornar em todos os caminhos", name),
                        );
                    }
                }
            }
            Action::CheckMain => {
                if !self.functions.contains_key("main") {
                    let (l, c) = (self.last_line, self.last_col);
                    self.error(logger, l, c, "programa nao possui funcao 'main'");
                }
            }
            Action::PushFalse => {
                self.return_stack.push(false);
            }
            Action::EndPrint => {
                self.type_stack.pop();
                self.return_stack.push(false);
            }
            Action::ClearReturn => {
                self.return_stack.pop();
                self.return_stack.push(false);
            }
            Action::CombineReturn => {
                let b = self.return_stack.pop().unwrap_or(false);
                let a = self.return_stack.pop().unwrap_or(false);
                self.return_stack.push(a && b);
            }
        }

        let should_dump = match action {
            Action::InsertFunction | Action::InsertParameter | Action::InsertVariable => true,
            Action::CloseScope => !self.removed.is_empty(),
            _ => false,
        };
        if should_dump {
            self.dump_table(logger);
        }
    }

    fn error<W: Write>(&mut self, logger: &mut Logger<W>, line: usize, col: usize, detail: &str) {
        self.errors += 1;
        logger.semantic_error(line, col, detail);
    }

    fn lookup_variable(&self, name: &str) -> Option<DataType> {
        self.scopes
            .iter()
            .rev()
            .find_map(|scope| scope.get(name).map(|info| info.data_type))
    }

    fn dump_table<W: Write>(&mut self, logger: &mut Logger<W>) {
        let mut rows: Vec<(usize, usize, String, &str, &str, RowKind)> = Vec::new();
        for (name, sig) in &self.functions {
            let kind = self.row_kind(0, name);
            rows.push((0, sig.seq, name.clone(), "procedure", type_name(sig.return_type), kind));
        }
        for (level, scope) in self.scopes.iter().enumerate() {
            for (name, info) in scope {
                let kind = self.row_kind(level + 1, name);
                rows.push((
                    level + 1,
                    info.seq,
                    name.clone(),
                    category_name(info.category),
                    type_name(info.data_type),
                    kind,
                ));
            }
        }
        let pending_level = self.scopes.len() + 1;
        for (name, data_type, seq) in &self.pending_params {
            let kind = self.row_kind(pending_level, name);
            rows.push((
                pending_level,
                *seq,
                name.to_string(),
                "parametro",
                type_name(*data_type),
                kind,
            ));
        }
        for (level, name, category, data_type, seq) in &self.removed {
            rows.push((*level, *seq, name.clone(), category, data_type, RowKind::Removed));
        }
        rows.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| a.1.cmp(&b.1)));

        let lines: Vec<([String; 4], RowKind)> = rows
            .iter()
            .map(|(level, _seq, name, category, data_type, kind)| {
                (
                    [
                        name.clone(),
                        category.to_string(),
                        data_type.to_string(),
                        level.to_string(),
                    ],
                    *kind,
                )
            })
            .collect();
        logger.symbol_table(&lines);
        self.removed.clear();
    }

    fn row_kind(&self, level: usize, name: &str) -> RowKind {
        match &self.last_inserted {
            Some((new_name, new_level)) if new_name == name && *new_level == level => RowKind::New,
            _ => RowKind::Normal,
        }
    }

    fn is_declared_in_scope(&self, name: &str) -> bool {
        self.scopes.iter().any(|scope| scope.contains_key(name))
    }
}

fn category_name(category: Category) -> &'static str {
    match category {
        Category::Variable => "variavel",
        Category::Parameter => "parametro",
    }
}

fn compatible(target: DataType, source: DataType) -> bool {
    target == source || (target == DataType::Float && source == DataType::Integer)
}

fn type_name(data_type: DataType) -> &'static str {
    match data_type {
        DataType::Integer => "int",
        DataType::Float => "float",
        DataType::Boolean => "booleano",
    }
}
