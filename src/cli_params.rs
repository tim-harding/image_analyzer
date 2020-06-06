use clap::Clap;

#[derive(Clap)]
#[clap(version = "1.0", author = "Tim Harding <Tim@TimHarding.co>")]
pub struct Opts {
    #[clap(short, long)]
    pub dir: String,

    #[clap(short, long)]
    pub out: String,
}
