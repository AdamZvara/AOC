use std::fs::File;
use std::io::Read;
use std::collections::HashMap;
use std::collections::HashSet;

type Stone = u64;
type StoneCnt = u64;

// Read stones from file
fn read_stones(filename: &str) -> Vec<Stone> {
    let mut file = File::open(filename).unwrap();
    let mut text = String::new();
    file.read_to_string(&mut text).unwrap();
    text.split_whitespace().map(|line| line.parse().unwrap()).collect()
}

// Blink a single stone as per defined rules and return new sequence of stones
fn blink_stone(stone: Stone, stones: &mut Vec<Stone>) {
    if stone == 0 {
        stones.push(1);
        return;
    } 
    let stone_str = stone.to_string();
    let stone_str_len = stone_str.len();
    if stone_str_len % 2 == 0 {
        let halves = stone_str.split_at(stone_str_len / 2);
        stones.push(halves.0.parse().unwrap());
        stones.push(halves.1.parse().unwrap());
    } else {
        stones.push(stone * 2024);
    }
}

// Blink with a sequence of stones returning new sequence of items
fn blink_stones(stones: &Vec<Stone>) -> Vec<Stone> {
    let mut new_stones = Vec::new();
    stones.iter().for_each(|&stone| blink_stone(stone, &mut new_stones));
    new_stones
}

// Blink stone n times and return number of elements after n blinks
fn blink_stone_n_cnt(stone: Stone, n: u8) -> StoneCnt {
    let mut new_stones = vec![stone];
    for _ in 0..n {
        new_stones = blink_stones(&new_stones);
    }
    new_stones.len() as u64
}

// Fill stone map with stone sequence after `n` iterations and number of stones after `m` iterations (m > n) 
fn fill_map(stones: &Vec<Stone>, map: &mut HashMap<Stone, (StoneCnt, Vec<Stone>)>, n: u8, m: u8) {
    let mut new_stones = Vec::new();
    for s in stones.iter() {
        if map.contains_key(s) {
                continue;
            } 
        new_stones.clear();
        new_stones.push(*s);
        let mut stone_sequence = Vec::new();
        for i in 0..m {
            new_stones = blink_stones(&new_stones);
            if i == n - 1 {
                stone_sequence = new_stones.clone();
            } 
        }
        map.insert(*s, (new_stones.len() as StoneCnt, stone_sequence));        
    }
}

fn main() {
    let mut stone_map: HashMap<Stone, (StoneCnt, Vec<Stone>)> = HashMap::new();
    let mut stones: Vec<Stone> = read_stones("input11");
    let blinks_part1 = 25;
    let blinks_part2 = 75;

    // Solution 1
    let mut stones_part1 = stones.clone();
    for _ in 0..blinks_part1 {
        stones_part1 = blink_stones(&stones_part1)
    }
    println!("Part 1: {}", stones_part1.len());

    /* Solution 2
    Separate iterations into 3 stages:
    1st stage: do 30 iterations as they are fast but also for each visited stone, store sequence after 
               `second_stage` (20) number of iterations and length of sequence after `third_stone` iterations (25)
    2nd stage: remove duplicates and calculate new sequence (if stone was visited, this is instant)
    3rd stage: the same as in 2nd stage but now we only care for the overall number of stones in last sequence
    */
    let blinks_first_stage = 30; 
    let blinks_second_stage = 20;
    let blinks_third_stage = blinks_part2 - blinks_first_stage - blinks_second_stage;

    // First stage
    for _ in 0..blinks_first_stage {
        fill_map(&stones, &mut stone_map, blinks_second_stage, blinks_third_stage);
        stones = blink_stones(&stones);
    }

    // Get unique stones from first 30 iterations
    let unique_stones: HashSet<_> = stones.iter().copied().collect();
    let mut sum = 0;

    // Second stage
    for s in unique_stones {
        // How many times stone appears
        let s_count = stones.iter().filter(|&x| *x == s).count() as u64;
        let mut s_sequence;
        if stone_map.contains_key(&s) {
            // Stone sequence after 20 iterations has been done in part 1, use it directly
            s_sequence = stone_map.get(&s).unwrap().1.clone();
        } else {
            // Stone sequence after 20 iterations has not been done, do it
            s_sequence = vec![s];
            for _ in 0..blinks_second_stage {
                s_sequence = blink_stones(&s_sequence);
            }
        }

        // Do unique stones after second stage
        let unique_stones2: HashSet<_> = s_sequence.iter().copied().collect();
        
        let mut s_sum = 0;
        for s2 in unique_stones2 {
            // How many times stone appears in second stage
            let s_count2 = s_sequence.iter().filter(|&x| *x == s2).count() as u64;
            if stone_map.contains_key(&s2) {
                // Stone was found in map, get the final sequence length directly
                s_sum += s_count2 * stone_map.get(&s2).unwrap().0;
            } else {
                // Stone was not found, calculate the final sequence
                s_sum += s_count2 * blink_stone_n_cnt(s2, blinks_third_stage);
            }
        }

        sum += s_count * s_sum;
    }

    println!("Part 2: {}", sum);

}
