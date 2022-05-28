use clap::Parser;

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Args {
    #[clap(long = "seed")]
    pub raw_seed: Option<u64>,

    #[clap(long)]
    pub critters: u32,

    #[clap(skip)]
    pub seed: u64,

    #[clap(flatten)]
    pub health: Health,
}

#[derive(Parser)]
#[clap(rename_all = "kebab-case")]
pub struct Health {
    #[clap(long = "init-health-min", default_value = "1000.0")]
    pub init_min: f32,
    #[clap(long = "init-health-max", default_value = "1000.0")]
    pub init_max: f32,

    #[clap(long = "health-per-vel", default_value = "10.0")]
    pub per_vel: f32,
}

impl Args {
    pub fn parse() -> Self {
        Parser::parse()
    }
}
