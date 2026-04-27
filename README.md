# Analisador Léxico

Analisador léxico implementado em Rust

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
cargo run -- testes/teste.txt
```

**Exemplo sem saida:**
```bash
cargo run -- testes/teste.txt --no-output
```

---

### Formato da saida

Cada token e impresso no formato:

```
<Terminal, codigo, "lexema", linha:coluna>
```

**Exemplo:**
```
<Int, 1, "int", 1:1>
<Id, 8, "fatorial", 1:5>
<StartParen, 21, "(", 1:13>
...
<Eof, 27, "", 10:1>
```

Erros lexicos sao reportados no `stderr` com a posicao do caractere invalido, e o processo encerra com codigo de saida `1`.

---

## Testes

Os arquivos de teste estao no diretorio `testes/`:

- `fat.c` - funcoes matematicas com float (raiz quadrada, raiz cubica, pi, area, distancia)
- `fib.c` - calculo de mdc e raiz quadrada
- `gcd.c` - teoria dos números (mdc, mmc, potencia, primos)
