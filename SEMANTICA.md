# Regras Semânticas

#### Todas implementadas

1. Variáveis devem ser declaradas antes do uso

```
int main() {
    int x;
    x = 10;        // OK: x foi declarada
    y = 5;         // ERRO: y não foi declarada
    return 0;
}
```

2. Variáveis não podem ser redeclaradas no mesmo escopo ou em escopos "maiores"

```
int main() {
    int x;
    int x;         // ERRO: x já declarada neste escopo
    {
        int x;     // ERRO: x já existe em escopo "maior"
    }
    return 0;
}
```

3. Atribuição a variáveis deve respeitar seu tipo declarado

```
int main() {
    int x;
    float y;
    x = 10;        // OK: int recebe int
    y = x;         // OK: float recebe int (coerção)
    x = y;         // ERRO: int não recebe float (estreitamento)
    return 0;
}
```

4. Funções devem ser declaradas antes do uso

```
int main() {
    int x;
    x = soma(1, 2);   // ERRO: soma() não foi declarada antes
    return 0;
}

int soma(int a, int b) {
    return a + b;
}
```

5. Não pode haver funções duplicadas

```
int soma(int a, int b) {
    return a + b;
}

int soma(int a, int b) {   // ERRO: soma() já foi declarada
    return a - b;
}

int main() {
    return 0;
}
```

6. Chamadas de função devem respeitar o número de argumentos definido

```
int soma(int a, int b) {
    return a + b;
}

int main() {
    int r;
    r = soma(1);          // ERRO: soma() espera 2 argumentos
    r = soma(1, 2, 3);    // ERRO: soma() espera 2 argumentos
    return 0;
}
```

7. Chamadas de função devem respeitar o tipo dos argumentos

```
int soma(int a, int b) {
    return a + b;
}

int main() {
    float f;
    int r;
    f = 1.5;
    r = soma(f, 2);       // ERRO: 1º argumento é float, esperado int
    return 0;
}
```

8. O retorno das funções deve respeitar o tipo definido

```
int meio() {
    float x;
    x = 1.5;
    return x;             // ERRO: função é int e retorno é float (estreitamento)
}

int main() {
    return 0;
}
```

(O contrário — uma função `float` retornando `int` — é aceito por coerção.)

9. Todas as funções devem retornar um valor

```
int soma(int a, int b) {
    int r;
    r = a + b;            // ERRO: função não possui return
}

int main() {
    return 0;
}
```

10. Condições do `if` ou `while` devem resolver a um booleano

```
int main() {
    int x;
    x = 5;
    if (x) {              // ERRO: x é int, não resolve a booleano
        x = 0;
    }
    while (x < 10) {      // OK: x < 10 resolve a booleano
        x = x + 1;
    }
    return 0;
}
```

11. Operações entre `int` e `float` são permitidas por coerção (resultado `float`)

```
int main() {
    int x;
    float y;
    float r;
    x = 1;
    y = 2.0;
    r = x + y;            // OK: int + float resulta em float (coerção)
    return 0;
}
```

12. Programas devem possuir uma função `main()` como ponto de entrada

```
int soma(int a, int b) {
    return a + b;
}
                         // ERRO: nenhuma função main() definida
```

# Gramática com as ações semânticas

