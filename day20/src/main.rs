use itertools::Itertools;



fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let alg = input.lines().take(1).flat_map(|l|l.trim().chars()).map(|c|c=='#').collect_vec();
    let mut image = input.lines().skip(2).map(|l|l.trim().chars().map(|c|c=='#').collect_vec()).collect_vec();
    for round in 0..50{
        let def = round%2==1&&alg[0];
        image = enhance(&image,&alg,def);
        println!();
        image.iter().for_each(|l|{
            l.iter().for_each(|c|print!("{}",if *c {'#'} else {'.'}));
            println!();
        });
    }
    
    let lit = image.iter().flat_map(|v|v).filter(|c|**c).count();
    println!("{}",lit);
}

fn enhance(image: &Vec<Vec<bool>>, alg:&Vec<bool>, default_char:bool) -> Vec<Vec<bool>>{
    let mut original = vec![vec![default_char;image[0].len()+2];1];
    for l in image{
        original.push(vec![default_char;1]);
        original.last_mut().unwrap().extend(l.iter());
        original.last_mut().unwrap().extend(vec![default_char;1]);
    }
    original.extend(vec![vec![default_char;image[0].len()+2];1]);
    let mut enhanced = vec![vec![default_char;original[0].len()];original.len()];
    for row in 0..original.len(){
    for col in 0..original[row].len(){
        let mut number = "".to_string();
        for i in -1i32..=1{
        for j in -1i32..=1{
            let ri = if i.is_negative(){row.checked_sub(i.abs().try_into().unwrap())} else {row.checked_add(i.abs().try_into().unwrap())};
            let cj = if j.is_negative(){col.checked_sub(j.abs().try_into().unwrap())} else {col.checked_add(j.abs().try_into().unwrap())};
            let char = ri.map_or(default_char, |r|cj.map_or(default_char, |c|original.get(r).map_or(default_char,|rw| *rw.get(c).unwrap_or(&default_char))));
            number.push(if !char {'0'} else {'1'});
        }
        }
        enhanced[row][col] = alg[usize::from_str_radix(number.as_str(), 2).unwrap()];
    }
    }
    enhanced
}