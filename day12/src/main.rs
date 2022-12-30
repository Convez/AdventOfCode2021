use std::{collections::{HashMap, HashSet, VecDeque}};

use itertools::Itertools;

#[derive(Debug,Clone,Default)]
struct State<'a>{
    path:Vec<&'a str>,
    visited:HashSet<&'a str>,
    special_visited:bool
}

fn calculate_paths(start:&str,end:&str,edges:&HashMap<&str,Vec<&str>>){
    let mut completed: Vec<State>=Default::default();
    let mut stack: VecDeque<State> = Default::default();
    let starting = State{
        path: vec![start],
        visited: vec![start].iter().map(|s|*s).collect(),
        special_visited:false
    };
    stack.push_back(starting);
    while let Some(State { path, visited, special_visited:false }) = stack.pop_front() {
        let current_cave = path.last().unwrap();
        if *current_cave==end{
            completed.push(State { path:path.clone(), visited:visited.clone(), special_visited:false });
            continue;
        }
        let connected = &edges[*current_cave];
        for cave in connected{
            if !visited.contains(*cave){
                // Then cave is eligible
                let mut new_visited = visited.clone();
                if cave.chars().nth(0).unwrap().is_lowercase(){
                    new_visited.insert(*cave);
                }
                let mut new_path = path.clone();
                new_path.push(cave);
                stack.push_back(State{path:new_path,visited:new_visited, special_visited:false});
            }
        }
    }
    
    println!("We calculated {} paths", completed.len());
}

fn calculate_paths2(start:&str,end:&str, caves:&HashSet<&str>,edges:&HashMap<&str,Vec<&str>>){
    let mut completed: Vec<State>=Default::default();
    let special_caves = caves.iter().filter(|s|**s!=start && **s!=end && s.chars().nth(0).unwrap().is_lowercase()).collect_vec();
    for special_cave in special_caves{
        let mut stack: VecDeque<State> = Default::default();
        let starting = State{
            path: vec![start],
            visited: vec![start].iter().map(|s|*s).collect(),
            special_visited: false
        };
        stack.push_back(starting);

        while let Some(State { path, visited, special_visited }) = stack.pop_front() {
            let current_cave = path.last().unwrap();
            if *current_cave==end{
                completed.push(State { path:path.clone(), visited:visited.clone(), special_visited });
                continue;
            }
            let connected = &edges[*current_cave];
            for cave in connected{
                if !visited.contains(*cave){
                    // Then cave is eligible

                    let mut new_visited = visited.clone();
                    let mut new_special_visited = special_visited;
                    if cave.chars().nth(0).unwrap().is_lowercase(){
                        if cave== special_cave {
                            if !special_visited{
                                new_special_visited = true;
                            }else{
                                new_visited.insert(*cave);
                            }
                        }else{
                            new_visited.insert(*cave);
                        }
                    }
                    let mut new_path = path.clone();
                    new_path.push(cave);
                    stack.push_back(State{path:new_path,visited:new_visited, special_visited:new_special_visited});
                }
            }
        }
    }
    let unique_paths = completed.iter().map(|c|c.path.clone()).unique().collect_vec();
    println!("We calculated {} paths", unique_paths.len());
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let caves: HashSet<&str> = input.lines().flat_map(|l|l.trim().split("-")).collect();
    let mut edges: HashMap<&str,Vec<&str>> = Default::default();
    caves.iter().for_each(|c|{ edges.insert(*c, Default::default()); });
    input.lines().map(|l|l.trim().split("-"))
    .map(|mut s|{(s.next().unwrap(),s.next().unwrap())}).for_each(|(c1,c2)|{
        let c1_e = edges.get_mut(c1).unwrap(); 
        c1_e.push(c2); 
        let c2_e = edges.get_mut(c2).unwrap();
        c2_e.push(c1);
    });
    calculate_paths("start", "end", &edges);
    calculate_paths2("start", "end", &caves, &edges);
}