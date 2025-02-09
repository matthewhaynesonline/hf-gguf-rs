use std::{io, path::PathBuf, str::FromStr};

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

#[derive(Debug)]
pub enum SizeSuffix {
    K,
    M,
    G,
}

impl SizeSuffix {
    fn multiplier(&self) -> i64 {
        match self {
            SizeSuffix::K => 1000,
            SizeSuffix::M => 1000 * 1000,
            SizeSuffix::G => 1000 * 1000 * 1000,
        }
    }

    pub fn from_char(c: char) -> Option<Self> {
        match c {
            'K' => Some(SizeSuffix::K),
            'M' => Some(SizeSuffix::M),
            'G' => Some(SizeSuffix::G),
            _ => None,
        }
    }
}

// split_str_to_n_bytes()
// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#5021
pub fn split_max_size_str_to_n_bytes(split_str: &str) -> Result<i64, String> {
    let value_str = split_str.trim_end_matches(|c: char| c.is_whitespace());
    let suffix = value_str.chars().last();
    let suffix_enum = suffix.and_then(SizeSuffix::from_char);

    let multiplier = match &suffix_enum {
        Some(suffix_enum) => suffix_enum.multiplier(),
        None => 1,
    };

    let value_str = if suffix_enum.is_some() {
        &value_str[..value_str.len() - 1]
    } else {
        value_str
    };

    return match i64::from_str(value_str) {
        Ok(n) => {
            if n < 0 {
                Err(format!("Invalid value: {split_str} must be positive"))
            } else {
                Ok(n * multiplier)
            }
        }
        Err(_) => Err(format!(
            "Invalid value: {split_str} must be a number, optionally followed by K, M, or G"
        )),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validate_is_split_and_use_temp_file_0() {
        let result = validate_is_split_and_use_temp_file(1, true);
        assert_eq!(
            result,
            Err(String::from("Cannot use temp file when splitting"))
        );
    }

    #[test]
    fn test_validate_is_split_and_use_temp_file_1() {
        let result = validate_is_split_and_use_temp_file(0, true);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_validate_is_split_and_use_temp_file_2() {
        let result = validate_is_split_and_use_temp_file(1, false);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_validate_is_split_and_use_temp_file_3() {
        let result = validate_is_split_and_use_temp_file(0, false);
        assert_eq!(result, Ok(()));
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_0() {
        let result = split_max_size_str_to_n_bytes("1Z");
        assert_eq!(
            result,
            Err(String::from(
                "Invalid value: 1Z must be a number, optionally followed by K, M, or G"
            ))
        );
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_1() {
        let result = split_max_size_str_to_n_bytes("0");
        assert_eq!(result, Ok(0));
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_2() {
        let result = split_max_size_str_to_n_bytes("1");
        assert_eq!(result, Ok(1));
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_3() {
        let result = split_max_size_str_to_n_bytes("1K");
        assert_eq!(result, Ok(1000));
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_4() {
        let result = split_max_size_str_to_n_bytes("1M");
        assert_eq!(result, Ok(1000000));
    }

    #[test]
    fn test_split_max_size_str_to_n_bytes_5() {
        let result = split_max_size_str_to_n_bytes("1G");
        assert_eq!(result, Ok(1000000000));
    }
}
