use anyhow::Result;
use clap::Parser;
use maud::{DOCTYPE, html};
use std::{fs, path::PathBuf};
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[clap(version, author, about)]
struct Args {
    /// Which directory to generate listing of
    #[arg(default_value = ".")]
    input_dir: PathBuf,

    /// Which file to write generated HTML to
    #[arg(short, long, default_value = "index.html")]
    output: PathBuf,

    /// Which files/directories to NOT include in the output
    #[arg(long, value_delimiter = ',')]
    ignored: Vec<PathBuf>,
}

fn generate_html(paths: impl Iterator<Item = PathBuf>) -> String {
    html! {
        (DOCTYPE)
        ul {
            @for path in paths {
                li { (path.to_string_lossy()) }
            }
        }
    }
    .into_string()
}

fn main() -> Result<()> {
    let args = Args::parse();

    let input_paths = WalkDir::new(args.input_dir)
        .into_iter()
        .filter_map(|e| match e {
            Ok(e) => Some(e.into_path()),
            _ => None,
        })
        .filter(|e| {
            for i in &args.ignored {
                if e.starts_with(i) {
                    return false;
                }
            }
            true
        });

    fs::write(args.output, generate_html(input_paths))?;

    Ok(())
}
