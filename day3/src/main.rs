use itertools::Itertools;

fn get_bits_rating(nums: &Vec<&str>) -> Vec<(i32,i32)>{
    let mut bits =  vec![(0,0);nums[0].len()];
    nums.iter().for_each(|l|l.chars().enumerate().for_each(|(i,c)|{
        if c == '0' {
            bits[i].0 +=1;
        }else{
            bits[i].1 +=1;
        }
    }));
    bits
}


fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let nums = input.lines().map(|l|l.trim()).collect_vec();
    let bits =  get_bits_rating(&nums);
    let mut gamma = "".to_owned();
    let mut epsilon = "".to_owned();
    bits.iter().for_each(|b|{
        if b.0>b.1{
            gamma.push('0');
            epsilon.push('1');
        }else{
            gamma.push('1');
            epsilon.push('0');
        }
    });
    println!("{}",isize::from_str_radix(gamma.as_str(), 2).unwrap()*isize::from_str_radix(epsilon.as_str(), 2).unwrap());
    let mut oxygen = nums.clone();
    let mut co2 = nums.clone();
    for i in 0..oxygen[0].len(){
        if oxygen.len()>1{
            let oxygen_bits = get_bits_rating(&oxygen);
            let char = if oxygen_bits[i].0 <= oxygen_bits[i].1 {'1'} else {'0'};
            oxygen = oxygen.iter().filter(|l|{l.chars().nth(i).unwrap() == char}).map(|l|*l).collect_vec();
        }
    }
    for i in 0..co2[0].len(){
        if co2.len()>1{
            let co2_bits = get_bits_rating(&co2);
            let char = if co2_bits[i].0 <= co2_bits[i].1 {'0'} else {'1'};
            co2 = co2.iter().filter(|l|{l.chars().nth(i).unwrap()==char}).map(|l|*l).collect_vec();
        }
    }
    println!("{}",isize::from_str_radix(oxygen[0], 2).unwrap()*isize::from_str_radix(co2[0], 2).unwrap());
    
}