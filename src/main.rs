use clap::Clap;
use isocal::IsoDate;
use chrono::{Local, Datelike};
use ts::{config::get_config, options::{Opts, SubCommands}};
use std::{fs, fs::{OpenOptions, File}};
use std::io::Write;
use std::path::Path;

fn csv_content(status: String, year: i32, month: u32, day: u32, week: String,
               time: String, task: String) -> String {

    let date = format!("{}/{}/{}", month, day, year);
    format!("{},{},{},{},{}", status, date, week, time, task)
}

fn write_csv<S: Into<String>>(status: S, task: S) {

    let cfg = get_config();

    let dt_local = Local::now();
    let iso_week = dt_local.iso_week();
    let time_format = "%H:%M";

    let header = format!("Status,Date,Week,Time,Task");

    let cfg_path = cfg.directory;
    let cfg_folder = cfg.folder;
    let file_name = format!("{}-{}.csv", dt_local.year(), iso_week.week_fancy());
    let ts_file = format!("{}\\{}\\{}", cfg_path, cfg_folder, file_name);
    let ts_path = format!("{}\\{}", cfg_path, cfg_folder);

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

        if let Err(err) = writeln!(init, "{}", header) {
            eprintln!("Couldn't write to file: {}", err);
        }
    }

    // Append status to time card file
    let mut csv = OpenOptions::new()
        .write(true)
        .append(true)
        .open(&ts_file)
        .unwrap();

    let record = csv_content(status.into(), dt_local.year(), dt_local.month(),
                             dt_local.day(),iso_week.week_fancy(),
                             dt_local.time().format(time_format).to_string(),
                             task.into());

    if let Err(err) = writeln!(csv, "{}", record) {
        eprintln!("Couldn't write to file: {}", err);
    }
}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.subcmds {
        SubCommands::In(check_in) => {
            write_csv("In", check_in.task.as_str());
        }
        SubCommands::Out(_) => {
            write_csv("Out", "");
        }
    };
}

#[cfg(test)]
mod tests {
    use crate::csv_content;

    #[test]
    fn content_test() {
        let content =  csv_content("In".to_string(), 2020, 10,30,
                                   "W44".to_string(), "10:08".to_string(),
                                   "Escort Russel".to_string());

        assert_eq!("In,10/30/2020,W44,10:08,Escort Russel", content);
        assert_ne!("Out,10/30/2020,W44,11:08,", content);
    }
}