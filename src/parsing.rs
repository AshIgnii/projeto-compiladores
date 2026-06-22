use crate::symbols::{FIRST_NONTERMINAL, FIRST_TERMINAL, Symbol};

static PARSING_TABLE: [[u8; 28]; 37] = [
    [
        1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // Program
    [
        2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // FunctionList
    [
        3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 4, 0,
    ], // FunctionListPrime
    [
        5, 5, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // Function
    [
        6, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 7, 0, 0, 0, 0, 0, 0,
    ], // ParamListOpt
    [
        8, 8, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // ParamList
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 10, 0, 0, 9, 0, 0, 0,
    ], // ParamListPrime
    [
        11, 11, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // Param
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 12, 0, 0, 0, 0, 0,
    ], // Block
    [
        13, 13, 14, 0, 14, 14, 14, 14, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 14, 14, 0, 0, 0, 0,
    ], // DeclListOpt
    [
        15, 15, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // DeclList
    [
        16, 16, 17, 0, 17, 17, 17, 17, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 17, 17, 0, 0, 0, 0,
    ], // DeclListPrime
    [
        18, 18, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // VarDecl
    [
        0, 0, 19, 0, 19, 19, 19, 19, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 19, 20, 0, 0, 0, 0,
    ], // StmtListOpt
    [
        0, 0, 21, 0, 21, 21, 21, 21, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 21, 0, 0, 0, 0, 0,
    ], // StmtList
    [
        0, 0, 22, 0, 22, 22, 22, 22, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 22, 23, 0, 0, 0, 0,
    ], // StmtListPrime
    [
        0, 0, 25, 0, 26, 28, 27, 24, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 29, 0, 0, 0, 0, 0,
    ], // Stmt
    [
        0, 0, 0, 0, 0, 0, 0, 30, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // AssignStmt
    [
        0, 0, 0, 0, 0, 31, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // ReturnStmt
    [
        0, 0, 0, 0, 0, 0, 32, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // PrintStmt
    [
        0, 0, 33, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // IfStmt
    [
        0, 0, 35, 34, 35, 35, 35, 35, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 35, 35, 0, 0, 0, 0,
    ], // ElsePart
    [
        0, 0, 0, 0, 36, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // WhileStmt
    [
        0, 0, 0, 0, 0, 0, 0, 37, 37, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 37, 0, 0, 0, 0, 0, 0, 0,
    ], // Expr
    [
        0, 0, 0, 0, 0, 0, 0, 38, 38, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 38, 0, 0, 0, 0, 0, 0, 0,
    ], // RelExpr
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 39, 39, 39, 39, 39, 39, 0, 40, 0, 0, 40, 40, 0, 0,
    ], // RelExprPrime
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 41, 42, 43, 44, 45, 46, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // RelOp
    [
        0, 0, 0, 0, 0, 0, 0, 47, 47, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 47, 0, 0, 0, 0, 0, 0, 0,
    ], // AddExpr
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 48, 49, 0, 0, 50, 50, 50, 50, 50, 50, 0, 50, 0, 0, 50, 50, 0,
        0,
    ], // AddExprPrime
    [
        0, 0, 0, 0, 0, 0, 0, 51, 51, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 51, 0, 0, 0, 0, 0, 0, 0,
    ], // MulExpr
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 54, 54, 52, 53, 54, 54, 54, 54, 54, 54, 0, 54, 0, 0, 54, 54,
        0, 0,
    ], // MulExprPrime
    [
        0, 0, 0, 0, 0, 0, 0, 56, 57, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 55, 0, 0, 0, 0, 0, 0, 0,
    ], // Factor
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 59, 59, 59, 59, 59, 59, 59, 59, 59, 59, 58, 59, 0, 0, 59, 59,
        0, 0,
    ], // FactorTail
    [
        0, 0, 0, 0, 0, 0, 0, 60, 60, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 60, 61, 0, 0, 0, 0, 0, 0,
    ], // ArgListOpt
    [
        0, 0, 0, 0, 0, 0, 0, 62, 62, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 62, 0, 0, 0, 0, 0, 0, 0,
    ], // ArgList
    [
        0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 64, 0, 0, 63, 0, 0, 0,
    ], // ArgListPrime
    [
        65, 66, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    ], // Type
];

static PRODUCTIONS: [&[u8]; 67] = [
    &[],
    &[83, 30],                // Program -> FunctionList #verificarMain#
    &[31, 32],                // FunctionList -> Function FunctionList'
    &[31, 32],                // FunctionList' -> Function FunctionList'
    &[],                      // FunctionList' -> e
    &[82, 37, 22, 33, 21, 68, 8, 65], // Function -> Type id #inserirFuncao# ( ParamListOpt ) Block #verificarRetornoObrigatorio#
    &[34],                    // ParamListOpt -> ParamList
    &[],                      // ParamListOpt -> e
    &[35, 36],                // ParamList -> Param ParamList'
    &[35, 36, 25],            // ParamList' -> , Param ParamList'
    &[],                      // ParamList' -> e
    &[69, 8, 65],             // Param -> Type id #inserirParametro#
    &[67, 24, 42, 38, 66, 23], // Block -> { #abrirEscopo# DeclListOpt StmtListOpt } #removerEscopo#
    &[39],                    // DeclListOpt -> DeclList
    &[],                      // DeclListOpt -> e
    &[40, 41],                // DeclList -> VarDecl DeclList'
    &[40, 41],                // DeclList' -> VarDecl DeclList'
    &[],                      // DeclList' -> e
    &[26, 70, 8, 65],         // VarDecl -> Type id #inserirVariavel# ;
    &[43],                    // StmtListOpt -> StmtList
    &[],                      // StmtListOpt -> e
    &[44, 45],                // StmtList -> Stmt StmtList'
    &[44, 45],                // StmtList' -> Stmt StmtList'
    &[],                      // StmtList' -> e
    &[46],                    // Stmt -> AssignStmt
    &[49],                    // Stmt -> IfStmt
    &[51],                    // Stmt -> WhileStmt
    &[48],                    // Stmt -> PrintStmt
    &[47],                    // Stmt -> ReturnStmt
    &[37],                    // Stmt -> Block
    &[26, 84, 73, 52, 10, 72, 71, 8], // AssignStmt -> id #empilharNome# #verificarVariavelDeclarada# = Expr #verificarAtribuicao# #empurraFalso# ;
    &[26, 81, 52, 6],         // ReturnStmt -> return Expr #verificarTipoRetorno# ;
    &[26, 85, 22, 52, 21, 7], // PrintStmt -> print ( Expr ) #fimPrint# ;
    &[50, 45, 22, 77, 52, 21, 3], // IfStmt -> if ( Expr #verificarCondicaoBooleana# ) Stmt ElsePart
    &[87, 45, 4],             // ElsePart -> else Stmt #combinaRetorno#
    &[86],                    // ElsePart -> e #anulaRetorno#
    &[86, 45, 22, 77, 52, 21, 5], // WhileStmt -> while ( Expr #verificarCondicaoBooleana# ) Stmt #anulaRetorno#
    &[53],                    // Expr -> RelExpr
    &[54, 56],                // RelExpr -> AddExpr RelExpr'
    &[76, 75, 56, 55],        // RelExpr' -> RelOp AddExpr #verificarTiposIguais# #resultadoBooleano#
    &[],                      // RelExpr' -> e
    &[15],                    // RelOp -> ==
    &[16],                    // RelOp -> !=
    &[17],                    // RelOp -> <
    &[18],                    // RelOp -> >
    &[19],                    // RelOp -> <=
    &[20],                    // RelOp -> >=
    &[57, 58],                // AddExpr -> MulExpr AddExpr'
    &[57, 75, 58, 11],        // AddExpr' -> + MulExpr #verificarTiposIguais# AddExpr'
    &[57, 75, 58, 12],        // AddExpr' -> - MulExpr #verificarTiposIguais# AddExpr'
    &[],                      // AddExpr' -> e
    &[59, 60],                // MulExpr -> Factor MulExpr'
    &[59, 75, 60, 13],        // MulExpr' -> * Factor #verificarTiposIguais# MulExpr'
    &[59, 75, 60, 14],        // MulExpr' -> / Factor #verificarTiposIguais# MulExpr'
    &[],                      // MulExpr' -> e
    &[22, 52, 21],            // Factor -> ( Expr )
    &[61, 71, 8],             // Factor -> id #empilharNome# FactorTail
    &[74, 9],                 // Factor -> num #empilharTipoNumero#
    &[80, 22, 62, 78, 21],    // FactorTail -> ( #abrirArgs# ArgListOpt ) #verificarChamadaFuncao#
    &[72],                    // FactorTail -> e #verificarVariavelDeclarada#
    &[63],                    // ArgListOpt -> ArgList
    &[],                      // ArgListOpt -> e
    &[64, 79, 52],            // ArgList -> Expr #contarArgumento# ArgList'
    &[64, 79, 52, 25],        // ArgList' -> , Expr #contarArgumento# ArgList'
    &[],                      // ArgList' -> e
    &[1],                     // Type -> int
    &[2],                     // Type -> float
];

static PANIC_FOLLOW: [&[u8]; 37] = [
    &[27],                                                 // Program       { $ }
    &[27],                                                 // FunctionList  { $ }
    &[27],                                                 // FunctionList' { $ }
    &[1, 2, 27],                                           // Function      { int float $ }
    &[22],                                                 // ParamListOpt  { ) }
    &[22],                                                 // ParamList     { ) }
    &[22],                                                 // ParamList'    { ) }
    &[22, 25],                                             // Param         { ) , }
    &[8, 3, 5, 7, 6, 23, 24, 4, 1, 2, 27], // Block      { id if while print return { } else int float $ }
    &[8, 3, 5, 7, 6, 23, 24],              // DeclListOpt   { id if while print return { } }
    &[8, 3, 5, 7, 6, 23, 24],              // DeclList      { id if while print return { } }
    &[8, 3, 5, 7, 6, 23, 24],              // DeclList'     { id if while print return { } }
    &[8, 3, 5, 7, 6, 23, 24, 1, 2], // VarDecl       { id if while print return { } int float }
    &[24],                          // StmtListOpt   { } }
    &[24],                          // StmtList      { } }
    &[24],                          // StmtList'     { } }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // Stmt          { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // AssignStmt    { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // ReturnStmt    { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // PrintStmt     { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // IfStmt        { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // ElsePart      { id if while print return { } else }
    &[8, 3, 5, 7, 6, 23, 24, 4],    // WhileStmt     { id if while print return { } else }
    &[22, 25, 26],                  // Expr          { ) , ; }
    &[22, 25, 26],                  // RelExpr       { ) , ; }
    &[22, 25, 26],                  // RelExpr'      { ) , ; }
    &[21, 8, 9],                    // RelOp         { ( id num }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20], // AddExpr    { ) , ; == != < > <= >= }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20], // AddExpr'   { ) , ; == != < > <= >= }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20, 11, 12], // MulExpr  { ) , ; == != < > <= >= + - }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20, 11, 12], // MulExpr' { ) , ; == != < > <= >= + - }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20, 11, 12, 13, 14], // Factor     { ) , ; == != < > <= >= + - * / }
    &[22, 25, 26, 15, 16, 17, 18, 19, 20, 11, 12, 13, 14], // FactorTail  { ) , ; == != < > <= >= + - * / }
    &[22],                                                 // ArgListOpt    { ) }
    &[22],                                                 // ArgList       { ) }
    &[22],                                                 // ArgList'      { ) }
    &[8],                                                  // Type          { id }
];

fn as_symbols(codes: &'static [u8]) -> &'static [Symbol] {
    unsafe { std::mem::transmute(codes) }
}

pub fn entry(nonterminal: Symbol, lookahead: Symbol) -> u8 {
    PARSING_TABLE[(nonterminal.code() - FIRST_NONTERMINAL) as usize]
        [(lookahead.code() - FIRST_TERMINAL) as usize]
}

pub fn production_rhs(number: u8) -> &'static [Symbol] {
    as_symbols(PRODUCTIONS[number as usize])
}

pub fn follow(nonterminal: Symbol) -> &'static [Symbol] {
    as_symbols(PANIC_FOLLOW[(nonterminal.code() - FIRST_NONTERMINAL) as usize])
}
