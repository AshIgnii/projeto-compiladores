```
<Program> ::= <FunctionList>
<FunctionList> ::= <Function> <FunctionList'>
<FunctionList'> ::= <Function> <FunctionList'>
<FunctionList'> ::= ε
<Function> ::= <Type> id ( <ParamListOpt> ) <Block>
<ParamListOpt> ::= <ParamList>
<ParamListOpt> ::= ε
<ParamList> ::= <Param> <ParamList'>
<ParamList'> ::= , <Param> <ParamList'>
<ParamList'> ::= ε
<Param> ::= <Type> id
<Block> ::= { <DeclListOpt> <StmtListOpt> }
<DeclListOpt> ::= <DeclList>
<DeclListOpt> ::= ε
<DeclList> ::= <VarDecl> <DeclList'>
<DeclList'> ::= <VarDecl> <DeclList'>
<DeclList'> ::= ε
<VarDecl> ::= <Type> id ;
<StmtListOpt> ::= <StmtList>
<StmtListOpt> ::= ε
<StmtList> ::= <Stmt> <StmtList'>
<StmtList'> ::= <Stmt> <StmtList'>
<StmtList'> ::= ε
<Stmt> ::= <AssignStmt>
<Stmt> ::= <IfStmt>
<Stmt> ::= <WhileStmt>
<Stmt> ::= <PrintStmt>
<Stmt> ::= <ReturnStmt>
<Stmt> ::= <Block>
<AssignStmt> ::= id = <Expr> ;
<ReturnStmt> ::= return <Expr> ;
<PrintStmt> ::= print ( <Expr> ) ;
<IfStmt> ::= if ( <Expr> ) <Stmt> <ElsePart>
<ElsePart> ::= else <Stmt>
<ElsePart> ::= ε
<WhileStmt> ::= while ( <Expr> ) <Stmt>
<Expr> ::= <RelExpr>
<RelExpr> ::= <AddExpr> <RelExpr'>
<RelExpr'> ::= <RelOp> <AddExpr>
<RelExpr'> ::= ε
<RelOp> ::= ==
<RelOp> ::= !=
<RelOp> ::= <
<RelOp> ::= >
<RelOp> ::= <=
<RelOp> ::= >=
<AddExpr> ::= <MulExpr> <AddExpr'>
<AddExpr'> ::= + <MulExpr> <AddExpr'>
<AddExpr'> ::= - <MulExpr> <AddExpr'>
<AddExpr'> ::= ε
<MulExpr> ::= <Factor> <MulExpr'>
<MulExpr'> ::= \* <Factor> <MulExpr'>
<MulExpr'> ::= / <Factor> <MulExpr'>
<MulExpr'> ::= ε
<Factor> ::= ( <Expr> )
<Factor> ::= id <FactorTail>
<Factor> ::= num
<FactorTail> ::= ( <ArgListOpt> )
<FactorTail> ::= ε
<ArgListOpt> ::= <ArgList>
<ArgListOpt> ::= ε
<ArgList> ::= <Expr> <ArgList'>
<ArgList'> ::= , <Expr> <ArgList'>
<ArgList'> ::= ε
<Type> ::= int
<Type> ::= float
```

