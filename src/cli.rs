use clap::Parser;

#[derive(clap::ArgEnum, Clone, Debug)]
enum Size {
    #[clap(alias("s"))]
    Small,
    #[clap(alias("n"))]
    #[clap(alias("medium"))]
    #[clap(alias("m"))]
    Normal,
    #[clap(alias("l"))]
    Large,
    #[clap(alias("xl"))]
    ExtraLarge,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(arg_required_else_help(true))] 
pub struct Cli {
    files: Vec<String>,

    #[clap(short, long)]
    find: Option<Vec<String>>,

    #[clap(short, long)]
    #[clap(arg_enum)]
    size: Option<Size>,
}

pub fn run()-> Cli {
    let cli = Cli::parse();
    cli
}
