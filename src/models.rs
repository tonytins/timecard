use clap::{Clap};

#[derive(Clap, Debug)]
#[clap(author, about, version)]
pub struct Options {
    #[clap(short, long, default_value = "timecard.csv")]
    pub file: String,
    #[clap(long)]
    pub check_in: bool,
    #[clap(long)]
    pub check_out: bool,
    #[clap(short, long)]
    pub break_time: bool
}