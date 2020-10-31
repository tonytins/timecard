use clap::{Clap};

#[derive(Clap, Debug, Clone)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmds: SubCommands
}

#[derive(Clap, Debug, Clone)]
pub enum SubCommands {
    #[clap(author, about = "Records the time you started your work.", version)]
    In(CheckIn),
    #[clap(author, about = "Records the time you finished your work.", version)]
    Out(CheckOut)
}

#[derive(Clap, Debug, Clone)]
pub struct CheckOut {
}

#[derive(Clap, Debug, Clone)]
pub struct CheckIn {
    #[clap(short, long)]
    pub task: String
}
