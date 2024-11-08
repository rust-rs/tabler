use clap::{Parser, ValueEnum};
use std::{num::NonZero, path::PathBuf};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(help = "Path(s) to the file(s) to be opened.", required = false)]
    pub files: Vec<PathBuf>,

    #[arg(short, long, help = "Path to the startup script.", required = false)]
    pub script: Option<PathBuf>,

    #[arg(
        short,
        long,
        help = "Specifies the input format. By default, the format is selected based on the file extension",
        value_enum
    )]
    pub format: Option<Format>,

    #[arg(
        long,
        help = "Specifies if the input does not contain a header row.",
        default_value_t = false
    )]
    pub no_header: bool,

    #[arg(
        long,
        help = "Ignores parsing errors while loading.",
        default_value_t = false
    )]
    pub ignore_errors: bool,

    #[arg(
        long,
        help = "Specifies the method to infer the schema.",
        required = false,
        value_enum,
        default_value_t = InferSchema::Safe,
    )]
    pub infer_schema: InferSchema,

    #[arg(
        long,
        help = "Character used as the field separator or delimiter while loading DSV files.",
        required = false,
        default_value_t = ','
    )]
    pub separator: char,

    #[arg(
        long,
        help = "Character used to quote fields while loading DSV files.",
        required = false,
        default_value_t = '"'
    )]
    pub quote_char: char,

    #[arg(
        long,
        help = "A comma-separeted list of widths, which specifies the column widths for FWF files.",
        required = false,
        default_value_t = String::default(),
    )]
    pub widths: String,

    #[arg(
        long,
        help = "Specifies the separator length for FWF files.",
        required = false,
        default_value_t = 1_usize
    )]
    pub separator_length: usize,

    #[arg(
        long,
        help = "Sets strict column width restrictions for FWF files.",
        required = false,
        default_value_t = false
    )]
    pub no_flexible_width: bool,

    #[arg(
        long,
        help = "Tabler theme",
        required = false,
        value_enum,
        default_value_t = AppTheme::Monokai
    )]
    pub theme: AppTheme,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum Format {
    Dsv,
    Parquet,
    Jsonl,
    Json,
    Arrow,
    Fwf,
}

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum InferSchema {
    No,
    Fast,
    Full,
    Safe,
}

#[derive(Debug, Clone, ValueEnum)]
pub enum AppTheme {
    Monokai,
    Argonaut,
    Terminal,
}

impl InferSchema {
    pub fn to_csv_infer_schema_length(&self) -> Option<usize> {
        match self {
            InferSchema::No => Some(0),
            InferSchema::Fast => Some(128),
            InferSchema::Full => None,
            InferSchema::Safe => Some(0),
        }
    }

    pub fn to_json_infer_schema_length(&self) -> Option<NonZero<usize>> {
        match self {
            InferSchema::No => None,
            InferSchema::Fast => Some(NonZero::new(128).unwrap()),
            InferSchema::Full => None,
            InferSchema::Safe => None,
        }
    }
}
