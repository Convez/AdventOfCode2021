use std::{collections::{HashSet}, ops::Mul};

use itertools::Itertools;
fn get_adjacents(row:usize,col:usize,map:&Vec<Vec<usize>>)->Vec<usize>{
    let mut adj = vec![];
    if col > 0{
        adj.push(map[row][col-1]);
    }
    if row > 0{
        adj.push(map[row-1][col]);
    }
    if col < map[row].len()-1{
        adj.push(map[row][col+1]);
    }
    if row < map.len()-1{
        adj.push(map[row+1][col]);
    }
    adj
}
fn find_basin(row:usize,col:usize,map:&Vec<Vec<usize>>,seen:&mut HashSet<(usize,usize)>)->usize{
    seen.insert((row,col));
    let mut adj = vec![];
    if col > 0 && map[row][col-1]!=9{
        adj.push((row,col-1));
    }
    if row > 0 && map[row-1][col]!=9{
        adj.push((row-1,col));
    }
    if col < map[row].len()-1 && map[row][col+1]!=9{
        adj.push((row,col+1));
    }
    if row < map.len()-1&&map[row+1][col]!=9{
        adj.push((row+1,col));
    }
    let mut basin_size=1;
    for a in adj{
        if !seen.contains(&a){
            basin_size+=find_basin(a.0, a.1, map, seen);
        }
    } 
    basin_size
}
fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let map = input.lines().map(|l|l.trim().chars().map(|c|c.to_string().parse::<usize>().unwrap()).collect_vec()).collect_vec();
    let mut risk_level=0;
    let mut basins = vec![];
    for row in 0..map.len(){
        for col in 0..map[row].len(){
            let adj = get_adjacents(row, col, &map);
            if adj.iter().all(|a|*a>map[row][col]){
                risk_level += map[row][col]+1;
                let mut seen: HashSet<(usize,usize)> = Default::default();
                let basin_size = find_basin(row, col, &map,&mut seen);
                basins.push(basin_size);
            }
        }
    }
    println!("Risk level: {}",risk_level);
    let basins_score = basins.iter().sorted().rev().take(3).map(|i|*i).reduce(|acc,item|acc.mul(item)).unwrap();
    println!("Score of the top 3 basins is: {}",basins_score);
}