use itertools::Itertools;



fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};

    let crabs = input.lines().flat_map(|l|l.trim().split(",").map(|n|n.parse::<usize>().unwrap())).collect_vec();

    let min = crabs.iter().min().unwrap();
    let max = crabs.iter().max().unwrap();
    let mut min_fuel = usize::MAX;
    for pos in *min..=*max{
        let fuel = crabs.iter().map(|c|c.abs_diff(pos)).sum::<usize>();
        min_fuel = min_fuel.min(fuel);
    }
    println!("{}",min_fuel);
    let mut min_fuel = usize::MAX;
    for pos in *min..=*max{
        let fuel = crabs.iter().map(|c|(c.abs_diff(pos)*(c.abs_diff(pos)+1)/2)).sum::<usize>();
        min_fuel = min_fuel.min(fuel);
    }
    println!("{}",min_fuel);
}