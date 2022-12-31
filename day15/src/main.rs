use itertools::Itertools;
use petgraph::{graph::{DiGraph, EdgeReference}, algo::dijkstra};

fn path_from_map(map: &Vec<Vec<i32>>)->i32{

    let mut g = DiGraph::<(usize,usize),i32>::new();
    let nodes = map.iter().enumerate().map(|(row,r)|r.iter().enumerate().map(|(col,_)|g.add_node((row,col))).collect_vec()).collect_vec();

    for row in 0..map.len(){
        for col in 0..map[0].len()-1{
            g.add_edge(nodes[row][col], nodes[row][col+1],map[row][col+1]);
            g.add_edge(nodes[row][col+1], nodes[row][col],map[row][col]);
        }
    }
    for col in 0..map[0].len(){
        for row in 0..map.len()-1{
            g.add_edge(nodes[row][col], nodes[row+1][col], map[row+1][col]);
            g.add_edge(nodes[row+1][col], nodes[row][col], map[row][col]);
        }
    }
    
    let node_map = dijkstra(&g, nodes[0][0], Some(nodes[nodes.len()-1][nodes[nodes.len()-1].len()-1]), |e: EdgeReference<_>| *e.weight());
    let cost = node_map.get(&nodes[nodes.len()-1][nodes[nodes.len()-1].len()-1]).unwrap();
    *cost
}
fn new_tile(map:&Vec<Vec<i32>>)->Vec<Vec<i32>>{
    map.iter().map(|r|r.iter().map(|c|(c+1)%10).map(|c|if c==0{1}else {c}).collect()).collect()
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let map = input.lines().map(|l|l.trim().chars().map(|c|c.to_string().parse::<i32>().unwrap()).collect_vec()).collect_vec();
    let cost = path_from_map(&map);
    println!("Cost is {}",cost);
    let mut final_map = map.clone();
    let mut tile = map.clone();
    for _ in 0..4{
        tile = new_tile(&tile);
        final_map.extend(tile.iter().map(|v|v.clone()));
    }
    tile = final_map.clone();
    for _ in 0..4{
        tile = new_tile(&tile);
        for i in 0..tile.len(){
            final_map[i].extend(tile[i].clone());   
        }
    }
    let cost = path_from_map(&final_map);
    println!("Cost of the full map is {}",cost);
}