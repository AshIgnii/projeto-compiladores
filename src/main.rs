mod automaton;
mod reader;
mod terminal;
mod token;

use crate::automaton::Automaton;
use crate::reader::Reader;
use memmap2::Mmap;
use std::env;
use std::fs::File;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        let program_name: &str = &args[0].split('\\').last().unwrap_or_else(|| &args[0]);
        eprintln!("Nenhum Arquivo Fornecido.\nUso: {} <arquivo>", program_name);
        exit(1);
    }

    let filename = &args[1];
    if !file_exists(filename) {
        eprintln!("Arquivo '{}' não encontrado.", filename);
        exit(1);
    }

    process_file(filename);
}

fn process_file(filename: &str) {
    let file: File = File::open(filename).expect("Erro ao abrir o arquivo");
    let mmap: Mmap = unsafe { Mmap::map(&file).expect("Erro ao mapear arquivo em memoria") };

    let reader: Reader = Reader::new(&(*mmap));
    let mut automaton: Automaton = Automaton::new(reader);

    while let Some(token) = automaton.next_token() {
        println!("{}", token);
    }
}

fn file_exists(path: &str) -> bool {
    Path::new(path).try_exists().unwrap_or_else(|_err| false)
}

fn exit(code: i32) {
    std::process::exit(code);
}
