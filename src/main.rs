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
                let mut fields: Vec<&str> = line.split('\t').collect();
                
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

fn fix_endpoints(fields: &mut Vec<&str>) -> (bool, bool) {
    let mut query_fixed = false;
    let mut target_fixed = false;

    // Fix query endpoints
    if let (Ok(query_start), Ok(query_end), Some(cigar)) = (
        fields[2].parse::<usize>(),
        fields[3].parse::<usize>(),
        fields.last()
    ) {
        if cigar.starts_with("cg:Z:") {
            let correct_query_end = calculate_query_end(query_start, &cigar[5..]);
            if correct_query_end != query_end {
                fields[3] = correct_query_end.to_string().as_str();
                query_fixed = true;
            }
        }
    }

    // Fix target endpoints
    if let (Ok(target_start), Ok(target_end), Some(cigar)) = (
        fields[7].parse::<usize>(),
        fields[8].parse::<usize>(),
        fields.last()
    ) {
        if cigar.starts_with("cg:Z:") {
            let correct_target_end = calculate_target_end(target_start, &cigar[5..]);
            if correct_target_end != target_end {
                fields[8] = correct_target_end.to_string().as_str();
                target_fixed = true;
            }
        }
    }

    (query_fixed, target_fixed)
}

fn calculate_query_end(start: usize, cigar: &str) -> usize {
    let mut query_length = 0;
    let mut num_buffer = String::new();

    for ch in cigar.chars() {
        if ch.is_digit(10) {
            num_buffer.push(ch);
        } else {
            let num: usize = num_buffer.parse().unwrap_or(0);
            num_buffer.clear();

            match ch {
                'M' | '=' | 'X' | 'I' | 'S' | 'H' => query_length += num,
                'D' | 'N' => {}
                _ => {}
            }
        }
    }

    start + query_length
}

fn calculate_target_end(start: usize, cigar: &str) -> usize {
    let mut target_length = 0;
    let mut num_buffer = String::new();

    for ch in cigar.chars() {
        if ch.is_digit(10) {
            num_buffer.push(ch);
        } else {
            let num: usize = num_buffer.parse().unwrap_or(0);
            num_buffer.clear();

            match ch {
                'M' | '=' | 'X' | 'D' | 'N' => target_length += num,
                'I' | 'S' | 'H' => {}
                _ => {}
            }
        }
    }

    start + target_length
}
