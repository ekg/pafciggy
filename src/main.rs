use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

fn main() -> std::io::Result<()> {
    let input_file = File::open("x.paf.bad")?;
    let output_file = File::create("x.paf.fixed")?;
    let reader = BufReader::new(input_file);
    let mut writer = BufWriter::new(output_file);

    for line in reader.lines() {
        let line = line?;
        let mut fields: Vec<String> = line.split('\t').map(String::from).collect();
        
        if fields.len() >= 12 {
            let query_start: usize = fields[2].parse().unwrap_or(0);
            let empty_string = String::new();
            let cigar = fields.last().unwrap_or(&empty_string);
            
            if cigar.starts_with("cg:Z:") {
                let correct_query_end = calculate_query_end(query_start, &cigar[5..]);
                fields[3] = correct_query_end.to_string();
            }
        }
        
        writeln!(writer, "{}", fields.join("\t"))?;
    }

    Ok(())
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
