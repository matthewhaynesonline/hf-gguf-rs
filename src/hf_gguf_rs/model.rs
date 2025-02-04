use std::path::PathBuf;

use super::gguf::LlamaFileType;

// class Model
// https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L48
pub struct Model {
    // _model_classes: dict[str, type[Model]] = {}
    dir_model: PathBuf,
    ftype: LlamaFileType,
    fname_out: PathBuf,
    // is_big_endian: bool,
    // endianess: gguf.GGUFEndian
    // use_temp_file: bool,
    // lazy: bool,
    // part_names: list[str]
    // is_safetensors: bool,
    // hparams: dict[str, Any]
    // block_count: i32,
    // tensor_map: gguf.TensorNameMap
    // tensor_names: set[str] | None
    // gguf_writer: gguf.GGUFWriter
    // model_name: Option<String>,
    // metadata_override: Option<PathBuf>,
    // dir_model_card: PathBuf,
    // # subclasses should define this!
    // model_arch: gguf.MODEL_ARCH
}

impl Model {
    // TODO: default args
    pub fn new(
        dir_model: PathBuf,
        ftype: LlamaFileType,
        fname_out: PathBuf,
        // is_big_endian: bool,
        // use_temp_file: bool,
        // eager: bool,
    ) -> Self {
        return Self {
            dir_model,
            ftype,
            fname_out,
        };
    }

    pub fn get_registered_models() -> [String; 1] {
        return [String::from("TODO get models")];
    }
}
