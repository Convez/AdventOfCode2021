use itertools::Itertools;

fn calc_increasing(v:&Vec<i32>)->i32{
    let mut larger = 0;
    for i in 1..v.len(){
        if v[i] > v[i-1] {
            larger += 1;
        }
    }
    larger
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let l = input.lines().map(|l|l.trim().parse::<i32>().unwrap()).collect_vec();
    // P1
    println!("{}",calc_increasing(&l));
    let mut intervals: Vec<i32> = Default::default();
    for i in 0..l.len()-2{
        let mut sum = 0;
        for y in 0..3{
            sum += l[i+y];
        }
        intervals.push(sum);
    }
    println!("{}",calc_increasing(&intervals));
}