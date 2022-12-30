use std::{collections::{HashMap}};

use itertools::Itertools;

#[derive(Debug,Clone)]
struct Octopus{
    pos:(i32,i32),
    level:usize,
    flashed:bool
}
impl Octopus {
    fn reset(&mut self){
        if self.level>9{
            self.level=0;
            self.flashed=false;
        }
    }
    fn charge(&mut self){
        self.level+=1;
    }
}

fn get_adj(oc: &Octopus)->Vec<(i32, i32)>{
    vec![
        (oc.pos.0-1,oc.pos.1-1),(oc.pos.0,oc.pos.1-1),(oc.pos.0+1,oc.pos.1-1),
        (oc.pos.0-1,oc.pos.1),(oc.pos.0+1,oc.pos.1),
        (oc.pos.0-1,oc.pos.1+1),(oc.pos.0,oc.pos.1+1),(oc.pos.0+1,oc.pos.1+1)
        ].iter().map(|i|*i)
        .collect_vec()
}
fn simulate1(map:&mut HashMap<(i32,i32),Octopus>, rounds:usize)->usize{
    let mut flashes = 0;
    for i in 0..rounds{
        //Step one, all octopuses increase level
        map.iter_mut().for_each(|os|os.1.charge());
        //Step 2, all octopuses with level more than 9 flashes until none flash
        loop {
            let to_update = map.iter()
            .filter(|o|o.1.level>9 && !o.1.flashed)
                .flat_map(|o|get_adj(o.1)).collect_vec();
            let flashing = map.iter().filter(|o|o.1.level>9 && !o.1.flashed)
                .map(|o|o.1.pos).collect_vec();
            to_update.iter().for_each(|o|{
                if let Some(octopus) = map.get_mut(o){
                    octopus.charge();
                }
            });
            flashes += flashing.len();
            flashing.iter().for_each(|o|{
                if let Some(octopus) = map.get_mut(o){
                    octopus.flashed=true;
                }
            });
            if map.iter().all(|o|o.1.flashed){
                println!("All flashes after {} rounds",i+1);
                return 0;
            }
            if flashing.is_empty(){
                break;
            }
        }
        //Step 3
        map.iter_mut().for_each(|o|o.1.reset());
    }
    flashes
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let mut map:HashMap<(i32,i32),Octopus>= input.lines().enumerate()
        .flat_map(|(row,l)|l.trim().chars().enumerate().map(move |(col,c)|((row as i32,col as i32),Octopus{ pos: (row as i32,col as i32), level: c.to_string().parse::<usize>().unwrap(), flashed: false }))).collect();
    let mut map2 = map.clone();
    // println!("{:?}",map);
    let flashes = simulate1(&mut map,100);
    println!("{}",flashes);
    simulate1(&mut map2,usize::MAX);
}