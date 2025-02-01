// class LlamaFileType(IntEnum)
// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/gguf-py/gguf/constants.py#L1665
#[repr(i32)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LlamaFileType {
    AllF32 = 0,
    MostlyF16 = 1,     // except 1d tensors
    MostlyQ4_0 = 2,    // except 1d tensors
    MostlyQ4_1 = 3,    // except 1d tensors
    MostlyQ8_0 = 7,    // except 1d tensors
    MostlyQ5_0 = 8,    // except 1d tensors
    MostlyQ5_1 = 9,    // except 1d tensors
    MostlyQ2K = 10,    // except 1d tensors
    MostlyQ3KS = 11,   // except 1d tensors
    MostlyQ3KM = 12,   // except 1d tensors
    MostlyQ3KL = 13,   // except 1d tensors
    MostlyQ4KS = 14,   // except 1d tensors
    MostlyQ4KM = 15,   // except 1d tensors
    MostlyQ5KS = 16,   // except 1d tensors
    MostlyQ5KM = 17,   // except 1d tensors
    MostlyQ6K = 18,    // except 1d tensors
    MostlyIQ2XXS = 19, // except 1d tensors
    MostlyIQ2XS = 20,  // except 1d tensors
    MostlyQ2KS = 21,   // except 1d tensors
    MostlyIQ3XS = 22,  // except 1d tensors
    MostlyIQ3XXS = 23, // except 1d tensors
    MostlyIQ1S = 24,   // except 1d tensors
    MostlyIQ4NL = 25,  // except 1d tensors
    MostlyIQ3S = 26,   // except 1d tensors
    MostlyIQ3M = 27,   // except 1d tensors
    MostlyIQ2S = 28,   // except 1d tensors
    MostlyIQ2M = 29,   // except 1d tensors
    MostlyIQ4XS = 30,  // except 1d tensors
    MostlyIQ1M = 31,   // except 1d tensors
    MostlyBF16 = 32,   // except 1d tensors
    MostlyTQ1_0 = 36,  // except 1d tensors
    MostlyTQ2_0 = 37,  // except 1d tensors

    Guessed = 1024, // not specified in the model file
}

impl LlamaFileType {
    pub fn to_int(&self) -> i32 {
        return *self as i32;
    }
}
