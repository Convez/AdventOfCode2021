use std::{iter::Skip, str::Lines};

use itertools::{Itertools, Chunk};

#[derive(Debug)]
struct BingoTile{
    nums: Vec<Vec<(i64,bool)>>
}
impl BingoTile {
    fn from_chunk(c: Chunk<Skip<Lines>>) -> Self {
        BingoTile{
            nums: c.filter(|l|!l.trim().is_empty()).map(|l|l.trim().split_whitespace().map(|n|(n.parse::<i64>().unwrap(),false)).collect_vec()).collect_vec(),
        }
    }

    fn mark_extracted(&mut self,num: i64){
        for row in 0..self.nums.len(){
            for col in 0..self.nums[row].len(){
                if self.nums[row][col].0 == num{
                    self.nums[row][col].1 = true;
                }
            }
        }
    }
    fn bingo_row(&self) -> bool{
        for row in 0..self.nums.len(){
            if self.nums[row].iter().all(|c|c.1){
                println!("Row bingo");
                return true;
            }
        }
        return false;
    }
    fn bingo_col(&self)->bool{
        for col in 0..self.nums[0].len(){
            let mut bingo = true;
            for row in 0..self.nums.len(){
                if !self.nums[row][col].1{
                    bingo = false;
                    break;
                }
            }
            if bingo{
                println!("Col bingo");
                return true;
            }
        }
        return false;
    }
    fn did_bingo(&self) -> bool{
        self.bingo_row() || self.bingo_col()
    }
    fn score(&self, extraction:i64) -> i64{
        let unmarked = self.nums.iter()
        .flat_map(|row|row.iter().filter_map(|(n,b)|{if *b {None} else {Some(n)}})).sum::<i64>();
        unmarked * extraction
    }

}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let extractions = input.lines().take(1)
    .flat_map(|l|l.trim().split(",").map(|n|n.parse::<i64>().unwrap())).collect_vec();

    let mut tiles = input.lines().skip(1).chunks(6).into_iter().map(|c|BingoTile::from_chunk(c)).collect_vec();
    for (turn, extraction) in extractions.iter().enumerate(){
        tiles.iter_mut().for_each(|t|t.mark_extracted(*extraction));
        if let Some(tile) = tiles.iter().find(|t|t.did_bingo()){
            println!("Bingo at turn {} {}",turn,tile.score(*extraction));
            tiles.retain(|t|!t.did_bingo());
            println!("{} tiles left", tiles.len());
        }
    }
}