| NT | First | Follow |
|----|-------|--------|
| <Program> | int \| float | $ |
| <FunctionList> | int \| float | $ |
| <FunctionList'> | int \| float \| ε | $ |
| <Function> | int \| float | int \| float \| $ |
| <ParamListOpt> | int \| float \| ε | ) |
| <ParamList> | int \| float | ) |
| <ParamList'> | , \| ε | ) |
| <Param> | int \| float | ) \| , |
| <Block> | { | id \| if \| while \| print \| return \| { \| } \| else \| int \| float \| $ |
| <DeclListOpt> | int \| float \| ε | id \| if \| while \| print \| return \| { \| } |
| <DeclList> | int \| float | id \| if \| while \| print \| return \| { \| } |
| <DeclList'> | int \| float \| ε | id \| if \| while \| print \| return \| { \| } |
| <VarDecl> | int \| float | id \| if \| while \| print \| return \| { \| } \| int \| float |
| <StmtListOpt> | id \| if \| while \| print \| return \| { \| ε | } |
| <StmtList> | id \| if \| while \| print \| return \| { | } |
| <StmtList'> | id \| if \| while \| print \| return \| { \| ε | } |
| <Stmt> | id \| if \| while \| print \| return \| { | id \| if \| while \| print \| return \| { \| } \| else |
| <AssignStmt> | id | id \| if \| while \| print \| return \| { \| } \| else |
| <ReturnStmt> | return | id \| if \| while \| print \| return \| { \| } \| else |
| <PrintStmt> | print | id \| if \| while \| print \| return \| { \| } \| else |
| <IfStmt> | if | id \| if \| while \| print \| return \| { \| } \| else |
| <ElsePart> | else \| ε | id \| if \| while \| print \| return \| { \| } \| else |
| <WhileStmt> | while | id \| if \| while \| print \| return \| { \| } \| else |
| <Expr> | ( \| id \| num | ) \| , \| ; |
| <RelExpr> | ( \| id \| num | ) \| , \| ; |
| <RelExpr'> | == \| != \| < \| > \| <= \| >= \| ε | ) \| , \| ; |
| <RelOp> | == \| != \| < \| > \| <= \| >= | ( \| id \| num |
| <AddExpr> | ( \| id \| num | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= |
| <AddExpr'> | + \| - \| ε | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= |
| <MulExpr> | ( \| id \| num | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= \| + \| - |
| <MulExpr'> | \* \| / \| ε | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= \| + \| - |
| <Factor> | ( \| id \| num | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= \| + \| - \| \* \| / |
| <FactorTail> | ( \| ε | ) \| , \| ; \| == \| != \| < \| > \| <= \| >= \| + \| - \| \* \| / |
| <ArgListOpt> | ( \| id \| num \| ε | ) |
| <ArgList> | ( \| id \| num | ) |
| <ArgList'> | , \| ε | ) |
| <Type> | int \| float | id |

| Parsing | id | ( | ) | , | { | } | ; | return | print | if | else | while | == | != | < | > | <= | >= | + | - | * | / | num | int | float | $ |
|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|----|
| <Program> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 1 | 1 |  |
| <FunctionList> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 2 | 2 |  |
| <FunctionList'> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 3 | 3 | 4 |
| <Function> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 5 | 5 |  |
| <ParamListOpt> |  |  | 7 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 6 | 6 |  |
| <ParamList> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 8 | 8 |  |
| <ParamList'> |  |  | 10 | 9 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <Param> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 11 | 11 |  |
| <Block> |  |  |  |  | 12 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <DeclListOpt> | 14 |  |  |  | 14 | 14 |  | 14 | 14 | 14 |  | 14 |  |  |  |  |  |  |  |  |  |  |  | 13 | 13 |  |
| <DeclList> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 15 | 15 |  |
| <DeclList'> | 17 |  |  |  | 17 | 17 |  | 17 | 17 | 17 |  | 17 |  |  |  |  |  |  |  |  |  |  |  | 16 | 16 |  |
| <VarDecl> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 18 | 18 |  |
| <StmtListOpt> | 19 |  |  |  | 19 | 20 |  | 19 | 19 | 19 |  | 19 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <StmtList> | 21 |  |  |  | 21 |  |  | 21 | 21 | 21 |  | 21 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <StmtList'> | 22 |  |  |  | 22 | 23 |  | 22 | 22 | 22 |  | 22 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <Stmt> | 24 |  |  |  | 29 |  |  | 28 | 27 | 25 |  | 26 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <AssignStmt> | 30 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <ReturnStmt> |  |  |  |  |  |  |  | 31 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <PrintStmt> |  |  |  |  |  |  |  |  | 32 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <IfStmt> |  |  |  |  |  |  |  |  |  | 33 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <ElsePart> | 35 |  |  |  | 35 | 35 |  | 35 | 35 | 35 | 34 | 35 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <WhileStmt> |  |  |  |  |  |  |  |  |  |  |  | 36 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <Expr> | 37 | 37 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 37 |  |  |  |
| <RelExpr> | 38 | 38 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 38 |  |  |  |
| <RelExpr'> |  |  | 40 | 40 |  |  | 40 |  |  |  |  |  | 39 | 39 | 39 | 39 | 39 | 39 |  |  |  |  |  |  |  |  |
| <RelOp> |  |  |  |  |  |  |  |  |  |  |  |  | 41 | 42 | 43 | 44 | 45 | 46 |  |  |  |  |  |  |  |  |
| <AddExpr> | 47 | 47 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 47 |  |  |  |
| <AddExpr'> |  |  | 50 | 50 |  |  | 50 |  |  |  |  |  | 50 | 50 | 50 | 50 | 50 | 50 | 48 | 49 |  |  |  |  |  |  |
| <MulExpr> | 51 | 51 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 51 |  |  |  |
| <MulExpr'> |  |  | 54 | 54 |  |  | 54 |  |  |  |  |  | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 54 | 52 | 53 |  |  |  |  |
| <Factor> | 56 | 55 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 57 |  |  |  |
| <FactorTail> |  | 58 | 59 | 59 |  |  | 59 |  |  |  |  |  | 59 | 59 | 59 | 59 | 59 | 59 | 59 | 59 | 59 | 59 |  |  |  |  |
| <ArgListOpt> | 60 | 60 | 61 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 60 |  |  |  |
| <ArgList> | 62 | 62 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 62 |  |  |  |
| <ArgList'> |  |  | 64 | 63 |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| <Type> |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | 65 | 66 |  |