```
<Program> ::= <FunctionList> #verificarMain#
<FunctionList> ::= <Function> <FunctionList'>
<FunctionList'> ::= <Function> <FunctionList'>
<FunctionList'> ::= ε
<Function> ::= <Type> id #inserirFuncao# ( <ParamListOpt> ) <Block> #verificarRetornoObrigatorio#
<ParamListOpt> ::= <ParamList>
<ParamListOpt> ::= ε
<ParamList> ::= <Param> <ParamList'>
<ParamList'> ::= , <Param> <ParamList'>
<ParamList'> ::= ε
<Param> ::= <Type> id #inserirParametro#
<Block> ::= { #abrirEscopo# <DeclListOpt> <StmtListOpt> } #removerEscopo#
<DeclListOpt> ::= <DeclList>
<DeclListOpt> ::= ε
<DeclList> ::= <VarDecl> <DeclList'>
<DeclList'> ::= <VarDecl> <DeclList'>
<DeclList'> ::= ε
<VarDecl> ::= <Type> id #inserirVariavel# ;
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
<AssignStmt> ::= id #empilharNome# #verificarVariavelDeclarada# = <Expr> #verificarAtribuicao# #empurraFalso# ;
<ReturnStmt> ::= return <Expr> #verificarTipoRetorno# ;
<PrintStmt> ::= print ( <Expr> ) #fimPrint# ;
<IfStmt> ::= if ( <Expr> #verificarCondicaoBooleana# ) <Stmt> <ElsePart>
<ElsePart> ::= else <Stmt> #combinaRetorno#
<ElsePart> ::= ε #anulaRetorno#
<WhileStmt> ::= while ( <Expr> #verificarCondicaoBooleana# ) <Stmt> #anulaRetorno#
<Expr> ::= <RelExpr>
<RelExpr> ::= <AddExpr> <RelExpr'>
<RelExpr'> ::= <RelOp> <AddExpr> #verificarTiposIguais# #resultadoBooleano#
<RelExpr'> ::= ε
<RelOp> ::= ==
<RelOp> ::= !=
<RelOp> ::= <
<RelOp> ::= >
<RelOp> ::= <=
<RelOp> ::= >=
<AddExpr> ::= <MulExpr> <AddExpr'>
<AddExpr'> ::= + <MulExpr> #verificarTiposIguais# <AddExpr'>
<AddExpr'> ::= - <MulExpr> #verificarTiposIguais# <AddExpr'>
<AddExpr'> ::= ε
<MulExpr> ::= <Factor> <MulExpr'>
<MulExpr'> ::= \* <Factor> #verificarTiposIguais# <MulExpr'>
<MulExpr'> ::= / <Factor> #verificarTiposIguais# <MulExpr'>
<MulExpr'> ::= ε
<Factor> ::= ( <Expr> )
<Factor> ::= id #empilharNome# <FactorTail>
<Factor> ::= num #empilharTipoNumero#
<FactorTail> ::= ( #abrirArgs# <ArgListOpt> ) #verificarChamadaFuncao#
<FactorTail> ::= #verificarVariavelDeclarada#
<ArgListOpt> ::= <ArgList>
<ArgListOpt> ::= ε
<ArgList> ::= <Expr> #contarArgumento# <ArgList'>
<ArgList'> ::= , <Expr> #contarArgumento# <ArgList'>
<ArgList'> ::= ε
<Type> ::= int
<Type> ::= float
```

### Ações semânticas

- `#abrirEscopo#` — ao consumir `{`, cria um novo nível de escopo e move para ele os
  parâmetros pendentes da função.
- `#removerEscopo#` — ao consumir `}`, remove (descarta) o escopo do topo, eliminando todas
  as variáveis/parâmetros daquele nível.
- `#inserirFuncao#` — insere a função na tabela com seu tipo
  de retorno; erro se já existe. Marca a função como a "função atual".
- `#inserirParametro#` — registra o parâmetro e acrescenta seu tipo à
  assinatura da função atual; erro se duplicado no escopo
- `#inserirVariavel#` — insere a variável no escopo atual; erro se já
  existe no mesmo escopo ou em escopo externo aberto.
- `#empilharNome#` — guarda o nome do `id` lido em `<Factor>`/`<AssignStmt>` até se saber se é
  uso de variável ou chamada de função.
- `#verificarVariavelDeclarada#` — busca o nome na tabela, erro se não declarado e
  empilha o tipo da variável para as verificações seguintes.
- `#verificarAtribuicao#` — compara o tipo do alvo com o tipo da expressão aplicando coerção
  (`float <- int` ok; `int <- float` erro).
- `#empilharTipoNumero#` — empilha o tipo do literal numérico (`int` ou `float`).
- `#verificarTiposIguais#` — combina os dois operandos de uma operação; resultado `float` se
  algum for `float`, senão `int`.
- `#resultadoBooleano#` — marca o resultado de uma comparação relacional como `booleano`.
- `#verificarCondicaoBooleana#` — garante que a condição de `if`/`while` é `booleana`.
- `#abrirArgs#` — inicia a contagem de argumentos de uma chamada.
- `#contarArgumento#` — incrementa o número de argumentos da chamada atual.
- `#verificarChamadaFuncao#` — busca a função (erro se não declarada), confere a
  quantidade e o tipo de cada argumento e empilha o tipo de retorno.
- `#verificarTipoRetorno#` — compara o tipo do `return` com o tipo declarado da função.
- `#verificarRetornoObrigatorio#` — garante que a função retorna em todos os caminhos.
- `#verificarMain#` — garante a existência de uma função `main`.
- `#empurraFalso#` — marca um comando de atribuição como "não garante retorno".
- `#fimPrint#` — descarta o tipo da expressão do `print` e marca o comando como "não garante
  retorno".
- `#anulaRetorno#` — usado no `while` e no `if` sem `else`: o ramo não garante retorno.
- `#combinaRetorno#` — combina os ramos `then`/`else` de um `if` (só garante retorno se ambos
  garantirem).
