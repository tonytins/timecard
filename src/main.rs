use clap::Clap;
use isocal::IsoDate;
use chrono::{Local, Datelike};
use ts::models::Opts;
use std::{fs, fs::{OpenOptions, File}};
use std::io::Write;
use std::path::Path;
use directories::UserDirs;

fn write_csv<S: Into<String>>(status: S, year: i32, week: S, day: u32, time: S) {

    let opts: Opts = Opts::parse();
    let mut ts_file = String::new();
    let task = opts.task.unwrap_or_default();

    if let Some(user_dirs) = UserDirs::new() {
        let docs_dir = user_dirs.document_dir()
            .expect("There was a problem detecting documents path.");

        ts_file = format!("{}\\{}\\{}", docs_dir.display(), &opts.path, &opts.file);
        let ts_path = format!("{}\\{}", docs_dir.display(), &opts.path);

        if !Path::new(&ts_path).exists() {
            fs::create_dir(&ts_path)
                .expect("There was a problem creating time sheet directory.");
        }

        // Create a new time card file, if it doesn't exist
        if !Path::new(&ts_file).exists() {
            File::create(&ts_file).expect("Error creating file");

            // Append status to time card file
            let mut init = OpenOptions::new()
                .write(true)
                .append(true)
                .open(&ts_file)
                .unwrap();

            let header = if !task.is_empty() {
                format!("Status,Date,Time,Task")
            } else {
                format!("Status,Date,Time")
            };

            if let Err(err) = writeln!(init, "{}", header) {
                eprintln!("Couldn't write to file: {}", err);
            }
        }
    };

    // Append status to time card file
    let mut csv = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&ts_file)
        .unwrap();


    let record = if task.is_empty() {
        format!("{},{}-{}-{},{}", status.into(), year, week.into(),
                day, time.into())
    } else {
        format!("{},{}-{}-{},{},{}", status.into(), year, week.into(),
                day, time.into(), task)
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
