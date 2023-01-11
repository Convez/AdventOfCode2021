use std::collections::HashMap;

use itertools::Itertools;

#[derive(Debug,Default)]
struct DeterministicDie{
    rolls:usize,
    sides:usize,
    curr_roll:usize
}
impl DeterministicDie{
    fn new(sides:usize)->Self{
        let mut s:Self = Default::default();
        s.sides = sides;
        s
    }
    fn roll3(&mut self)->usize{
        let mut result = 0;
        for _ in 0..3{
            self.rolls+=1;
            self.curr_roll+=1;
            if self.curr_roll>self.sides{
                self.curr_roll=self.curr_roll%self.sides;
            }
            result+=self.curr_roll;
        }
        result
    }
}
#[derive(Default,Debug,Hash,Clone,PartialEq, Eq)]
struct Player{
    position:usize,
    score:usize,
    track_size:usize,
    win_condition:usize
}
impl Player {
    fn new(position:usize,track_size:usize,win_condition:usize)->Self{
        Player { position, score: 0, track_size, win_condition }
    }
    fn advance_by(&mut self, amount:usize){
        self.position = ((self.position + amount-1)%10)+1;
        self.score+=self.position;
    }
    fn won(&self)->bool{
        self.score>=self.win_condition
    }
}
fn trial_game(starting_pos:&Vec<usize>){
    let mut player1= Player::new(starting_pos[0], 10, 1000);
    let mut player2:Player = Player::new(starting_pos[1], 10, 1000);
    let mut die = DeterministicDie::new(100);
    loop {
        let advance = die.roll3();
        player1.advance_by(advance);
        if player1.won(){
            break;
        }
        let advance = die.roll3();
        player2.advance_by(advance);
        if player2.won(){
            break;
        }
    }
    let losing= if player1.won() {player2} else {player1};
    let score = losing.score as u32 * die.rolls as u32;
    println!("Losing player score: {}",losing.score);
    println!("Dice rolled {} times",die.rolls);
    println!("Score: {}",score);
}

#[derive(Debug,Clone, PartialEq, Eq,Hash)]
struct DiracGame{
    curr_player:usize,
    players:Vec<Player>
}
impl DiracGame {
    fn calculate_win(self, visited: &mut HashMap<DiracGame,[u64;2]>,dice_combinations:&Vec<(usize,usize)>)->[u64;2]{
        if visited.contains_key(&self){
            return visited[&self];
        }
        let mut result = [0u64,0u64];
        dice_combinations.iter().for_each(|amount|{
            let mut new_game = self.clone();
            new_game.players[new_game.curr_player].advance_by(amount.0);
            if new_game.players[new_game.curr_player].won(){
                // I win in all the universes in which I roll this value
                result[new_game.curr_player]+=amount.1 as u64;
            }else {
                new_game.curr_player = 1-new_game.curr_player;
                let partial_result = new_game.calculate_win(visited, dice_combinations);
                // This partial result happens in the amount of universes in which i roll the value
                result[0] += partial_result[0] * amount.1 as u64;
                result[1] += partial_result[1] * amount.1 as u64;
            }
        });
        visited.insert(self, result);
        result
    }
}

fn dirac_game(starting_pos:&Vec<usize>){
    let player1= Player::new(starting_pos[0], 10, 21);
    let player2:Player = Player::new(starting_pos[1], 10, 21);
    
    // We calculate the number of universes that can generate a certain value (1+1+1),(1+2+1),ecc.
    let dice_possibilities = (1usize..=3).flat_map(|one|(1..=3).flat_map(move |two|(1..=3).map(move |three|one+two+three))).collect_vec();
    let dice_combinations = dice_possibilities.iter().map(|i|(*i,dice_possibilities.iter().filter(|j|*i==**j).count())).unique().collect_vec();
    println!("{:?}",dice_combinations);
    let game = DiracGame{ curr_player: 0, players: [player1,player2].to_vec() };
    let wins = game.calculate_win(&mut HashMap::new(), &dice_combinations);
    println!("{:?}",wins);
    println!("{}", wins.iter().max().unwrap());

}
fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let starting_pos = input.lines().map(|l|sscanf::sscanf!(l.trim(),"Player {usize} starting position: {usize}").unwrap().1).collect_vec();
    trial_game(&starting_pos);
    dirac_game(&starting_pos);
}
