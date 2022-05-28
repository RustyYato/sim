use clap::Parser;

#[derive(Parser)]
pub struct Args {
    #[clap(long = "seed")]
    pub raw_seed: Option<u64>,

    #[clap(long)]
    pub critters: u32,

    #[clap(skip)]
    pub seed: u64,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
