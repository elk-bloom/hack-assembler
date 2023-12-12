use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AssemblyError {
    InvalidCompMnemonic(String),
    InvalidDestMnemonic(String),
    InvalidJumpMnemonic(String),
    AdvanceError(String, u32),
    SymbolError(String, u32),
    CodeError(String, u32),
    CombinedErrors(Vec<AssemblyError>),
}

impl fmt::Display for AssemblyError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            AssemblyError::InvalidCompMnemonic(ref err) => {
                write!(f, "Invalid comp mnemonic: {}", err)
            }
            AssemblyError::InvalidDestMnemonic(ref err) => {
                write!(f, "Invalid dest mnemonic: {}", err)
            }
            AssemblyError::InvalidJumpMnemonic(ref err) => {
                write!(f, "Invalid jump mnemonic: {}", err)
            }
            AssemblyError::AdvanceError(ref err, line_number) => {
                write!(f, "Error reading line {}: {}", line_number, err)
            }
            AssemblyError::SymbolError(ref err, line_number) => {
                write!(f, "Invalid symbol on line {}: {}", line_number, err)
            }
            AssemblyError::CodeError(ref err, line_number) => {
                write!(f, "Invalid code on line {}: {}", line_number, err)
            }
            AssemblyError::CombinedErrors(ref errs) => {
                for (i, err) in errs.iter().enumerate() {
                    write!(f, "Error {}: {}", i, err)?;
                }
                Ok(())
            }
        }
    }
}

impl Error for AssemblyError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match *self {
            AssemblyError::InvalidCompMnemonic(_) => None,
            AssemblyError::InvalidDestMnemonic(_) => None,
            AssemblyError::InvalidJumpMnemonic(_) => None,
            AssemblyError::AdvanceError(_, _) => None,
            AssemblyError::SymbolError(_, _) => None,
            AssemblyError::CodeError(_, _) => None,
            AssemblyError::CombinedErrors(_) => None,
        }
    }
}
