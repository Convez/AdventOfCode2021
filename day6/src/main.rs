use std::{collections::{BTreeMap}};

use itertools::Itertools;



fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let mut fishes = input.lines().flat_map(|l|l.trim().split(",").map(|n|n.parse::<i32>().unwrap())).collect_vec();
    let fishes2 = fishes.clone();
    let days = 80;
    for _ in 0..days{
        let mut new_fishes = vec![];
        for fish in 0..fishes.len(){
            if fishes[fish] == 0{
                //Reset
                fishes[fish] = 6;
                new_fishes.push(8);
            }else{
                fishes[fish] -=1;
            }
        }
        fishes.extend(new_fishes.iter());
    }
    println!("After {} days there are {} fishes",days,fishes.len());
    let mut fishes_opt: BTreeMap<i32,usize> = Default::default();
    for i in 0..=8{
        fishes_opt.insert(i, fishes2.iter().filter(|f|**f==i).count());
    }
    println!("Inital state: {:?}", fishes_opt);
    let days = 256;
    for _ in 0..=days{
        let fishes_zero = fishes_opt[&0];
        for i in 0..8{
            fishes_opt.insert(i, fishes_opt[&(i+1)]);
        }
        fishes_opt.insert(8, fishes_zero);
        fishes_opt.insert(6, fishes_opt[&6]+fishes_zero);
    }
    let mut count = 0;
    for i in 0..8{
        count += fishes_opt[&i];
    }
    println!("After {} days count is {}",days,count);
}