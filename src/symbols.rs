use std::fmt;

pub const FIRST_TERMINAL: u8 = Terminal::Int.to_code();
pub const FIRST_NONTERMINAL: u8 = NonTerminal::Program.to_code();
pub const FIRST_ACTION: u8 = Action::OpenScope.to_code();
pub const START_SYMBOL: Symbol = Symbol::new(NonTerminal::Program.to_code());
pub const EOF_SYMBOL: Symbol = Symbol::new(Terminal::Eof.to_code());

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct Symbol(u8);

impl Symbol {
    pub const fn new(code: u8) -> Symbol {
        Symbol(code)
    }

    pub const fn code(self) -> u8 {
        self.0
    }

    pub const fn is_terminal(self) -> bool {
        self.0 < FIRST_NONTERMINAL
    }

    pub const fn is_nonterminal(self) -> bool {
        self.0 >= FIRST_NONTERMINAL && self.0 < FIRST_ACTION
    }

    pub const fn is_action(self) -> bool {
        self.0 >= FIRST_ACTION
    }

    pub fn name(self) -> &'static str {
        if self.is_terminal() {
            Terminal::from_code(self.0).name()
        } else if self.is_action() {
            Action::from_code(self.0).name()
        } else {
            NonTerminal::from_code(self.0).name()
        }
    }
}

impl fmt::Display for Symbol {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.name())
    }
}

#[derive(Debug, PartialEq)]
pub enum Terminal {
    Int,
    Float,
    If,
    Else,
    While,
    Return,
    Print,
    Id,
    Num,
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    Equals,
    NotEquals,
    LessThan,
    GreaterThan,
    LessThanOrEqual,
    GreaterThanOrEqual,
    StartParen,
    EndParen,
    StartBrace,
    EndBrace,
    Comma,
    Semicolon,
    Eof,
    Error,
}
impl Terminal {
    pub const fn to_code(&self) -> u8 {
        match self {
            Terminal::Int => 1,
            Terminal::Float => 2,
            Terminal::If => 3,
            Terminal::Else => 4,
            Terminal::While => 5,
            Terminal::Return => 6,
            Terminal::Print => 7,
            Terminal::Id => 8,
            Terminal::Num => 9,
            Terminal::Assign => 10,
            Terminal::Plus => 11,
            Terminal::Minus => 12,
            Terminal::Multiply => 13,
            Terminal::Divide => 14,
            Terminal::Equals => 15,
            Terminal::NotEquals => 16,
            Terminal::LessThan => 17,
            Terminal::GreaterThan => 18,
            Terminal::LessThanOrEqual => 19,
            Terminal::GreaterThanOrEqual => 20,
            Terminal::StartParen => 21,
            Terminal::EndParen => 22,
            Terminal::StartBrace => 23,
            Terminal::EndBrace => 24,
            Terminal::Comma => 25,
            Terminal::Semicolon => 26,
            Terminal::Eof => 27,
            Terminal::Error => 28,
        }
    }

