use clap::{builder::PossibleValue, ValueEnum};
use std::{fmt, str::FromStr};

use super::gguf::LlamaFileType;

// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L4961
/**
 * By default clap will expect these to be passed in with dashes, like
 * q8-0, instead of q8_0. Wire up parsing manually.
 */
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Outtype {
    F32,
    F16,
    Bf16,
    Q8_0,
    Tq1_0,
    Tq2_0,
    Auto,
}

impl Outtype {
    pub fn to_str(&self) -> &'static str {
        return match *self {
            Outtype::F32 => "f32",
            Outtype::F16 => "f16",
            Outtype::Bf16 => "bf16",
            Outtype::Q8_0 => "q8_0",
            Outtype::Tq1_0 => "tq1_0",
            Outtype::Tq2_0 => "tq2_0",
            Outtype::Auto => "auto",
        };
    }

    // https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L5058
    pub fn to_llama_file_type(&self) -> LlamaFileType {
        return match *self {
            Outtype::F32 => LlamaFileType::AllF32,
            Outtype::F16 => LlamaFileType::MostlyF16,
            Outtype::Bf16 => LlamaFileType::MostlyBF16,
            Outtype::Q8_0 => LlamaFileType::MostlyQ8_0,
            Outtype::Tq1_0 => LlamaFileType::MostlyTQ1_0,
            Outtype::Tq2_0 => LlamaFileType::MostlyTQ1_0,
            Outtype::Auto => LlamaFileType::Guessed,
        };
    }
}

impl FromStr for Outtype {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        return match s.replace('-', "_").as_str() {
            "f32" => Ok(Outtype::F32),
            "f16" => Ok(Outtype::F16),
            "bf16" => Ok(Outtype::Bf16),
            "q8_0" => Ok(Outtype::Q8_0),
            "tq1_0" => Ok(Outtype::Tq1_0),
            "tq2_0" => Ok(Outtype::Tq2_0),
            "auto" => Ok(Outtype::Auto),
            _ => Err(format!("Invalid value for Outtype: {}", s)),
        };
    }
}

// Enables CLI parsing with clap::ValueEnum
impl ValueEnum for Outtype {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            Outtype::F32,
            Outtype::F16,
            Outtype::Bf16,
            Outtype::Q8_0,
            Outtype::Tq1_0,
            Outtype::Tq2_0,
            Outtype::Auto,
        ]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        return match *self {
            Outtype::F32 => Some(PossibleValue::new("f32")),
            Outtype::F16 => Some(PossibleValue::new("f16")),
            Outtype::Bf16 => Some(PossibleValue::new("bf16")),
            Outtype::Q8_0 => Some(PossibleValue::new("q8_0")),
            Outtype::Tq1_0 => Some(PossibleValue::new("tq1_0")),
            Outtype::Tq2_0 => Some(PossibleValue::new("tq2_0")),
            Outtype::Auto => Some(PossibleValue::new("auto")),
        };
    }
}

impl fmt::Display for Outtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_str() {
        assert_eq!(Outtype::F32.to_str(), "f32");
        assert_eq!(Outtype::F16.to_str(), "f16");
        assert_eq!(Outtype::Bf16.to_str(), "bf16");
        assert_eq!(Outtype::Q8_0.to_str(), "q8_0");
        assert_eq!(Outtype::Tq1_0.to_str(), "tq1_0");
        assert_eq!(Outtype::Tq2_0.to_str(), "tq2_0");
        assert_eq!(Outtype::Auto.to_str(), "auto");
    }

    #[test]
    fn test_to_llama_file_type() {
        assert_eq!(Outtype::F32.to_llama_file_type(), LlamaFileType::AllF32);
        assert_eq!(Outtype::F16.to_llama_file_type(), LlamaFileType::MostlyF16);
        assert_eq!(
            Outtype::Bf16.to_llama_file_type(),
            LlamaFileType::MostlyBF16
        );
        assert_eq!(
            Outtype::Q8_0.to_llama_file_type(),
            LlamaFileType::MostlyQ8_0
        );
        assert_eq!(
            Outtype::Tq1_0.to_llama_file_type(),
            LlamaFileType::MostlyTQ1_0
        );
        assert_eq!(
            Outtype::Tq2_0.to_llama_file_type(),
            LlamaFileType::MostlyTQ1_0
        );
        assert_eq!(Outtype::Auto.to_llama_file_type(), LlamaFileType::Guessed);
    }

    #[test]
    fn test_from_str_valid() {
        assert_eq!("f32".parse::<Outtype>().unwrap(), Outtype::F32);
        assert_eq!("f16".parse::<Outtype>().unwrap(), Outtype::F16);
        assert_eq!("bf16".parse::<Outtype>().unwrap(), Outtype::Bf16);
        assert_eq!("q8_0".parse::<Outtype>().unwrap(), Outtype::Q8_0);
        assert_eq!("tq1_0".parse::<Outtype>().unwrap(), Outtype::Tq1_0);
        assert_eq!("tq2_0".parse::<Outtype>().unwrap(), Outtype::Tq2_0);
        assert_eq!("auto".parse::<Outtype>().unwrap(), Outtype::Auto);
    }

    #[test]
    fn test_from_str_invalid() {
        assert!("invalid".parse::<Outtype>().is_err());
        assert!("q8-1".parse::<Outtype>().is_err());
        assert!("".parse::<Outtype>().is_err());
    }

    #[test]
    fn test_value_variants() {
        let variants = Outtype::value_variants();
        assert!(variants.contains(&Outtype::F32));
        assert!(variants.contains(&Outtype::F16));
        assert!(variants.contains(&Outtype::Bf16));
        assert!(variants.contains(&Outtype::Q8_0));
        assert!(variants.contains(&Outtype::Tq1_0));
        assert!(variants.contains(&Outtype::Tq2_0));
        assert!(variants.contains(&Outtype::Auto));
    }

    #[test]
    fn test_display() {
        assert_eq!(format!("{}", Outtype::F32), "f32");
        assert_eq!(format!("{}", Outtype::F16), "f16");
        assert_eq!(format!("{}", Outtype::Bf16), "bf16");
        assert_eq!(format!("{}", Outtype::Q8_0), "q8_0");
        assert_eq!(format!("{}", Outtype::Tq1_0), "tq1_0");
        assert_eq!(format!("{}", Outtype::Tq2_0), "tq2_0");
        assert_eq!(format!("{}", Outtype::Auto), "auto");
    }
}
