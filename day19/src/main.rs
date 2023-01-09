use std::{collections::{HashMap, VecDeque}, cmp::Ordering};

use itertools::Itertools;
use nalgebra::{Vector3, Matrix3};

fn manhattan(c1:Vector3<i32>,c2:Vector3<i32>)->i32{
    (c1.x-c2.x).abs() + (c1.y-c2.y).abs() + (c1.z-c2.z).abs()
}
#[derive(Debug,Default,Clone)]
struct Scanner{
    id: usize,
    beacons: Vec<Vector3<i32>>,
    coords: Option<Vector3<i32>>,
    rotation: Option<Matrix3<i32>>
}
impl Scanner {
    fn from_id(id:usize)-> Self{
        let mut s:Self = Default::default();
        s.id = id;
        s
    }
    fn get_manhattan(&self) -> Vec<i32> {
        let mut d = vec![];
        for i in 0..self.beacons.len()-1{
            for j in i+1..self.beacons.len(){
                d.push(manhattan(self.beacons[i], self.beacons[j]));
            }
        }
        d.sort();
        d
    }
    fn is_candidate_of(&self, reference:&Scanner)->bool{
        let distances = self.get_manhattan();
        let ref_dist = reference.get_manhattan();
        let mut iter1 = distances.iter().peekable();
        let mut iter2 = ref_dist.iter().peekable();
        let mut equal_count = 0;
        while iter1.peek().is_some() && iter2.peek().is_some() {
            if iter1.peek().unwrap() == iter2.peek().unwrap(){
                equal_count +=1;
                iter1.next(); iter2.next();
            }else if iter1.peek().unwrap() > iter2.peek().unwrap(){
                iter2.next();
            }else {
                iter1.next();
            }
        }
        equal_count >= OVERLAP_DISTANCE

    }
}
const OVERLAP_BEACONS:usize = 12;
const OVERLAP_DISTANCE:usize = 66; //12 * 11 / 2 <- Unordered combinations formula

fn main(){
    let is_test = false;
    let input = if is_test {include_str!("input_tst.txt")} else {include_str!("input.txt")};
    let scanners = parse(input);
    let aligned = to_aligned(&scanners);
    assert!(aligned.len() == scanners.len(), "Not all scanners were aligned");
    let unique_beacons = aligned.values()
    .flat_map(|s|s.beacons.iter().map(|b|(s.rotation.unwrap()*b)).map(|b|s.coords.unwrap()+b))
    .unique().count();
    println!("{}",unique_beacons);
    let mut distances = vec![];
    for i in 0..aligned.len()-1{
        for j in i+1..aligned.len(){
            distances.push(manhattan(aligned[&i].coords.unwrap(), aligned[&j].coords.unwrap()));
        }
    }
    println!("Largest distance between two scanners is: {}",distances.iter().max().unwrap());
}
fn to_aligned(scanners: &HashMap<usize,Scanner>)->HashMap<usize,Scanner>{
    let mut scanners = scanners.clone();
    let mut scanner0 = scanners.remove(&0).unwrap();
    let mut unaligned = scanners.clone();
    // Let's use scanner 0 as reference for all
    scanner0.coords = Some(Vector3::new(0, 0, 0));
    scanner0.rotation = Some(Matrix3::identity());

    let mut aligned: HashMap<usize,Scanner> = Default::default();
    aligned.insert(scanner0.id, scanner0.clone());
    let mut seen: VecDeque<Scanner> = Default::default();
    seen.push_back(scanner0);
    while let Some(reference) = seen.pop_front() {
        for al in &aligned{
            unaligned.remove(&al.1.id);
        }
        // We get the candidates
        let candidates = unaligned.values().into_iter().filter(|s|s.is_candidate_of(&reference)).collect_vec();
        println!("For scanner {} we have the following candidat1es: {}",reference.id, candidates.iter().map(|s|s.id.to_string()).join(", "));
        for candidate in &candidates{
            let mut candidate = (*candidate).clone();
            if let Ok(()) = try_align(&mut candidate,&reference){
                aligned.insert(candidate.id, candidate.clone());
                seen.push_back(candidate);
            }
        }
    }
    aligned
}
/// To align we need to:
/// - We find a scanner that has beacons with the same distance to each other 
///     - The min beacons that need to overlap are 12. This means the min distances (combination) is 66
/// - Once we find a candidate, for all possible rotations
/// - We rotate the candidate beacons and we check that the diff in the coords are the same
/// - Once we find a candidate rotation
///   We figure out if the box is the offset of the scanner 
///   (aka, offset that makes the readings align) (for this we need 12 scanners to overlap)
fn try_align(scanner:&mut Scanner,reference:&Scanner)->Result<(),()>{
    let ref_beacons = reference.beacons.iter().map(|b|reference.rotation.unwrap()*b).sorted_by(cmp).collect_vec();
    let ref_diff = vectorial_differences(&ref_beacons);
    let mut candidate_rotation = None;
    for rotation in generate_rotations(){
        let rotated_beacons:Vec<Vector3<i32>> = scanner.beacons.iter().map(|b|rotation*b).sorted_by(cmp).collect();
        let rotated_diff = vectorial_differences(&rotated_beacons);
        let overlap = count_equal_vectors(&ref_diff,&rotated_diff);
        println!("Between scanner {} and {} there are {} overlaps",scanner.id,reference.id,overlap);
        if overlap >= OVERLAP_DISTANCE{
            println!("Found rotation: {}",rotation);
            candidate_rotation = Some(rotation);
            break;
        }
    }
    if candidate_rotation.is_none(){
        return Err(());
    }
    let mut candidate_offset = None;
    let rotated_candidates = scanner.beacons.iter().map(|b|candidate_rotation.unwrap()*b).sorted_by(cmp).collect_vec();
    'label: for ref_beacon in &ref_beacons{
        for rotated in &rotated_candidates{
            let offset = ref_beacon-rotated;
            let shifted = rotated_candidates.iter().map(|b|b+offset).sorted_by(cmp).collect_vec();
            let shift_overlap = count_equal_vectors(&shifted, &ref_beacons);
            if shift_overlap>= OVERLAP_BEACONS{
                println!("Found offset: {}",offset);
                candidate_offset = Some(offset);
                break 'label;
            }

        }
    }
    if candidate_offset.is_none(){
        return Err(());
    }
    println!("We found both rotation and offset");
    scanner.coords = Some(reference.coords.unwrap() + candidate_offset.unwrap());
    scanner.rotation = Some(candidate_rotation.unwrap());
    return Ok(());
}

