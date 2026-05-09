use clap::Parser;
use etch_gen::cli::{Cli, Commands, Preset};
use etch_gen::design::presets;
use etch_gen::factory::{CrateWriter, ManifestGenerator, TemplateRenderer};
use etch_gen::llm::{DesignParser, LlmClient, PromptTemplate};
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            prompt,
            output_dir,
            api_url,
            model,
            benchmarks,
        } => {
            println!(" Consulting LLM at {} using model {}...", api_url, model);
            let client = LlmClient::new(&api_url);
            let prompt_template = PromptTemplate::new();
            let rendered_prompt = prompt_template.render(&prompt);
            let response = client.chat_completion(&rendered_prompt, &model).await?;

            println!(" Parsing LLM response...");
            let parser = DesignParser::new();
            let design = parser.parse(&response)?;
            println!(
                " Design parsed: {} - {}",
                design.design_name, design.description
            );

            generate_crate(&design, &output_dir, benchmarks).await?;
        }
        Commands::Preset {
            preset,
            output_dir,
            benchmarks,
        } => {
            let design = match preset {
                Preset::Mainnet => presets::mainnet(),
                Preset::FastTest => presets::fast_test(),
            };
            println!(" Using preset: {:?}", preset);
            generate_crate(&design, &output_dir, benchmarks).await?;
        }
    }

    Ok(())
}

async fn generate_crate(
    design: &etch_gen::design::BlockchainDesign,
    output_dir: &Path,
    benchmarks: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    println!(" Generating crate '{}'...", design.design_name);

    let manifest = ManifestGenerator::generate(design, benchmarks);
    let renderer = TemplateRenderer::new()?;
    let files = renderer.render_all(design)?;
    let writer = CrateWriter::new(output_dir);
    let crate_path = writer.write(&design.design_name, manifest, files)?;

    println!(
        " Blockchain crate generated at: {}",
        crate_path.canonicalize()?.display()
    );
    println!(" To build: cd {} && cargo check", crate_path.display());

    Ok(())
}
