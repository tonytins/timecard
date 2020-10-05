use clap::{Clap};
use chrono::{Datelike, Local};
use tc::models::Options;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

fn write_csv<S: Into<String>>(status: S, month: u32, day: u32, year: i32, time: S) {
    let opts: Options = Options::parse();
    // Create a new time card, if it doesn't exist
    if !Path::new(&opts.file).exists() {
        let mut file = File::create(&opts.file)
            .expect("Error creating file");
        if let Err(err) = writeln!(file, "Status,Date,Time") {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    // Append status to time card file
    let mut csv = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&opts.file)
        .unwrap();

    let time_sheet = format!("{},{}/{}/{},{}", status.into(), month, day,
                             year, time.into());

    if let Err(err) = writeln!(csv, "{}", time_sheet) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let opts: Options = Options::parse();
    let dt_local = Local::now();

    if opts.check_in {
        write_csv("Checked In", dt_local.month(), dt_local.day(),
                  dt_local.year(), &dt_local.time().to_string());
    }

    if opts.check_out {
        write_csv("Checked Out",dt_local.month(), dt_local.day(),
                  dt_local.year(), &dt_local.time().to_string());
    }

    if opts.break_time {
        write_csv("Break",dt_local.month(), dt_local.day(),
                  dt_local.year(), &dt_local.time().to_string());

    }
}
