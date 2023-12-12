use crate::code_constants::{COMP_CODE_BY_MNEMONIC, DEST_MNEMONICS, JUMP_MNEMONICS};
use crate::errors::AssemblyError;

pub fn dest(string: Option<&str>) -> Result<u8, AssemblyError> {
    match string {
        None => Ok(0),
        Some(s) => DEST_MNEMONICS
            .iter()
            .position(|&x| x == s)
            .map(|i| i as u8 + 1)
            .ok_or(AssemblyError::InvalidDestMnemonic(s.to_string())),
    }
}

pub fn comp(string: Option<&str>) -> Result<u16, AssemblyError> {
    match string {
        None => Err(AssemblyError::InvalidCompMnemonic("".to_string())),
        Some(s) => COMP_CODE_BY_MNEMONIC
            .get(s)
            .copied()
            .ok_or(AssemblyError::InvalidCompMnemonic(s.to_string())),
    }
}

pub fn jump(string: Option<&str>) -> Result<u8, AssemblyError> {
    match string {
        None => Ok(0),
        Some(s) => JUMP_MNEMONICS
            .iter()
            .position(|&x| x == s)
            .map(|i| i as u8 + 1)
            .ok_or(AssemblyError::InvalidJumpMnemonic(s.to_string())),
    }
}
