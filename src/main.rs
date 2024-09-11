use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader, Write};

#[cfg(test)]
mod tests {
    use super::*;

    fn test_fix_endpoints(input: &str, expected: &str) {
        let mut input_fields: Vec<String> = input.split('\t').map(String::from).collect();
        let expected_fields: Vec<String> = expected.split('\t').map(String::from).collect();
        
        let (query_fixed, target_fixed, _, _) = fix_endpoints(&mut input_fields);
        
        assert_eq!(input_fields, expected_fields, "Fields do not match after fixing");
        assert!(query_fixed || target_fixed, "No fixes were applied");
    }

    #[test]
    fn test_paf_records() {
        let test_cases = [
            (
                "S288C#1#chrI\t219929\t0\t44981\t+\tSGDref#1#chrI\t230218\t0\t44561\t44280\t45000\t18\tgi:f:0.993583\tbi:f:0.984\tmd:f:0.998965\tcg:Z:1=49I6754=3I5956=384I7260=2I3765=1I1603=1X147=1X8=1X8=1X2=1X2=1X2=1X20=1X5=1X17=1X23=1X26=1X11=1X29=2X1=1X1=2X1=1X15=1X8=1X5=1X2=1X14=4X17=1X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X5=1X3=3X2=2X1=1X2=1X12=1X3=2X11=1X2=1X8=1X1=1X9=1X2=1X4=2X2=1X2=1X2=1X5=1X2=1X8=1X6=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X12=1X3=2X14=1X8=1X1=1X9=1X2=1X4=2X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X9=3X2=2X1=1X2=1X12=1X3=2X14=1X8=1X1=1X9=1X2=1X4=2X2=1X5=1X5=1X2=1X8=1X7=3X23=3X2=2X1=1X2=1X5=1X6=1X1=1X1=2X7=2X14=3X12=1X5=1X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X5=1X3=3X2=2X1=1X2=1X12=1X3=2X11=1X14=1X17=1X5=1X1=1X3=1X2=1X2=1X5=1X2=2X5=1X1=2X9=1X4=1X4=1X3=3X2=2X1=1X2=1X16=2X11=1X14=1X41=1X2=2X5=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X16=2X11=1X14=1X41=1X2=2X5=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X16=2X11=1X2=1X11=1X5=1X35=1X2=2X6=3X23=3X2=2X1=1X2=1X12=1X3=2X21=1X7=1X13=2X2=1X4=1X3=1X2=1X2=1X5=1X2=1X5=1X1=1X1=1X7=1X2=1X2=1X29=1X3=1X3=2X7=1X15=1X2=1X8=1X2=1X4=2X2=1X5=1X8=1X5=1X20=1X2=1X17432=",
                "S288C#1#chrI\t219929\t0\t45000\t+\tSGDref#1#chrI\t230218\t0\t44561\t44280\t45000\t18\tgi:f:0.993583\tbi:f:0.984\tmd:f:0.998965\tcg:Z:1=49I6754=3I5956=384I7260=2I3765=1I1603=1X147=1X8=1X8=1X2=1X2=1X2=1X20=1X5=1X17=1X23=1X26=1X11=1X29=2X1=1X1=2X1=1X15=1X8=1X5=1X2=1X14=4X17=1X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X5=1X3=3X2=2X1=1X2=1X12=1X3=2X11=1X2=1X8=1X1=1X9=1X2=1X4=2X2=1X2=1X2=1X5=1X2=1X8=1X6=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X12=1X3=2X14=1X8=1X1=1X9=1X2=1X4=2X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X9=3X2=2X1=1X2=1X12=1X3=2X14=1X8=1X1=1X9=1X2=1X4=2X2=1X5=1X5=1X2=1X8=1X7=3X23=3X2=2X1=1X2=1X5=1X6=1X1=1X1=2X7=2X14=3X12=1X5=1X2=1X8=1X11=1X2=1X5=2X2=1X7=1X1=2X2=1X5=1X3=3X2=2X1=1X2=1X12=1X3=2X11=1X14=1X17=1X5=1X1=1X3=1X2=1X2=1X5=1X2=2X5=1X1=2X9=1X4=1X4=1X3=3X2=2X1=1X2=1X16=2X11=1X14=1X41=1X2=2X5=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X16=2X11=1X14=1X41=1X2=2X5=1X1=2X9=1X9=1X3=3X2=2X1=1X2=1X16=2X11=1X2=1X11=1X5=1X35=1X2=2X6=3X23=3X2=2X1=1X2=1X12=1X3=2X21=1X7=1X13=2X2=1X4=1X3=1X2=1X2=1X5=1X2=1X5=1X1=1X1=1X7=1X2=1X2=1X29=1X3=1X3=2X7=1X15=1X2=1X8=1X2=1X4=2X2=1X5=1X8=1X5=1X20=1X2=1X17432="
            ),
            (
                "S288C#1#chrI\t219929\t45000\t94990\t+\tSGDref#1#chrI\t230218\t44561\t94560\t49999\t50000\t47\tgi:f:0.99998\tbi:f:0.99998\tmd:f:1\tcg:Z:1089=1I48910=",
                "S288C#1#chrI\t219929\t45000\t95000\t+\tSGDref#1#chrI\t230218\t44561\t94560\t49999\t50000\t47\tgi:f:0.99998\tbi:f:0.99998\tmd:f:1\tcg:Z:1089=1I48910="
            ),
            (
                "S288C#1#chrI\t219929\t95000\t144993\t+\tSGDref#1#chrI\t230218\t94560\t144557\t49994\t50000\t39\tgi:f:0.9999\tbi:f:0.99988\tmd:f:0.999929\tcg:Z:6720=1X24=1I25590=2I14134=1X104=1X3422=",
                "S288C#1#chrI\t219929\t95000\t145000\t+\tSGDref#1#chrI\t230218\t94560\t144557\t49994\t50000\t39\tgi:f:0.9999\tbi:f:0.99988\tmd:f:0.999929\tcg:Z:6720=1X24=1I25590=2I14134=1X104=1X3422="
            ),
        ];

        for (input, expected) in test_cases.iter() {
            test_fix_endpoints(input, expected);
        }
    }
}

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
    let mut query_fixed_count = 0;
    let mut target_fixed_count = 0;
    let mut total_query_displacement = 0;
    let mut total_target_displacement = 0;
    let mut error_records = 0;

    for (line_number, line) in input.lines().enumerate() {
        match line {
            Ok(line) => {
                let mut fields: Vec<String> = line.split('\t').map(String::from).collect();
                
                if fields.len() >= 12 {
                    let (query_fixed, target_fixed, query_displacement, target_displacement) = fix_endpoints(&mut fields);
                    if query_fixed || target_fixed {
                        fixed_records += 1;
                        if query_fixed {
                            query_fixed_count += 1;
                            total_query_displacement += query_displacement;
                        }
                        if target_fixed {
                            target_fixed_count += 1;
                            total_target_displacement += target_displacement;
                        }
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
    eprintln!("Query fixes: {}", query_fixed_count);
    eprintln!("Target fixes: {}", target_fixed_count);
    if fixed_records > 0 {
        eprintln!("Average query displacement: {:.2}", total_query_displacement as f64 / query_fixed_count as f64);
        eprintln!("Average target displacement: {:.2}", total_target_displacement as f64 / target_fixed_count as f64);
    }
    if error_records > 0 {
        eprintln!("Encountered errors in {} records", error_records);
    }
    Ok(())
}

fn fix_endpoints(fields: &mut Vec<String>) -> (bool, bool, usize, usize) {
    let mut query_fixed = false;
    let mut target_fixed = false;
    let mut query_displacement = 0;
    let mut target_displacement = 0;

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
                query_displacement = correct_query_end.abs_diff(query_end);
                fields[3] = correct_query_end.to_string();
                query_fixed = true;
            }

            let correct_target_end = target_start + target_length;
            if correct_target_end != target_end {
                target_displacement = correct_target_end.abs_diff(target_end);
                fields[8] = correct_target_end.to_string();
                target_fixed = true;
            }
        }
    }

    (query_fixed, target_fixed, query_displacement, target_displacement)
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
