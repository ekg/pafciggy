use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: {} <input_file or '-' for stdin>", args[0]);
        std::process::exit(1);
    }

    let input: Box<dyn BufRead> = if args[1] == "-" {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(File::open(&args[1])?))
    };

    let stdout = io::stdout();
    let mut writer = io::BufWriter::new(stdout.lock());
    let mut fixed_records = 0;
    let mut error_records = 0;

    for (line_number, line) in input.lines().enumerate() {
        match line {
            Ok(line) => {
                let mut fields: Vec<String> = line.split('\t').map(String::from).collect();
                
                if fields.len() >= 12 {
                    let (query_fixed, target_fixed) = fix_endpoints(&mut fields);
                    if query_fixed || target_fixed {
                        fixed_records += 1;
                    }
                }
                
                if let Err(e) = writeln!(writer, "{}", fields.join("\t")) {
                    eprintln!("Error writing line {}: {}", line_number + 1, e);
                    error_records += 1;
                }
            },
            Err(e) => {
                eprintln!("Error reading line {}: {}", line_number + 1, e);
                error_records += 1;
            }
        }
    }

    eprintln!("Fixed {} records", fixed_records);
    if error_records > 0 {
        eprintln!("Encountered errors in {} records", error_records);
    }
    Ok(())
}

fn fix_endpoints(fields: &mut Vec<String>) -> (bool, bool) {
    let mut query_fixed = false;
    let mut target_fixed = false;

    if let (Ok(query_start), Ok(query_end), Ok(target_start), Ok(target_end), Some(cigar)) = (
        fields[2].parse::<usize>(),
        fields[3].parse::<usize>(),
        fields[7].parse::<usize>(),
        fields[8].parse::<usize>(),
        fields.last()
    ) {
        if cigar.starts_with("cg:Z:") {
            let (query_length, target_length) = calculate_lengths(&cigar[5..]);
            
            let correct_query_end = query_start + query_length;
            if correct_query_end != query_end {
                fields[3] = correct_query_end.to_string();
                query_fixed = true;
            }

            let correct_target_end = target_start + target_length;
            if correct_target_end != target_end {
                fields[8] = correct_target_end.to_string();
                target_fixed = true;
            }
        }
    }

    (query_fixed, target_fixed)
}

fn calculate_lengths(cigar: &str) -> (usize, usize) {
    let mut query_length = 0;
    let mut target_length = 0;
    let mut num_buffer = String::new();

    for ch in cigar.chars() {
        if ch.is_digit(10) {
            num_buffer.push(ch);
        } else {
            let num: usize = num_buffer.parse().unwrap_or(0);
            num_buffer.clear();

            match ch {
                'M' | '=' | 'X' => {
                    query_length += num;
                    target_length += num;
                }
                'I' | 'S' | 'H' => query_length += num,
                'D' | 'N' => target_length += num,
                _ => {}
            }
        }
    }

    (query_length, target_length)
}
