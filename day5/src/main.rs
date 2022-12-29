use itertools::{Itertools};
use std::cmp::{min,max};

fn print_map(map:&Vec<Vec<i32>>){
    for row in map{
        for col in row{
            print!("{}",col);
        }
        println!();
    }
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let ranges = input.lines().map(|l|{
        sscanf::sscanf!(l.trim(),"{usize},{usize} -> {usize},{usize}").unwrap()
    }).collect_vec();
    let max_x = ranges.iter().flat_map(|i|vec![i.0,i.2]).max().unwrap();
    let max_y = ranges.iter().flat_map(|i|vec![i.1,i.3]).max().unwrap();
    let mut map = vec![vec![0;max_x+1];max_y+1];
    ranges.iter().filter(|i|i.0 == i.2 || i.1 == i.3)
    .for_each(|(x1,y1,x2,y2)|{
        if x1 == x2{
            for i in *min(y1, y2)..=*max(y1, y2){
                map[i][*x1] +=1;
            }
        }else{
            for i in *min(x1,x2)..=*max(x1,x2){
                map[*y1][i] +=1;
            }
        }
    });
    if is_test{
        print_map(&map);
    }
    let at_least_two = map.iter().flat_map(|p|p.iter()).filter(|p|**p>=2).count();
    println!("Number of point with at least 2 overlap is: {}",at_least_two);
    ranges.iter().filter(|i|!(i.0 == i.2 || i.1 == i.3)).for_each(
        |(x1,y1,x2,y2)|{
            let shift_horizontal = (*x2 as i32-*x1 as i32).signum();
            let shift_vertical = (*y2 as i32-*y1 as i32).signum();
            for i in 0..=max(x1, x2)-min(x1, x2){
                map[(*y1 as i32 +i as i32*shift_vertical)as usize][(*x1 as i32+i as i32*shift_horizontal)as usize]+=1;
            }
        }
    );
    if is_test{
        print_map(&map);
    }
    let at_least_two_complete = map.iter().flat_map(|p|p.iter()).filter(|p|**p>=2).count();
    println!("Number of point with at least 2 overlap is: {}",at_least_two_complete);
    
}