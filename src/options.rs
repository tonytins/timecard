use clap::{Clap};

#[derive(Clap, Debug, Clone)]
#[clap(author, about, version)]
pub struct Opts {
    #[clap(short, long, default_value = "timesheet.csv")]
    pub file: String,
    #[clap(short, long, default_value = "timesheet")]
    pub path: String,
    #[clap(name = "in", short, long)]
    pub check_in: bool,
    #[clap(name = "out", short, long)]
    pub check_out: bool,
    #[clap(name = "break", short, long)]
    pub break_time: bool,
    #[clap(short, long)]
    pub task: String
}