    pub fn from_code(code: u8) -> Terminal {
        match code {
            1 => Terminal::Int,
            2 => Terminal::Float,
            3 => Terminal::If,
            4 => Terminal::Else,
            5 => Terminal::While,
            6 => Terminal::Return,
            7 => Terminal::Print,
            8 => Terminal::Id,
            9 => Terminal::Num,
            10 => Terminal::Assign,
            11 => Terminal::Plus,
            12 => Terminal::Minus,
            13 => Terminal::Multiply,
            14 => Terminal::Divide,
            15 => Terminal::Equals,
            16 => Terminal::NotEquals,
            17 => Terminal::LessThan,
            18 => Terminal::GreaterThan,
            19 => Terminal::LessThanOrEqual,
            20 => Terminal::GreaterThanOrEqual,
            21 => Terminal::StartParen,
            22 => Terminal::EndParen,
            23 => Terminal::StartBrace,
            24 => Terminal::EndBrace,
            25 => Terminal::Comma,
            26 => Terminal::Semicolon,
            27 => Terminal::Eof,
            _ => Terminal::Error,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Terminal::Int => "int",
            Terminal::Float => "float",
            Terminal::If => "if",
            Terminal::Else => "else",
            Terminal::While => "while",
            Terminal::Return => "return",
            Terminal::Print => "print",
            Terminal::Id => "id",
            Terminal::Num => "num",
            Terminal::Assign => "=",
            Terminal::Plus => "+",
            Terminal::Minus => "-",
            Terminal::Multiply => "*",
            Terminal::Divide => "/",
            Terminal::Equals => "==",
            Terminal::NotEquals => "!=",
            Terminal::LessThan => "<",
            Terminal::GreaterThan => ">",
            Terminal::LessThanOrEqual => "<=",
            Terminal::GreaterThanOrEqual => ">=",
            Terminal::StartParen => "(",
            Terminal::EndParen => ")",
            Terminal::StartBrace => "{",
            Terminal::EndBrace => "}",
            Terminal::Comma => ",",
            Terminal::Semicolon => ";",
            Terminal::Eof => "$",
            Terminal::Error => "ERRO",
        }
    }

    pub fn as_symbol(&self) -> Symbol {
        Symbol::new(self.to_code())
    }