fn vectorial_differences(beacons: &Vec<Vector3<i32>>) -> Vec<Vector3<i32>>{
    let mut diff:Vec<Vector3<i32>> = Default::default();
    for i in 0..beacons.len()-1{
        for j in i+1..beacons.len(){
            diff.push(beacons[i]-beacons[j]);
        }
    }
    diff.sort_by(cmp);
    diff
}
fn count_equal_vectors(v1:&Vec<Vector3<i32>>,v2:&Vec<Vector3<i32>>)->usize{
    let mut iter1 = v1.iter().peekable();
    let mut iter2 = v2.iter().peekable();
    let mut count = 0;
    while iter1.peek().is_some() && iter2.peek().is_some() {
        if cmp(iter1.peek().unwrap(), iter2.peek().unwrap()) == Ordering::Equal{
            count +=1;
            iter1.next(); iter2.next();
        } else if cmp(iter1.peek().unwrap(), iter2.peek().unwrap()) ==  Ordering::Greater{
            iter2.next();
        } else {
            iter1.next();
        }
    }
    count
}
fn cmp(v1:&Vector3<i32>,v2:&Vector3<i32>)->Ordering{
    if v1.x == v2.x {
        if v1.y == v2.y {
            if v1.z == v2.z { Ordering::Equal
            }else if v1.z > v2.z { Ordering::Greater
            }else { Ordering::Less}
        }else if v1.y > v2.y {Ordering::Greater
        }else {Ordering::Less}
    }else if v1.x > v2.x {Ordering::Greater
    }else {Ordering::Less}
}
fn generate_rotations()->Vec<Matrix3<i32>>{
    vec![[1,0,0],[0,1,0],[0,0,1]].into_iter()
    .permutations(3).map(|v|Matrix3::from_iterator(v.concat().into_iter()))
    .flat_map(|m|vec![m,multiply_axis(m, 0, -1)])
    .flat_map(|m|vec![m,multiply_axis(m, 1, -1)])
    .flat_map(|m|vec![m,multiply_axis(m, 2, -1)])
    .filter(|m|determinant(m)==1)
    .collect_vec()
}
fn determinant(m:&Matrix3<i32>)->i32{
    m[0] * (m[4]*m[8]-m[7]*m[5])- 
    m[3]*(m[1]*m[8]-m[7]*m[2])+
    m[6]*(m[1]*m[5]-m[4]*m[2])
}
fn multiply_axis(m: Matrix3<i32>,axis_idx:usize,amount:i32) -> Matrix3<i32>{
    let mut m = m.clone();
    for i in 0..3 {m[i*3+axis_idx]*=amount;}
    m
}
fn parse(input:&str)->HashMap<usize,Scanner>{
    let mut map: HashMap<usize,Scanner> = Default::default();
    let mut curr = 0;
    for line in input.lines(){
        if let Ok(id) = sscanf::sscanf!(line.trim(),"--- scanner {usize} ---"){
            curr = id;
            map.insert(curr, Scanner::from_id(id));
        }
        if let Ok((x,y,z)) = sscanf::sscanf!(line.trim(),"{i32},{i32},{i32}"){
            map.get_mut(&curr).unwrap().beacons.push(Vector3::new(x, y, z));
        }
    }
    map
}