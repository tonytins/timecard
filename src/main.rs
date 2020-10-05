use clap::{Clap};
use isocal::IsoDate;
use chrono::{Local, Datelike};
use tc::models::Opts;
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::path::Path;

fn write_csv<S: Into<String>>(status: S, year: i32, week: S, day: u32, time: S) {
    let opts: Opts = Opts::parse();
    // Create a new time card, if it doesn't exist
    if !Path::new(&opts.file).exists() {
        File::create(&opts.file).expect("Error creating file");
    }

    // Append status to time card file
    let mut csv = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&opts.file)
        .unwrap();

    let project = opts.project.unwrap_or_default();
    let record = if !project.is_empty() {
        format!("{},{}-{}-{},{},{}", status.into(), year, week.into(),
                day, time.into(),project)
    } else {
        format!("{},{}-{}-{},{}", status.into(), year, week.into(),
                day, time.into())
    };

    if let Err(err) = writeln!(csv, "{}", record) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let opts: Opts = Opts::parse();
    let dt_local = Local::now();
    let iso_week = dt_local.iso_week();

    if opts.check_in {
        write_csv("Checked In", dt_local.year(), &iso_week.week_fancy(),
                  dt_local.day(), &dt_local.time().format("%H:%M").to_string());
    }

    if opts.check_out {
        write_csv("Checked Out",dt_local.year(), &iso_week.week_fancy(),
                  dt_local.day(), &dt_local.time().format("%H:%M").to_string());
    }

    if opts.break_time {
        write_csv("Break",dt_local.year(), &iso_week.week_fancy(),
                  dt_local.day(), &dt_local.time().format("%H:%M").to_string());

    }
}
