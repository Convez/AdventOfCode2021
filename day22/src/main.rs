use std::ops::RangeInclusive;

use itertools::Itertools;

#[derive(Debug, Clone)]
struct Cuboid {
    x_span: RangeInclusive<isize>,
    y_span: RangeInclusive<isize>,
    z_span: RangeInclusive<isize>,
    status: bool,
}
impl Cuboid {
    fn is_valid(&self) -> bool {
        !self.x_span.is_empty() && !self.y_span.is_empty() && !self.z_span.is_empty()
    }
    fn from_line(l: &str) -> Self {
        let (status, x1, x2, y1, y2, z1, z2) = sscanf::sscanf!(
            l.trim(),
            "{str} x={isize}..{isize},y={isize}..{isize},z={isize}..{isize}"
        )
        .unwrap();
        Cuboid {
            x_span: x1..=x2,
            y_span: y1..=y2,
            z_span: z1..=z2,
            status: if status == "on" { true } else { false },
        }
    }
    fn execute(&self, cube: &mut Vec<Vec<Vec<bool>>>) {
        for x in self.x_span.clone() {
            for y in self.y_span.clone() {
                for z in self.z_span.clone() {
                    cube[(x + 50) as usize][(y + 50) as usize][(z + 50) as usize] = self.status;
                }
            }
        }
    }
    fn range_overlaps(&self, other: &Cuboid) -> bool {
        !(other.x_span.start().to_owned() > self.x_span.end().to_owned()
            || other.x_span.end().to_owned() < self.x_span.start().to_owned()
            || other.y_span.start().to_owned() > self.y_span.end().to_owned()
            || other.y_span.end().to_owned() < self.y_span.start().to_owned()
            || other.z_span.start().to_owned() > self.z_span.end().to_owned()
            || other.z_span.end().to_owned() < self.z_span.start().to_owned())
    }
    fn remove_overlapping_region(&self, other: &Cuboid) -> Vec<Cuboid> {
        vec![
            Cuboid {
                x_span: (*self.x_span.start()..=(other.x_span.start() - 1)),
                y_span: self.y_span.clone(),
                z_span: self.z_span.clone(),
                status: self.status,
            },
            Cuboid {
                x_span: ((other.x_span.end() + 1)..=*self.x_span.end()),
                y_span: self.y_span.clone(),
                z_span: self.z_span.clone(),
                status: self.status,
            },
            Cuboid {
                x_span: *self.x_span.start().max(other.x_span.start())
                    ..=*self.x_span.end().min(other.x_span.end()),
                y_span: (*self.y_span.start()..=(other.y_span.start() - 1)),
                z_span: self.z_span.clone(),
                status: self.status,
            },
            Cuboid {
                x_span: *self.x_span.start().max(other.x_span.start())
                    ..=*self.x_span.end().min(other.x_span.end()),
                y_span: (other.y_span.end() + 1)..=*self.y_span.end(),
                z_span: self.z_span.clone(),
                status: self.status,
            },
            Cuboid {
                x_span: *self.x_span.start().max(other.x_span.start())
                    ..=*self.x_span.end().min(other.x_span.end()),
                y_span: *self.y_span.start().max(other.y_span.start())
                    ..=*self.y_span.end().min(other.y_span.end()),
                z_span: (*self.z_span.start()..=(other.z_span.start() - 1)),
                status: self.status,
            },
            Cuboid {
                x_span: *self.x_span.start().max(other.x_span.start())
                    ..=*self.x_span.end().min(other.x_span.end()),
                y_span: *self.y_span.start().max(other.y_span.start())
                    ..=*self.y_span.end().min(other.y_span.end()),
                z_span: (other.z_span.end() + 1)..=*self.z_span.end(),
                status: self.status,
            },
        ].into_iter().filter(|c|c.is_valid()).collect()
    }
    fn calc_volume(&self)->u64{
        self.x_span.to_owned().count() as u64 *
        self.y_span.to_owned().count() as u64 *
        self.z_span.to_owned().count() as u64 
    }
}
fn range_overlaps(r1: &RangeInclusive<isize>, r2: &RangeInclusive<isize>) -> bool {
    return std::cmp::max(r1.start(), r2.start()) < std::cmp::min(r1.end(), r2.end());
}
#[derive(Debug)]
struct Cube {
    cuboids: Vec<Cuboid>,
}
impl Cube {
    fn score(&self)->u64{
        self.cuboids.iter().map(|c|c.calc_volume()).sum::<u64>()
    }
}
fn get_volume(cuboids: &Vec<Cuboid>)->u64{
    let mut cube = Cube { cuboids: vec![] };
    let mut i = 1;
    for cuboid in cuboids {
        println!("Instruction {}", i);
        i+=1;
        println!("Cuboids number: {}",cube.cuboids.len());
        if cuboid.status {
            // Cuboid is on, so we need to add it
            let mut split = vec![cuboid.clone()];
            for cube_cuboid in &cube.cuboids {
                let mut new_splits = vec![];
                for s in &split{
                    if s.range_overlaps(cube_cuboid) {
                        // We then remove all overlapping regions, which are already included in the new cuboid
                        new_splits.extend(s.remove_overlapping_region(cube_cuboid));
                    } else {
                        new_splits.push(s.clone());
                    }
                }
                split = new_splits;
            }
            cube.cuboids.extend(split);
        } else {
            let mut new_cuboids: Vec<Cuboid> = vec![];
            // The new cuboid is off, so we just need to remove the overlapping region from other cuboids
            for cube_cuboid in &cube.cuboids {
                if cube_cuboid.range_overlaps(cuboid) {
                    new_cuboids.extend(cube_cuboid.remove_overlapping_region(&cuboid));
                }else{
                    new_cuboids.push(cube_cuboid.clone());
                }
            }   
            cube.cuboids = new_cuboids;
        }
    }
    cube.score()
}

fn main() {
    let is_test = false;
    let input = if is_test {
        include_str!("input_tst.txt")
    } else {
        include_str!("input.txt")
    };
    let cuboids = input.lines().map(Cuboid::from_line).collect_vec();
    let cuberange = -50..=50;
    let mut cube_map = vec![vec![vec![false; 101]; 101]; 101];
    cuboids
        .iter()
        .filter(|c| {
            range_overlaps(&cuberange, &c.x_span)
                && range_overlaps(&cuberange, &c.y_span)
                && range_overlaps(&cuberange, &c.z_span)
        })
        .for_each(|c| c.execute(&mut cube_map));
    println!(
        "{}",cube_map.iter()
            .flat_map(|x| x.iter().flat_map(|y| y.iter().map(|z| *z as usize)))
            .sum::<usize>());
    println!("{}",get_volume(&cuboids));
}
