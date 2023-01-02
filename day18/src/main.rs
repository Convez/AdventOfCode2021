
use itertools::Itertools;

fn add(num1: &mut Vec<(i32,u32)>, other:&mut Vec<(i32,u32)>){
    num1.append(other);
    num1.iter_mut().for_each(|n|n.0+=1);
    reduce(num1);
}

fn explode(n: &mut Vec<(i32,u32)>)->bool{
    // Worst case, Last number is either part of a couple that needs to be exploded 
    //(we check first number of the pair)
    // Or a number that needs to be split. No need to check it
    for i in 0..n.len()-1{
        let (depth,value_left_pair) = n[i];
        if depth > 4{
            let (_,value_right_pair) = n[i+1];
            // Change pair with single 0
            n[i] = (depth-1,0);
            n.remove(i+1);
            // We update value to the left
            n.get_mut(i.overflowing_sub(1).0).map(|left|left.1+=value_left_pair);
            // We update value to the right
            n.get_mut(i+1).map(|right|right.1+=value_right_pair);
            return true;
        }
    }
    return false;
}

fn split(n: &mut Vec<(i32,u32)>)->bool{
    for i in 0..n.len(){
        let (depth,value) = n[i];
        if value>=10{
            n[i] = (depth+1,value/2);
            n.insert(i+1, (depth+1,value/2+value%2));
            return true;
        }
    }
    return false;
}
fn reduce(n: &mut Vec<(i32,u32)>){
    if explode(n) || split(n){
        reduce(n);
    }
}

fn magnitude(idx: &mut usize, depth: i32, n: &Vec<(i32,u32)>) -> u32 {
    3 * if n[*idx].0 == depth {
        *idx += 1;
        n[*idx - 1].1
    } else {
        magnitude(idx, depth + 1, n)
    } + 2 * if n[*idx].0 == depth {
        *idx += 1;
        n[*idx - 1].1
    } else {
        magnitude(idx, depth + 1, n)
    }
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let mut numbers = input.lines().map(
        |l|l.trim().chars().fold((0, vec![]), |(mut depth, mut acc),c|{
            match c{
                '[' => depth+=1,
                ']' => depth-=1,
                ',' => {},
                _ => acc.push((depth,c.to_string().parse::<u32>().unwrap()))
            };
            (depth,acc)
        }).1
    ).collect_vec();

    let binding = numbers.clone();
    let perms = binding.iter().permutations(2).collect_vec();
    
    let sum = numbers.iter_mut().reduce(|acc,n|{add(acc, n);acc}).unwrap();
    println!("Sum is: {:?}",sum);
    let mag = magnitude(&mut 0, 1, &sum);
    println!("And its magnitude is: {}",mag);
    let hightest_mag = perms.iter().map(|perm|{
        let mut p1 = perm[0].clone();
        let mut p2 = perm[1].clone();
        add(&mut p1,&mut p2);
        magnitude(&mut 0, 1, &p1)
    }).max().unwrap();
    println!("The highest magnitude possible is: {}", hightest_mag);
}