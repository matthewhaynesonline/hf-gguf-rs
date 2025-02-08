use std::{io, path::PathBuf};

pub mod gguf;
pub mod model;
pub mod outtype;

// class SentencePieceTokenTypes(IntEnum)
// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L36
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SentencePieceTokenTypes {
    Normal = 1,
    Unknown = 2,
    Control = 3,
    UserDefined = 4,
    Unused = 5,
    Byte = 6,
}

// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L5054
pub fn validate_model_dir(model_dir: &PathBuf) -> Result<(), io::Error> {
    if !model_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            "Model dir does not exist",
        ));
    } else if !model_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Model dir is not a directory",
        ));
    } else {
        return Ok(());
    }
}

// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L5068
pub fn validate_is_split_and_use_temp_file(
    split_max_tensors: i32,
    use_temp_file: bool,
) -> Result<(), String> {
    if split_max_tensors > 0 && use_temp_file {
        return Err(String::from("Cannot use temp file when splitting"));
    }

    return Ok(());
}
