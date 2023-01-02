use itertools::Itertools;

fn problem1(_min_x:f32,_max_x:f32,min_y:f32,_max_y:f32)->f32{
    let y0 = min_y.abs()-0.5;
    y0.powi(2)/2.0
}
fn problem2(min_x:f32,max_x:f32,min_y:f32,max_y:f32)->usize{
    let mut starting_velocities = 0;
    let abs_y = min_y.abs().max(max_y.abs()).floor() as i32;
    for start_vel_x in 0..max_x.floor() as i32+1{
        for start_vel_y in -abs_y..abs_y{
            // We simulate trajectory with these starting coords
                let mut x = 0.;
                let mut y = 0.;
                let mut vel_x = start_vel_x as f32;
                let mut vel_y = start_vel_y as f32;
            loop {
                // Simulation failed. Overshoot or we stopped horizontally before target cube
                if x > max_x{
                    break;
                }
                if vel_x==0. && !(min_x<=x && x<=max_x){
                    break;
                }
                if vel_x==0. && y<min_y{
                    break;
                }
                // We are in the target cube.OK
                if min_x<=x && x<=max_x && min_y<=y && y<=max_y{
                    starting_velocities+=1;
                    break;
                }
                
                x +=vel_x;
                y += vel_y;
                if vel_x>0. {
                    vel_x-=1.;
                }
                vel_y-=1.;
            }
        }
    }
    starting_velocities
}
fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    
    let (min_x,max_x,min_y,max_y) = sscanf::sscanf!(input.lines().collect_vec().first().unwrap().trim(),"target area: x={f32}..{f32}, y={f32}..{f32}").unwrap();
    let h_max = problem1(min_x, max_x, min_y, max_y);
    println!("{}",h_max.floor());
    let velocities = problem2(min_x, max_x, min_y, max_y);
    println!("{}",velocities);
}