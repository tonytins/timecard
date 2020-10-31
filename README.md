# Time Sheet

This is a simple command line application meant to keep track of your work time. Everything is saved to a simple CSV file in the documents path under ``/timesheet``. It's just a prototype right now.

## Usage

```
USAGE:
    ts [FLAGS] [OPTIONS]

FLAGS:
    -b, --break      
    -h, --help       Prints help information
    -i, --in         
    -o, --out        
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>    [default: timesheet.csv]
    -p, --path <path>    [default: timesheet]
    -t, --task <task> 
```

### Example

```csv
Status,Date,Week,Time,Task
Checked In,10/30/2020,W44,19:25,Test
Checked Out,10/30/2020,W44,19:25,
```

## To Do

- [ ] Calculate hours
- [x] Dedicated folder
- [ ] Make front-end

## Requirements

### Prerequisites

- Rust 2018

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details