use std::io::Read;
use std::fs::File;
use std::collections::HashMap;
use std::collections::HashSet;

// Read numbers from file
fn read_input(filename: &str) -> Vec<u64> {
    let mut file = File::open(filename).expect("File not found");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Error reading file");
    contents.lines().map(|x| x.parse().unwrap()).collect()
}

// Calculate next number in sequence
fn next_number(secret: &mut u64) {
    *secret ^= (*secret << 6) & 0xFFFFFF;
    *secret ^= (*secret >> 5) & 0xFFFFFF;
    *secret ^= (*secret << 11) & 0xFFFFFF;
}

// Get last digit of a number
fn digit(n: u64) -> i8 {
    (n % 10) as i8
}

// Convert sequence of differences to a number
fn seq2number(sequence: &Vec<i8>) -> u32 {
    let mut result: u32 = 0;
    for i in 0..4 {
        result <<= 1;
        if sequence[i] < 0 {
            result |= 1;
        }
        result <<= 5;
        result |= sequence[i].abs() as u32;
    }
    result
}

// Do n round of next number generation and update global map
fn do_n_rounds(secret: &mut u64, n: u64, global: &mut HashMap<u32, u32>) {
    let mut seen_sequences = HashSet::new();
    let mut prev_sequence = Vec::with_capacity(4);
    let mut prev_number = *secret;

    // Generate the first 3 numbers
    for _ in 0..3 {
        next_number(secret);
        prev_sequence.push(digit(*secret) - digit(prev_number));
        prev_number = *secret;
    }
    
    prev_sequence.push(0); // padding 

    // Generate the rest but also update global map
    for _ in 0..n-3 {
        // Generate next number, append it to sequence of differences
        next_number(secret);
        *prev_sequence.last_mut().unwrap() = digit(*secret) - digit(prev_number);
        prev_number = *secret;

        // Get number for this sequence
        let seqnum = seq2number(&prev_sequence);
        if seen_sequences.contains(&seqnum) {
            prev_sequence.rotate_left(1);
            continue;
        }
        // Not seen this sequence, update global map
        seen_sequences.insert(seqnum);
        let entry = global.entry(seqnum).or_insert(0);
        *entry += digit(*secret) as u32;
        // Shift sequence
        prev_sequence.rotate_left(1);
    }
}

fn main() {
    let mut secrets = read_input("input");

    let mut global = HashMap::new();
    let mut result = 0;

    for s in &mut secrets {
        do_n_rounds(s, 2000, &mut global);
        result += *s;
    }

    println!("Part 1: {}", result);
    println!("Part 2: {}", global.values().max().unwrap());
}
