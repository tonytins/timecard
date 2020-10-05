use clap::{Clap};
use chrono::{Datelike, Local};
use tc::models::Options;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

fn write_csv<S: Into<String>>(csv_file: S, status: S, month: u32, day: u32, year: i32, time: S) {

    // Create a new time card, if it doesn't exist
    if !Path::new(&csv_file.into()).exists() {
        let mut file = File::create(&csv_file.into())
            .expect("Error creating file");
        if let Err(err) = writeln!(file, "Status,Date,Time") {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    // Append status to time card file
    let mut file = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&csv_file.into())
        .unwrap();

    let time_sheet = format!("{},{}/{}/{},{}", status.into(), month, day,
                             year, time.into());

    if let Err(err) = writeln!(file, "{}", time_sheet) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let opts: Options = Options::parse();
    let dt_local = Local::now();

    if opts.check_in {
        write_csv(opts.file, "Checked In", dt_local.month(),
        dt_local.year(), dt_local.day(), &dt_local.time().to_string());
    }

    if opts.check_out {
        write_csv(&opts.file, "Checked Out",dt_local.month(), dt_local.day(),
                  dt_local.year(), dt_local.time().to_string());
    }

    if opts.break_time {
        write_csv(&opts.file, "Break",dt_local.month(), dt_local.day(),
                  dt_local.year(), dt_local.time().to_string());

    }
}
