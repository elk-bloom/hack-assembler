use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::PathBuf;

mod code;
mod code_constants;
mod errors;
mod instruction_enum;
mod parser;
mod predefined_symbols;
mod symbol_table;

use clap::Parser;

use instruction_enum::Instruction;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// The .asm file to assemble
    input_file: String,
    /// The optional .hack file to write to.
    /// If not provided, the .hack file will be written to the same directory as the .asm file.
    #[arg(short = 'o', long = "output")]
    output_file: Option<String>,
}

fn main() {
    let args = Args::parse();

    let path_to_asm = PathBuf::from(args.input_file);
    let path_to_hack = match args.output_file {
        Some(output_file) => PathBuf::from(output_file),
        None => path_to_asm.with_extension("hack"),
    };

    // initialize parser and symbol table
    let mut parser = parser::Parser::new(path_to_asm).unwrap();
    let mut symbol_table = symbol_table::SymbolTable::new();
    for (symbol, address) in predefined_symbols::PREDEFINED_SYMBOLS.iter() {
        symbol_table.add_entry(symbol.to_string(), *address);
    }

    // populate symbol table with first pass
    let mut instruction_address: u16 = 0;
    while {
        let lines_read = parser.advance().unwrap_or_else(|e| {
            panic!("Error reading line {}: {}", parser.current_line_number, e);
        });
        lines_read > 0
    } {
        let instruction_type = parser.instruction_type();
        match instruction_type {
            Instruction::L => {
                let symbol = parser.symbol().unwrap();
                if !symbol_table.contains(symbol) {
                    symbol_table.add_entry(symbol.to_string(), instruction_address);
                }
            }
            Instruction::A | Instruction::C => {
                instruction_address += 1;
            }
        }
    }

    // second pass to generate machine code
    parser.reset().expect("Reset before second pass failed.");
    let output_file = File::create(path_to_hack).unwrap();
    let mut writer = BufWriter::new(output_file);

    let mut new_symbols: u16 = 0;
    while {
        let lines_read = parser.advance().unwrap_or_else(|e| {
            panic!("Error reading line {}: {}", parser.current_line_number, e);
        });
        lines_read > 0
    } {
        let instruction_type = parser.instruction_type();
        let mut line_to_write: String;
        match instruction_type {
            Instruction::A => {
                let symbol = parser.symbol().unwrap();
                if symbol.chars().all(char::is_numeric) {
                    let binary_string = format!("{:016b}", symbol.parse::<u16>().unwrap());
                    line_to_write = binary_string;
                } else {
                    if !symbol_table.contains(symbol) {
                        let new_address: u16 = 16 + new_symbols;
                        symbol_table.add_entry(symbol.to_string(), new_address);
                        new_symbols += 1;
                    }

                    let address = symbol_table.get_address(symbol).or_else(|| {
                        panic!(
                            "Symbol {} not found in symbol table on line {}",
                            symbol, parser.current_line_number
                        )
                    });

                    let binary_string = format!("{:016b}", address.unwrap());
                    line_to_write = binary_string;
                }
            }
            Instruction::C => {
                let binary_string = parser.binary_string().unwrap();
                line_to_write = binary_string;
            }
            _ => continue,
        }
        line_to_write.push('\n');
        writer.write_all(line_to_write.as_bytes()).unwrap();
    }
}
