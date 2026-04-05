mod automaton;
mod reader;
mod terminal;
mod token;

use crate::automaton::Automaton;
use crate::reader::Reader;
use crate::token::Token;
use memmap2::Mmap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::process::exit;
use std::time::Instant;
use std::{env, io};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        let program_name = args[0].split('\\').next_back().unwrap_or(&args[0]);
        eprintln!("Nenhum Arquivo Fornecido.\nUso: {} <arquivo>", program_name);
        exit(1);
    }

    let output_enabled = !args.contains(&String::from("--no-output"));

    let filename = &args[1];
    if !file_exists(filename) {
        eprintln!("Arquivo '{}' não encontrado.", filename);
        exit(1);
    }

    process_file(filename, output_enabled);
}

fn process_file(filename: &str, output_enabled: bool) {
    let start_time = Instant::now();
    let file: File = File::open(filename).expect("Erro ao abrir o arquivo");
    let mmap: Mmap = unsafe { Mmap::map(&file).expect("Erro ao mapear arquivo em memoria") };

    let reader: Reader = Reader::new(&mmap);
    let mut automaton: Automaton = Automaton::new(reader);

    let mut errors: Vec<Token> = Vec::new();

    let stdout = io::stdout();
    let mut writer = BufWriter::new(stdout.lock());

    while let Some(token) = automaton.next_token() {
        if token.terminal == terminal::Terminal::Eof {
            if output_enabled {
                writeln!(writer, "{}", token).unwrap();
            }
            break;
        } else if token.terminal == terminal::Terminal::Error {
            errors.push(token);
        } else if output_enabled {
            writeln!(writer, "{}", token).unwrap();
        }
    }
    writer.flush().unwrap();

    let duration = start_time.elapsed();

    if output_enabled && !errors.is_empty() {
        let stderr = io::stderr();
        let mut ew = BufWriter::new(stderr.lock());

        writeln!(ew, "\x1b[31mErros léxicos encontrados:\x1b[0m").unwrap();
        for error in errors {
            writeln!(
                ew,
                "\x1b[31mCaractere inválido '{}' em {}:{}\x1b[0m",
                error.lexema, error.line, error.column
            )
            .unwrap();
        }
        ew.flush().unwrap();
        println!("\nTempo de processamento: {:.2?}", duration);
        exit(1);
    }

    println!("\nTempo de processamento: {:.2?}", duration);
}

fn file_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap_or(false)
}
