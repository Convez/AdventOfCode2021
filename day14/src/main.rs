use std::collections::HashMap;

use itertools::Itertools;
fn build_indices(polymer:String,insertion_rules: &HashMap<&str,char>)->Vec<(char,usize)>{
    let mut indices = vec![];
    for idx in 0..polymer.len()-1{
        let pair = &polymer[idx..=idx+1];
        if let Some(output) = insertion_rules.get(pair){
            indices.push((*output,idx+1));
        }
    }

    indices.iter().sorted_by_key(|i|i.1).map(|i|*i).collect_vec()
}

fn apply_rules(polymer: &str, insertion_rules: &HashMap<&str,char>, steps:usize)->String{
    let mut polymer = polymer.to_string();
    for _ in 1..=steps{
        let mut insertion_indices = build_indices(polymer.clone(), insertion_rules);
        while !insertion_indices.is_empty(){
            let (ch,idx) = insertion_indices.pop().unwrap();
            polymer.insert(idx, ch);
        }
    }
    polymer
}
fn find_paris(polymer: &str, insertion_rules: &HashMap<&str,char>)-> HashMap<String,usize>{
    let mut pairs_occurrences:HashMap<String,usize>= Default::default();
    for idx in 0..polymer.len()-1{
        let pair = &polymer[idx..=idx+1];
        if let Some(_) = insertion_rules.get(pair){
            pairs_occurrences.insert(pair.to_string(), pairs_occurrences.get(pair).unwrap_or(&0)+1);
        }
    }
    pairs_occurrences
}
fn apply_rules2(polymer: &str, insertion_rules: &HashMap<&str,char>, steps:usize)->HashMap<char, usize>{
    let polymer = polymer.to_string();

    // Find initial pairs from polymer
    let mut occurance_pairs = find_paris(polymer.as_str(), insertion_rules);
    // Set initial amount of characters
    let mut char_count: HashMap<char,usize> = Default::default();
    polymer.chars().for_each(|c|{char_count.insert(c, char_count.get(&c).unwrap_or(&0)+1);});
    for _ in 1..=steps{
        let mut new_pairs:HashMap<String,usize> = Default::default();
        for pair in occurance_pairs.iter_mut(){
            if let Some(ch) = insertion_rules.get(pair.0.as_str()){
                // From each pair, two new pairs spawn
                let mut new_str = pair.0.clone();
                new_str.insert(1, *ch);
                let left = &new_str[0..=1];
                let right = &new_str[1..=2];
                // We insert as many new pairs as there are occurrances of the old one
                new_pairs.insert(left.to_string(), new_pairs.get(left).unwrap_or(&0)+*pair.1);
                new_pairs.insert(right.to_string(), new_pairs.get(right).unwrap_or(&0)+*pair.1);
                // We insert one new char for each pair occurrance
                char_count.insert(*ch, char_count.get(ch).unwrap_or(&0)+*pair.1);
            }
        }
        occurance_pairs = new_pairs;
    }
    char_count
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let polymer = *input.lines().take(1).map(|l|l.trim()).collect_vec().first().unwrap();
    let insertion_rules = input.lines().skip(2).map(|l|sscanf::sscanf!(l.trim(),"{str} -> {char}").unwrap()).collect();

    let final_polymer = apply_rules(polymer, &insertion_rules, 10);
    let mut occurrences: HashMap<char,usize> = Default::default();
    final_polymer.chars().for_each(|c|{occurrences.insert(c, occurrences.get(&c).unwrap_or(&0)+1);});
    let max = occurrences.iter().max_by_key(|g|g.1).unwrap().1;
    let min = occurrences.iter().min_by_key(|g|g.1).unwrap().1;
    if is_test{
        println!("After step {} the polymer is: {}",10,polymer);
    }
    println!("After 10 steps, diff between most and least occurring is {}",max-min);
    let occurrences = apply_rules2(polymer, &insertion_rules, 40);
    let max = occurrences.iter().max_by_key(|g|g.1).unwrap().1;
    let min = occurrences.iter().min_by_key(|g|g.1).unwrap().1;
    println!("After 40 steps, diff between most and least occurring is {}",max-min);
    
}