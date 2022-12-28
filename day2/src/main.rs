
enum Direction{
    Horizontal,
    Vertical
}

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let mut horizontal =0;
    let mut vertical = 0;

    let mut horizontal2=0;
    let mut vertical2 =0;
    let mut aim = 0;

    input.lines().map(|l|{
        let (d,a) = sscanf::sscanf!(l.trim(),"{str} {i32}").unwrap();
        let dir = match d {
            "forward" => Direction::Horizontal,
            _ => Direction::Vertical
        };
        (dir, if d =="up"{-a} else {a})
    }).for_each(|(d,a)|{match d{
        Direction::Horizontal => {
            horizontal+=a; 
            horizontal2+=a;
            vertical2 += aim * a;
        },
        Direction::Vertical => {
            vertical+=a;
            aim += a;
        },
    } });
    println!("{}",horizontal*vertical);
    println!("{}",horizontal2*vertical2);



}