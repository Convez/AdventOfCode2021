use std::collections::HashMap;

use itertools::Itertools;


fn decode_output(input:&Vec<&str>,output:&Vec<&str>)->usize{
    // Find easy nums
    let mut equivalences: HashMap<char,char> = Default::default();
    let mut num = 0;

    let one = input.iter().find(|s|s.len()==2).unwrap();
    let four = input.iter().find(|s|s.len()==4).unwrap();
    let seven = input.iter().find(|s|s.len()==3).unwrap();
    let a_char = seven.chars().into_iter().find(|c|!one.contains(*c)).unwrap();
    equivalences.insert(a_char, 'a');
    let size_five = input.iter().filter(|s|s.len()==5).flat_map(|s|s.chars()).join("");
    let size_five_unique = size_five.chars().filter(|c|size_five.chars().filter(|c1|c1==c).count()==1).join("");
    let e_char = size_five_unique.chars().find(|c|!one.contains(*c) && !four.contains(*c)&&!seven.contains(*c)).unwrap();
    equivalences.insert(e_char, 'e');
    let b_char = size_five_unique.chars().find(|c|!one.contains(*c)&&!equivalences.contains_key(c)).unwrap();
    equivalences.insert(b_char, 'b');
    let d_char = four.chars().find(|c|!one.contains(*c)&&!equivalences.contains_key(c)).unwrap();
    equivalences.insert(d_char, 'd');
    let six_size = input.iter().filter(|s|s.len()==6).flat_map(|s|s.chars()).join("");
    let g_char = six_size.chars().find(|c|!equivalences.contains_key(c)&&!one.contains(*c)).unwrap();
    equivalences.insert(g_char, 'g');
    
    let two = input.iter().filter(|s|s.len()==5).find(|s|s.contains(a_char)&&s.contains(d_char)&&s.contains(e_char)&&s.contains(g_char)).unwrap();
    let c_char = two.chars().find(|c|!equivalences.contains_key(c)).unwrap();
    equivalences.insert(c_char, 'c');
    let f_char = one.chars().find(|c|!equivalences.contains_key(c)).unwrap();
    equivalences.insert(f_char, 'f');
    
    //now we have all chars
    let num_eq:HashMap<_,_> = vec![("abcefg",0),("cf",1),("acdeg",2),("acdfg",3),("bcdf",4),("abdfg",5),("abdefg",6),("acf",7),("abcdefg",8),("abcdfg",9)].iter().map(|i|*i).collect();
    let binding = output.iter().map(|n|n.chars().map(|c|equivalences[&c]).sorted().join("")).collect_vec();
    
    let cyphers = binding.iter().map(|n|num_eq[n.as_str()]).rev().collect_vec();
    for i in 0..cyphers.len(){
        num += cyphers[i]*(10_i32.pow(i as u32));
    }
    num as usize
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let values = input.lines().map(|l|(l.trim().split("|").nth(0).unwrap().split_whitespace().collect_vec(),l.trim().split("|").nth(1).unwrap().split_whitespace().collect_vec())).collect_vec();

    let count_unique = values.iter().flat_map(|(_,o)|o)
        .filter(|o|o.len()== 2 || o.len()==3 || o.len()==4 || o.len()==7).count();
    println!("{}",count_unique);
    let count_total = values.iter().map(|(i,o)|decode_output(i, o)).sum::<usize>();
    println!("{}",count_total);
}