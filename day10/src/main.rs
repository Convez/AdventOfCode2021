use itertools::Itertools;

fn get_expected(c:char)->char{
    match c{
        '{'=>'}',
        '['=>']',
        '<'=>'>',
        '('=>')',
        _=>todo!()
    }
}
fn get_corrupted_score(c:char)->usize{
    match c{
        '}'=>1197,
        ']'=>57,
        '>'=>25137,
        ')'=>3,
        _=>todo!()
    }
}
fn get_incompelte_score(c:char)->usize{
    match c{
        '}'=>3,
        ']'=>2,
        '>'=>4,
        ')'=>1,
        _=>todo!()
    }
}
fn find_corrupted_line_score(line: &str)->(usize,bool){
    let mut opens = vec![];
    for c in line.chars(){
        match c {
            '{'=>{opens.push(c)},
            '['=>{opens.push(c)},
            '<'=>{opens.push(c)},
            '('=>{opens.push(c)},
            '}'=>{let expected = get_expected(opens.pop().unwrap()); if c != expected{return (get_corrupted_score(c),true);}},
            ']'=>{let expected = get_expected(opens.pop().unwrap()); if c != expected{return (get_corrupted_score(c),true);}},
            '>'=>{let expected = get_expected(opens.pop().unwrap()); if c != expected{return (get_corrupted_score(c),true);}},
            ')'=>{let expected = get_expected(opens.pop().unwrap()); if c != expected{return (get_corrupted_score(c),true);}},
            _=>()
        }
    }
    return (0,false);
}
fn find_incomplete_line_score(line: &str)->usize{
    let mut opens = vec![];
    for c in line.chars(){
        match c {
            '{'=>{opens.push(c);},
            '['=>{opens.push(c);},
            '<'=>{opens.push(c);},
            '('=>{opens.push(c);},
            '}'=>{opens.pop();},
            ']'=>{opens.pop();},
            '>'=>{opens.pop();},
            ')'=>{opens.pop();},
            _=>()
        }
    }
    let completition_score = opens.iter().rev()
    .map(|c|get_incompelte_score(get_expected(*c)))
    .reduce(|a,i|a*5+i).unwrap();

    return completition_score;
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let score_corrupted = input.lines().map(|l|find_corrupted_line_score(l).0).sum::<usize>();
    println!("{}",score_corrupted);
    let score_incomplete = input.lines().
        filter_map(|l|if find_corrupted_line_score(l).1{None}else{Some(l)})
        .map(|l|find_incomplete_line_score(l)).sorted().collect_vec();
    println!("{}",score_incomplete[score_incomplete.len()/2]);
}