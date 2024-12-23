use std::io::Read;
use std::collections::HashMap;

// Read input from file
fn read_input(filename: &str) -> String {
    let mut file = std::fs::File::open(filename).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    contents
}

// Return hashmap of towels contained in a sequence
fn seq_towels<'a>(towels: &Vec<&'a str>, seq: &str) -> HashMap<&'a str, bool> {
    let mut result = HashMap::new();
    for towel in towels {
        if seq.contains(towel) {
            result.insert(*towel, true);
        }
    }
    result
}

// Get all possible combinations of towels in a sequence
fn try_combination<'a>(towels: &HashMap<&str, bool>, sequence: &'a str, cache: &mut HashMap<&'a str, u64>) -> u64 {
    let mut found = 0;

    // Sequence has been solved already
    if cache.contains_key(sequence) {
        found += cache.get(sequence).unwrap();
        return found;
    }

    // Iterate through all starting positions of the sequence
    for i in 1..sequence.len()+1 {
        let subsequence = &sequence[0..i];
        if towels.contains_key(subsequence) {
            let new_subsequence = &sequence[i..];
            if new_subsequence.len() == 0 {
                // Solved subsequence
                cache.insert(sequence, found + 1);
                return found + 1;
            }
            let found_subseq = try_combination(towels, new_subsequence, cache);
            found += found_subseq;
        }
    }

    // Store results in cache
    if cache.contains_key(sequence) {
        cache.insert(sequence, found);
    } else {
        cache.insert(sequence, found);
    }

    found
}

fn main() {
    let x = read_input("input");
    let parts = x.split("\n\n").collect::<Vec<&str>>();
    
    let towels = parts[0].split(", ").collect::<Vec<&str>>();
    let sequences = parts[1].split("\n").collect::<Vec<&str>>();

    let mut cache = HashMap::new();

    let mut part1 = 0;
    let mut part2 = 0;
    for s in &sequences {
        let towels_contained = seq_towels(&towels, s);
        part2 += try_combination(&towels_contained, s, &mut cache);
        part1 += if *cache.get(s).unwrap() != 0 { 1 } else { 0 }; 
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);

}