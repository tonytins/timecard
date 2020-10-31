# Time Sheet

This is a simple command line application meant to keep track of your work time. Everything is saved to a simple CSV file in the documents path under ``/timesheet``. It's just a prototype right now.

## Usage

```
USAGE:
    ts [OPTIONS] <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -p, --path <path>    [default: timesheet]

SUBCOMMANDS:
    help    Prints this message or the help of the given subcommand(s)
    in      Records the time you started your work.
    out     Records the time you finished your work.
```

### Example

```csv
Status,Date,Week,Time,Task
In,10/30/2020,W44,10:08,Escort Russel
Out,10/30/2020,W44,11:08,
```

## To Do

- [ ] Calculate hours
- [x] Dedicated folder
- [ ] Make front-end (separate repo)

## Requirements

### Prerequisites

- Rust 2018

### Configuration

Time Sheet doesn't create the config file for you, but it looks for one in the following locations:

- Linux: ``/home/[user]/.config/timesheet/config.toml``
- Windows: ``C:\Users\[user]\AppData\Roaming\Sixam\TimeSheet\config\config.toml``
- Mac: ``/Users/[user]/Library/Application Support/com.Sixam.TimeSheet/config.toml``

From the ``config.toml`` you can change the save ``folder`` or ``directory``. The path is where the folder would be saved too. The folder contains the time sheet files. By default, it saves to your documents directory under the ``/timesheet`` folder.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details