    pub fn keyword_to_terminal(keyword: &str) -> Option<Terminal> {
        match keyword {
            "int" => Some(Terminal::Int),
            "float" => Some(Terminal::Float),
            "if" => Some(Terminal::If),
            "else" => Some(Terminal::Else),
            "while" => Some(Terminal::While),
            "return" => Some(Terminal::Return),
            "print" => Some(Terminal::Print),
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum NonTerminal {
    Program,
    FunctionList,
    FunctionListPrime,
    Function,
    ParamListOpt,
    ParamList,
    ParamListPrime,
    Param,
    Block,
    DeclListOpt,
    DeclList,
    DeclListPrime,
    VarDecl,
    StmtListOpt,
    StmtList,
    StmtListPrime,
    Stmt,
    AssignStmt,
    ReturnStmt,
    PrintStmt,
    IfStmt,
    ElsePart,
    WhileStmt,
    Expr,
    RelExpr,
    RelExprPrime,
    RelOp,
    AddExpr,
    AddExprPrime,
    MulExpr,
    MulExprPrime,
    Factor,
    FactorTail,
    ArgListOpt,
    ArgList,
    ArgListPrime,
    Type,
}
impl NonTerminal {
    pub const fn to_code(&self) -> u8 {
        match self {
            NonTerminal::Program => 29,
            NonTerminal::FunctionList => 30,
            NonTerminal::FunctionListPrime => 31,
            NonTerminal::Function => 32,
            NonTerminal::ParamListOpt => 33,
            NonTerminal::ParamList => 34,
            NonTerminal::ParamListPrime => 35,
            NonTerminal::Param => 36,
            NonTerminal::Block => 37,
            NonTerminal::DeclListOpt => 38,
            NonTerminal::DeclList => 39,
            NonTerminal::DeclListPrime => 40,
            NonTerminal::VarDecl => 41,
            NonTerminal::StmtListOpt => 42,
            NonTerminal::StmtList => 43,
            NonTerminal::StmtListPrime => 44,
            NonTerminal::Stmt => 45,
            NonTerminal::AssignStmt => 46,
            NonTerminal::ReturnStmt => 47,
            NonTerminal::PrintStmt => 48,
            NonTerminal::IfStmt => 49,
            NonTerminal::ElsePart => 50,
            NonTerminal::WhileStmt => 51,
            NonTerminal::Expr => 52,
            NonTerminal::RelExpr => 53,
            NonTerminal::RelExprPrime => 54,
            NonTerminal::RelOp => 55,
            NonTerminal::AddExpr => 56,
            NonTerminal::AddExprPrime => 57,
            NonTerminal::MulExpr => 58,
            NonTerminal::MulExprPrime => 59,
            NonTerminal::Factor => 60,
            NonTerminal::FactorTail => 61,
            NonTerminal::ArgListOpt => 62,
            NonTerminal::ArgList => 63,
            NonTerminal::ArgListPrime => 64,
            NonTerminal::Type => 65,
        }
    }

    pub fn from_code(code: u8) -> NonTerminal {
        match code {
            29 => NonTerminal::Program,
            30 => NonTerminal::FunctionList,
            31 => NonTerminal::FunctionListPrime,
            32 => NonTerminal::Function,
            33 => NonTerminal::ParamListOpt,
            34 => NonTerminal::ParamList,
            35 => NonTerminal::ParamListPrime,
            36 => NonTerminal::Param,
            37 => NonTerminal::Block,
            38 => NonTerminal::DeclListOpt,
            39 => NonTerminal::DeclList,
            40 => NonTerminal::DeclListPrime,
            41 => NonTerminal::VarDecl,
            42 => NonTerminal::StmtListOpt,
            43 => NonTerminal::StmtList,
            44 => NonTerminal::StmtListPrime,
            45 => NonTerminal::Stmt,
            46 => NonTerminal::AssignStmt,
            47 => NonTerminal::ReturnStmt,
            48 => NonTerminal::PrintStmt,
            49 => NonTerminal::IfStmt,
            50 => NonTerminal::ElsePart,
            51 => NonTerminal::WhileStmt,
            52 => NonTerminal::Expr,
            53 => NonTerminal::RelExpr,
            54 => NonTerminal::RelExprPrime,
            55 => NonTerminal::RelOp,
            56 => NonTerminal::AddExpr,
            57 => NonTerminal::AddExprPrime,
            58 => NonTerminal::MulExpr,
            59 => NonTerminal::MulExprPrime,
            60 => NonTerminal::Factor,
            61 => NonTerminal::FactorTail,
            62 => NonTerminal::ArgListOpt,
            63 => NonTerminal::ArgList,
            64 => NonTerminal::ArgListPrime,
            _ => NonTerminal::Type,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            NonTerminal::Program => "Program",
            NonTerminal::FunctionList => "FunctionList",
            NonTerminal::FunctionListPrime => "FunctionList'",
            NonTerminal::Function => "Function",
            NonTerminal::ParamListOpt => "ParamListOpt",
            NonTerminal::ParamList => "ParamList",
            NonTerminal::ParamListPrime => "ParamList'",
            NonTerminal::Param => "Param",
            NonTerminal::Block => "Block",
            NonTerminal::DeclListOpt => "DeclListOpt",
            NonTerminal::DeclList => "DeclList",
            NonTerminal::DeclListPrime => "DeclList'",
            NonTerminal::VarDecl => "VarDecl",
            NonTerminal::StmtListOpt => "StmtListOpt",
            NonTerminal::StmtList => "StmtList",
            NonTerminal::StmtListPrime => "StmtList'",
            NonTerminal::Stmt => "Stmt",
            NonTerminal::AssignStmt => "AssignStmt",
            NonTerminal::ReturnStmt => "ReturnStmt",
            NonTerminal::PrintStmt => "PrintStmt",
            NonTerminal::IfStmt => "IfStmt",
            NonTerminal::ElsePart => "ElsePart",
            NonTerminal::WhileStmt => "WhileStmt",
            NonTerminal::Expr => "Expr",
            NonTerminal::RelExpr => "RelExpr",
            NonTerminal::RelExprPrime => "RelExpr'",
            NonTerminal::RelOp => "RelOp",
            NonTerminal::AddExpr => "AddExpr",
            NonTerminal::AddExprPrime => "AddExpr'",
            NonTerminal::MulExpr => "MulExpr",
            NonTerminal::MulExprPrime => "MulExpr'",
            NonTerminal::Factor => "Factor",
            NonTerminal::FactorTail => "FactorTail",
            NonTerminal::ArgListOpt => "ArgListOpt",
            NonTerminal::ArgList => "ArgList",
            NonTerminal::ArgListPrime => "ArgList'",
            NonTerminal::Type => "Type",
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Action {
    OpenScope,
    CloseScope,
    InsertFunction,
    InsertParameter,
    InsertVariable,
    PushName,
    CheckVariableDeclared,
    CheckAssignment,
    PushNumberType,
    CheckSameTypes,
    BooleanResult,
    CheckBooleanCondition,
    OpenArgs,
    CountArgument,
    CheckFunctionCall,
    CheckReturnType,
    CheckReturnRequired,
    CheckMain,
    PushFalse,
    EndPrint,
    ClearReturn,
    CombineReturn,
}

impl Action {
    pub const fn to_code(&self) -> u8 {
        match self {
            Action::OpenScope => 66,
            Action::CloseScope => 67,
            Action::InsertFunction => 68,
            Action::InsertParameter => 69,
            Action::InsertVariable => 70,
            Action::PushName => 71,
            Action::CheckVariableDeclared => 72,
            Action::CheckAssignment => 73,
            Action::PushNumberType => 74,
            Action::CheckSameTypes => 75,
            Action::BooleanResult => 76,
            Action::CheckBooleanCondition => 77,
            Action::OpenArgs => 78,
            Action::CountArgument => 79,
            Action::CheckFunctionCall => 80,
            Action::CheckReturnType => 81,
            Action::CheckReturnRequired => 82,
            Action::CheckMain => 83,
            Action::PushFalse => 84,
            Action::EndPrint => 85,
            Action::ClearReturn => 86,
            Action::CombineReturn => 87,
        }
    }

    pub fn from_code(code: u8) -> Action {
        match code {
            66 => Action::OpenScope,
            67 => Action::CloseScope,
            68 => Action::InsertFunction,
            69 => Action::InsertParameter,
            70 => Action::InsertVariable,
            71 => Action::PushName,
            72 => Action::CheckVariableDeclared,
            73 => Action::CheckAssignment,
            74 => Action::PushNumberType,
            75 => Action::CheckSameTypes,
            76 => Action::BooleanResult,
            77 => Action::CheckBooleanCondition,
            78 => Action::OpenArgs,
            79 => Action::CountArgument,
            80 => Action::CheckFunctionCall,
            81 => Action::CheckReturnType,
            82 => Action::CheckReturnRequired,
            83 => Action::CheckMain,
            84 => Action::PushFalse,
            85 => Action::EndPrint,
            86 => Action::ClearReturn,
            _ => Action::CombineReturn,
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            Action::OpenScope => "#abrirEscopo#",
            Action::CloseScope => "#removerEscopo#",
            Action::InsertFunction => "#inserirFuncao#",
            Action::InsertParameter => "#inserirParametro#",
            Action::InsertVariable => "#inserirVariavel#",
            Action::PushName => "#empilharNome#",
            Action::CheckVariableDeclared => "#verificarVariavelDeclarada#",
            Action::CheckAssignment => "#verificarAtribuicao#",
            Action::PushNumberType => "#empilharTipoNumero#",
            Action::CheckSameTypes => "#verificarTiposIguais#",
            Action::BooleanResult => "#resultadoBooleano#",
            Action::CheckBooleanCondition => "#verificarCondicaoBooleana#",
            Action::OpenArgs => "#abrirArgs#",
            Action::CountArgument => "#contarArgumento#",
            Action::CheckFunctionCall => "#verificarChamadaFuncao#",
            Action::CheckReturnType => "#verificarTipoRetorno#",
            Action::CheckReturnRequired => "#verificarRetornoObrigatorio#",
            Action::CheckMain => "#verificarMain#",
            Action::PushFalse => "#empurraFalso#",
            Action::EndPrint => "#fimPrint#",
            Action::ClearReturn => "#anulaRetorno#",
            Action::CombineReturn => "#combinaRetorno#",
        }
    }
}
