mod automaton;
mod logger;
mod parser;
mod parsing;
mod reader;
mod symbols;
mod token;

use crate::automaton::Automaton;
use crate::logger::Logger;
use crate::parser::Parser;
use crate::reader::Reader;
use memmap2::Mmap;
use std::fs::File;
use std::io::BufWriter;
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
    let automaton: Automaton = Automaton::new(reader);

    let stdout = io::stdout();
    let writer = BufWriter::new(stdout.lock());
    let mut logger = Logger::new(writer, output_enabled);

    let mut parser = Parser::new(automaton);
    let accepted = parser.parse(&mut logger);
    logger.flush();

    let duration = start_time.elapsed();
    println!("\nTempo de processamento: {:.2?}", duration);

    if !accepted {
        exit(1);
    }
}

fn file_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap_or(false)
}
