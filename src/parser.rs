use std::fs::File;
use std::io;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;

use crate::code;
use crate::errors::AssemblyError;
use crate::instruction_enum::Instruction;

pub struct Parser {
    file: File,
    reader: BufReader<File>,
    current_line: String,
    pub current_line_number: u32,
}

impl Parser {
    pub fn new<P: AsRef<Path>>(file_path: P) -> io::Result<Parser> {
        let file = File::open(file_path)?;
        let reader = BufReader::new(file.try_clone()?);
        Ok(Parser {
            file,
            reader,
            current_line: String::new(),
            current_line_number: 0,
        })
    }

    /// A return value of > 0 indicates that that many lines were read.
    /// A return value of 0 indicates that no lines were read due to EOF, the caller should know to stop.
    pub fn advance(&mut self) -> Result<u32, AssemblyError> {
        self.current_line.clear();
        let mut line = String::new();
        let mut lines_read = 0;

        while let Ok(bytes_read) = self
            .reader
            .read_line(&mut line)
            .map_err(|e| AssemblyError::AdvanceError(e.to_string(), self.current_line_number))
        {
            if bytes_read == 0 {
                return Ok(0);
            }
            lines_read += 1;
            self.current_line_number += 1;
            let content_before_comment = line.split("//").next().unwrap_or("").trim();
            if !content_before_comment.is_empty() {
                self.current_line.push_str(content_before_comment);
                return Ok(lines_read);
            }
            line.clear();
        }
        Ok(lines_read)
    }

    pub fn instruction_type(&self) -> Instruction {
        if self.current_line.starts_with('@') {
            Instruction::A
        } else if self.current_line.starts_with('(') && self.current_line.ends_with(')') {
            Instruction::L
        } else {
            Instruction::C
        }
    }

    pub fn symbol(&self) -> Result<&str, AssemblyError> {
        match self.instruction_type() {
            Instruction::A => Ok(&self.current_line[1..]),
            Instruction::L => Ok(&self.current_line[1..self.current_line.len() - 1]),
            _ => Err(AssemblyError::SymbolError(
                format!("{} is not an A or L instruction", self.current_line),
                self.current_line_number,
            )),
        }
    }

    pub fn dest(&self) -> Result<u8, AssemblyError> {
        match self.instruction_type() {
            Instruction::C => {
                let dest_string_option: Option<&str> = if self.current_line.contains('=') {
                    self.current_line.split('=').next()
                } else {
                    None
                };
                code::dest(dest_string_option).map_err(|_| {
                    AssemblyError::CodeError(
                        format!(
                            "{} does not contain a valid dest mnemonic",
                            self.current_line
                        ),
                        self.current_line_number,
                    )
                })
            }
            _ => Err(AssemblyError::CodeError(
                format!("{} is not a C instruction", self.current_line),
                self.current_line_number,
            )),
        }
    }

    pub fn comp(&self) -> Result<u16, AssemblyError> {
        match self.instruction_type() {
            Instruction::C => {
                let i = self.current_line.find('=').map(|i| i + 1).unwrap_or(0);
                let j = self
                    .current_line
                    .find(';')
                    .unwrap_or(self.current_line.len());
                let comp_string_option = self.current_line.get(i..j);
                code::comp(comp_string_option).map_err(|_| {
                    AssemblyError::CodeError(
                        format!(
                            "{} does not contain a valid comp mnemonic",
                            self.current_line
                        ),
                        self.current_line_number,
                    )
                })
            }
            _ => Err(AssemblyError::CodeError(
                format!("{} is not a C instruction", self.current_line),
                self.current_line_number,
            )),
        }
    }

    pub fn jump(&self) -> Result<u8, AssemblyError> {
        match self.instruction_type() {
            Instruction::C => {
                let dest_string_option = if self.current_line.contains(';') {
                    self.current_line.split(';').last()
                } else {
                    None
                };
                code::jump(dest_string_option).map_err(|_| {
                    AssemblyError::CodeError(
                        format!(
                            "{} does not contain a valid jump mnemonic",
                            self.current_line
                        ),
                        self.current_line_number,
                    )
                })
            }
            _ => Err(AssemblyError::CodeError(
                format!("{} is not a C instruction", self.current_line),
                self.current_line_number,
            )),
        }
    }

    pub fn binary_string(&self) -> Result<String, AssemblyError> {
        let mut errors: Vec<AssemblyError> = Vec::new();

        let dest_int: u8 = match self.dest() {
            Ok(i) => i,
            Err(e) => {
                errors.push(e);
                0
            }
        };
        let comp_int: u16 = match self.comp() {
            Ok(i) => i,
            Err(e) => {
                errors.push(e);
                0
            }
        };
        let jump_int: u8 = match self.jump() {
            Ok(i) => i,
            Err(e) => {
                errors.push(e);
                0
            }
        };

        if !errors.is_empty() {
            return Err(AssemblyError::CombinedErrors(errors));
        }

        let mut binary_int: u32 = 0b111 << 13;
        binary_int |= jump_int as u32;
        binary_int |= (dest_int as u32) << 3;
        binary_int |= (comp_int as u32) << 6;

        let binary_string = format!("{:016b}", binary_int);

        Ok(binary_string)
    }

    pub fn reset(&mut self) -> io::Result<()> {
        self.current_line.clear();
        self.current_line_number = 0;
        self.file.seek(SeekFrom::Start(0))?;
        self.reader = BufReader::new(self.file.try_clone()?);
        Ok(())
    }
}
