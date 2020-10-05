# Time Card

This is a simple command line application meant to keep track of your work time. Everything is saved to a simple CSV file. It's just a prototype right now.

## Usage

```
USAGE:
    tc.exe [FLAGS] [OPTIONS]

FLAGS:
    -b, --break      
    -h, --help       Prints help information
    -i, --in         
    -o, --out        
    -V, --version    Prints version information

OPTIONS:
    -f, --file <file>          [default: timecard.csv]
    -p, --project <project>    
```

## Example

```csv
Checked In,2020-W41-5,17:31
Break,2020-W41-5,17:31
Checked Out,2020-W41-5,17:31
```

## To Do

- [ ] Calculate hours
- [ ] Make front-end

## Requirements

### Prerequisites

- Rust 2018

## Authors

- **Anthony Leland** - _Initial work_ - [tonytins](https://github.com/tonytins)

See also the list of [contributors](https://github.com/tonytins/citylimits/contributors) who participated in this project.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details