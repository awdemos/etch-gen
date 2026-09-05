use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "etch-gen")]
#[command(about = "AI-powered blockchain generator using Etch architecture")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Generate {
        #[arg(
            short,
            long,
            help = "Natural language prompt describing desired blockchain"
        )]
        prompt: String,
        #[arg(short, long, default_value = "./generated", help = "Output directory")]
        output_dir: PathBuf,
        #[arg(
            short,
            long,
            default_value = "http://127.0.0.1:11435",
            help = "LLM API base URL"
        )]
        api_url: String,
        #[arg(
            short,
            long,
            default_value = "qwen2.5-coder-14b-instruct-q4-k-m",
            help = "LLM model name"
        )]
        model: String,
        #[arg(long, help = "Include benchmark module")]
        benchmarks: bool,
    },
    Preset {
        #[arg(value_enum)]
        preset: Preset,
        #[arg(short, long, default_value = "./generated")]
        output_dir: PathBuf,
        #[arg(long, help = "Include benchmark module")]
        benchmarks: bool,
    },
}

#[derive(Clone, Debug, ValueEnum)]
pub enum Preset {
    Mainnet,
    FastTest,
}
