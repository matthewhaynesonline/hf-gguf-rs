use std::path::PathBuf;

use clap::{error::ErrorKind, CommandFactory, Parser};

pub mod hf_gguf_rs;
use hf_gguf_rs::{
    model::Model, outtype::Outtype, split_max_size_str_to_n_bytes,
    validate_is_split_and_use_temp_file, validate_model_dir,
};

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    // parse_args()
    // https://github.com/ggerganov/llama.cpp/blob/864a0b67a6c8f648c43ce8271f9cb2e12dd5df6e/convert_hf_to_gguf.py#L4950
    #[arg(help = "Directory containing the model file.")]
    model: Option<PathBuf>,

    #[arg(long, help = "Extract only the vocab.")]
    vocab_only: bool,

    #[arg(
        long,
        help = "Path to write to; default: based on input. {ftype} will be replaced by the outtype."
    )]
    outfile: Option<PathBuf>,

    #[arg(
        value_enum,
        long,
        default_value_t = Outtype::F16,
        help = "Use f32 for float32, f16 for float16, bf16 for bfloat16, q8_0 for Q8_0, tq1_0 or tq2_0 for ternary, and auto for the highest-fidelity 16-bit float type depending on the first loaded tensor type."
    )]
    outtype: Outtype,

    #[arg(long, help = "Model is executed on big endian machine.")]
    bigendian: bool,

    #[arg(
        long,
        help = "Use the tempfile library while processing (helpful when running out of memory, process killed)."
    )]
    use_temp_file: bool,

    #[arg(
        long,
        help = "Use more RAM by computing all outputs before writing (use in case lazy evaluation is broken)."
    )]
    no_lazy: bool,

    #[arg(long, help = "Name of the model.")]
    model_name: Option<String>,

    #[arg(long, help = "Increase output verbosity.")]
    verbose: bool,

    #[arg(long, help = "Max tensors in each split.", default_value_t = 0)]
    split_max_tensors: i32,

    #[arg(long, help = "max size per split N(M|G)", default_value_t = String::from("0"))]
    split_max_size: String,

    #[arg(
        long,
        help = "Only print out a split plan and exit, without writing any new files."
    )]
    dry_run: bool,

    #[arg(long, help = "Do not add tensors to the first split.")]
    no_tensor_first_split: bool,

    #[arg(
        long,
        help = "Specify the path for an authorship metadata override file."
    )]
    metadata: Option<PathBuf>,

    #[arg(long, help = "Print the supported models.")]
    print_supported_models: bool,
}

fn main() {
    let cli = Cli::parse();
    let mut cmd = Cli::command();

    // llama.cpp hf-to-gguf has weird api surface where model is required
    // but if the print supported models is passed in then it's bypassed
    //
    // TODO: probably refactor this to a separate command, but
    // keeping for now to maintain api compatibility
    if cli.print_supported_models {
        let supported_models = Model::get_registered_models();

        for model_name in supported_models {
            println!("- {model_name}");
        }
        // TODO: refactor
        // process::exit(0);
        return;
    }

    if cli.model.is_none() {
        cmd.error(ErrorKind::MissingRequiredArgument, "MODEL is required")
            .exit()
    }

    let dir_model = cli.model.as_ref().unwrap();

    if let Err(e) = validate_model_dir(dir_model) {
        let error_message = format!("MODEL '{}' error: {e}", dir_model.display());
        cmd.error(ErrorKind::ValueValidation, error_message).exit();
    }

    if let Err(error_message) =
        validate_is_split_and_use_temp_file(cli.split_max_tensors, cli.use_temp_file)
    {
        cmd.error(ErrorKind::ValueValidation, error_message).exit();
    }

    let fname_out = match cli.outfile.as_ref() {
        Some(outfile) => outfile,
        None => &dir_model.to_path_buf(),
    };

    let split_max_size = match split_max_size_str_to_n_bytes(&cli.split_max_size) {
        Ok(n) => n,
        Err(error_message) => cmd.error(ErrorKind::ValueValidation, error_message).exit(),
    };

    println!("Loading model: {}", dir_model.display());
    // Model::load_model(&dir_model);

    // cli_debug_print(&cli, &fname_out);
}

fn cli_debug_print(cli: &Cli, fname_out: &PathBuf) {
    println!("Value for fname_out: {}", fname_out.display());

    println!("Value for --vocab-only: {}", cli.vocab_only);

    println!("Value for --outtype: {}", cli.outtype);
    println!(
        "Value for --ftype_map: {:?}",
        cli.outtype.to_llama_file_type()
    );

    if let Some(outfile) = cli.outfile.as_deref() {
        println!("Value for --outfile: {}", outfile.display());
    }

    println!("Value for --bigendian: {}", cli.bigendian);
    println!("Value for --use-temp-file: {}", cli.use_temp_file);
    println!("Value for --no-lazy: {}", cli.no_lazy);

    if let Some(model_name) = cli.model_name.as_deref() {
        println!("Value for --model-name: {model_name}");
    }

    println!("Value for --verbose: {}", cli.verbose);
    println!("Value for --split-max-tensors: {}", cli.split_max_tensors);
    // println!("Value for --split-max-size: {}", cli.split_max_size);
    println!("Value for --dry-run: {}", cli.dry_run);
    println!(
        "Value for --no-tensor-first-split: {}",
        cli.no_tensor_first_split
    );

    if let Some(metadata) = cli.metadata.as_deref() {
        println!("Value for --metadata: {}", metadata.display());
    }

    println!(
        "Value for --print-supported-models: {}",
        cli.print_supported_models
    );
}
