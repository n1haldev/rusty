use std::fs::File;
use std::io::{self, BufRead, BufReader, Error, ErrorKind};

fn main() -> io::Result<()> {
    let arguments: Vec<_> = std::env::args().collect();

    if arguments.len() < 2 {
        println!("Usage: {} `filename.csv`", arguments[0]);
        return Err(Error::new(ErrorKind::InvalidInput, "Filename not provided!"));
    }

    let fname = std::path::Path::new(&arguments[1]);
    let file = File::open(&fname)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    let mut headers = Vec::new();
    let mut col_widths = Vec::new();

    // Extract headers and calculate initial column widths
    match reader.read_line(&mut line) {
        Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Empty file")),
        Ok(_) => {
            for header in line.trim_end().split(",") {
                headers.push(header.to_string());
                col_widths.push(header.len());
            }
        },
        Err(err) => return Err(err),
    }

    // Print headers with padding
    for (i, header) in headers.iter().enumerate() {
        if i > 0 {
            print!("|"); // Separator between columns
        }
        let padding = col_widths[i] - header.len();
        print!(" {} ", format!("{:^width$}", header, width = header.len() + padding));
    }
    println!();

    // Read and print up to 10 lines of CSV data
    for _ in 0..10 {
        line.clear();

        match reader.read_line(&mut line) {
            Ok(0) => break, // EOF reached
            Ok(_) => {
                let cells: Vec<&str> = line.trim_end().split(",").collect();
                for (i, cell) in cells.iter().enumerate() {
                    if i > 0 {
                        print!("|"); // Separator between columns
                    }
                    let padding = col_widths[i].saturating_sub(cell.len());
                    print!(" {} ", format!("{:^width$}", cell, width = cell.len() + padding));
                }
                println!();
            },
            Err(err) => {
                eprintln!("Error reading line: {}", err);
                break;
            },
        }
    }

    Ok(())
}

