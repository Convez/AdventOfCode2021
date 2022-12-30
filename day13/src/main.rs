fn print_map(map: &Vec<Vec<char>>){
    for y in map{
        for x in y{
            print!("{x}");
        }
        println!();
    }
}
fn fold(map: &Vec<Vec<char>>,direction:&str, from:usize)-> Vec<Vec<char>>{
    if direction == "x"{
        // Fold vertically
        let mut new_map = vec![vec!['.';map[0].len()/2];map.len()];
        for i in 0..map.len(){
            for j in 0..from{
                if new_map[i][j] == '.'{
                    new_map[i][j] = map[i][j];
                }
                if new_map[i][j] == '.'{
                    new_map[i][j] = map[i][map[i].len()-1-j];
                }
            }
        }
        return new_map;
    }else{
        // Fold horizontally
        let mut new_map = vec![vec!['.';map[0].len()];map.len()/2];
        for i in 0..from{
            for j in 0..map[i].len(){
                if new_map[i][j] == '.'{
                    new_map[i][j] = map[i][j];
                }
                if new_map[i][j] == '.'{
                    new_map[i][j] = map[map.len()-1-i][j];
                }
            }
        }
        return new_map;
    }
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let mut sheet = true;
    let mut points : Vec<(usize,usize)> = Default::default();
    let mut folds: Vec<(&str,usize)> = Default::default();
    for line in input.lines(){
        if line.is_empty() {
            sheet = false;
            continue;
        }
        if sheet{
            points.push(sscanf::sscanf!(line.trim(),"{usize},{usize}").unwrap()); 
        }else{
            folds.push(sscanf::sscanf!(line.trim(),"fold along {str}={usize}").unwrap());
        }
    }
    let max_x = points.iter().max_by_key(|(x,_)|x).unwrap().0;
    let max_y = points.iter().max_by_key(|(_,y)|y).unwrap().1;
    let mut map = vec![vec!['.';max_x+1];max_y+1];
    points.iter().for_each(|(x,y)|map[*y][*x]='#');
    
    
    for (direction,from) in folds{
        map = fold(&map, direction, from);
        println!("Fold along {}={}",direction,from);
        let filled = map.iter().flat_map(|i|i).filter(|c|**c=='#').count();
        println!("At this point there are {} filled spaces",filled);
    }
    print_map(&map);
}