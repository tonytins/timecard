# Time Sheet

This is a simple command line application meant to keep track of your work time in CSV format. By default, files will be saved to your document's directory in the ``/timesheet`` folder.

While this is still just a prototype, I welcome contributors.

## Usage

```
USAGE:
    ts <SUBCOMMAND>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

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
- Windows: ``C:\Users\[user]\AppData\Roaming\Sixam\TimeSheet\config.toml``
- Mac: ``/Users/[user]/Library/Application Support/com.Sixam.TimeSheet/config.toml``

From the ``config.toml`` you can change the save ``folder`` or ``directory``. The ``directory`` is where the ``folder`` containing the time sheet files. For example:

```toml
folder = "Sixam"
directory = "/home/tonytins"
```

## License

I license this project under the MIT License - see the [LICENSE](LICENSE) file for details