# Compilador

Analisador lexico e sintatico (LL(1)) implementado em Rust

---

## Compilacao

### Pre-requisito: Rust 1.85 ou superior (2024)

**Linux / macOS:**
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Windows:**

Baixe e execute o instalador em: https://www.rust-lang.org/tools/install

Após a instalacao, reinicie o terminal e verifique com:
```bash
rustc --version
cargo --version
```

---

### Compilar Release

```bash
cargo build --release
```

O binario será gerado em `target/release/Compilador` (Linux/macOS) ou `target\release\Compilador.exe` (Windows).

---

## Execucao

```bash
cargo run -- <arquivo>
```

Ou diretamente pelo binario compilado:

**Linux / macOS:**
```bash
./target/release/Compilador <arquivo>
```

**Windows:**
```cmd
target\release\Compilador.exe <arquivo>
```

---

**Exemplo com arquivo de teste:**
```bash
cargo run -- testes/fat.c
```

**Exemplo sem o passo a passo (apenas o resultado):**
```bash
cargo run -- testes/fat.c --no-output
```

---

### Formato da saida

Por padrao, o analisador imprime o passo a passo do parsing: cada token lido, a acao tomada (expansao pela tabela `M(NaoTerminal, lookahead)` ou corte de terminal) e o estado da pilha.

**Exemplo:**
```
>> float (lexema: "float", codigo: 2, linha: 1, coluna: 1)
   acao: M(Program, float) = empilhando p1           pilha: FunctionList $
   acao: M(FunctionList, float) = empilhando p2      pilha: Function FunctionList' $
   acao: M(Type, float) = empilhando p66             pilha: float id ( ParamListOpt ) Block FunctionList' $
   acao: cortando float                              pilha: id ( ParamListOpt ) Block FunctionList' $
```

Ao final, e impresso `Entrada aceita.` ou, em caso de erro, `Entrada rejeitada.` seguido da lista de erros lexicos e sintaticos, cada um com linha e coluna. O flag `--no-output` suprime o passo a passo, mostrando apenas o resultado.

**Exemplo:**
```
Entrada rejeitada.

 2 erro(s) encontrado(s)
   - Erro lexico linha 3, coluna 11: Caractere invalido '@'
   - Erro sintatico linha 3, coluna 15: Token inesperado ';' em <MulExpr>
```

Quando a entrada e rejeitada, o processo encerra com codigo de saida `1`.

---

## Testes

Os arquivos de teste estao no diretorio `testes/`:

- `fat.c` - funcoes matematicas com float (raiz quadrada, raiz cubica, pi, area, distancia)
- `fib.c` - calculo de mdc e raiz quadrada
- `gcd.c` - teoria dos números (mdc, mmc, potencia, primos)
- `erro_lexico.c` - caracteres invalidos
- `erro_sintatico.c` - erros de sintaxe e recuperacao
- `erro_misto.c` - erros lexicos e sintaticos combinados
