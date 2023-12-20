/*
--- Day 14: Parabolic Reflector Dish ---
You reach the place where all of the mirrors were pointing: a massive parabolic reflector dish attached to the side of another large mountain.

The dish is made up of many small mirrors, but while the mirrors themselves are roughly in the shape of a parabolic reflector dish, each individual mirror seems to be pointing in slightly the wrong direction. If the dish is meant to focus light, all it's doing right now is sending it in a vague direction.

This system must be what provides the energy for the lava! If you focus the reflector dish, maybe you can go where it's pointing and use the light to fix the lava production.

Upon closer inspection, the individual mirrors each appear to be connected via an elaborate system of ropes and pulleys to a large metal platform below the dish. The platform is covered in large rocks of various shapes. Depending on their position, the weight of the rocks deforms the platform, and the shape of the platform controls which ropes move and ultimately the focus of the dish.

In short: if you move the rocks, you can focus the dish. The platform even has a control panel on the side that lets you tilt it in one of four directions! The rounded rocks (O) will roll when the platform is tilted, while the cube-shaped rocks (#) will stay in place. You note the positions of all of the empty spaces (.) and rocks (your puzzle input). For example:

O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
Start by tilting the lever so all of the rocks will slide north as far as they will go:

OOOO.#.O..
OO..#....#
OO..O##..O
O..#.OO...
........#.
..#....#.#
..O..#.O.O
..O.......
#....###..
#....#....
You notice that the support beams along the north side of the platform are damaged; to ensure the platform doesn't collapse, you should calculate the total load on the north support beams.

The amount of load caused by a single rounded rock (O) is equal to the number of rows from the rock to the south edge of the platform, including the row the rock is on. (Cube-shaped rocks (#) don't contribute to load.) So, the amount of load caused by each rock in each row is as follows:

OOOO.#.O.. 10
OO..#....#  9
OO..O##..O  8
O..#.OO...  7
........#.  6
..#....#.#  5
..O..#.O.O  4
..O.......  3
#....###..  2
#....#....  1
The total load is the sum of the load caused by all of the rounded rocks. In this example, the total load is 136.

Tilt the platform so that the rounded rocks all roll north. Afterward, what is the total load on the north support beams?
 */

use std::{fs::read_to_string, collections::HashMap};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Platform {   
    size: usize,
    rows: Vec<Vec<char>>,
    cols: Vec<Vec<char>>
}
 
fn load_platform(input: &str) -> Platform {
    let lines = read_to_string(input).unwrap();
    let size = lines.split("\n").count();
    let mut ret = Platform {
        size,
        rows: Vec::new(),
        cols: vec![Vec::new(); size]
    };

    for line in lines.split("\n") {
        ret.rows.push(line.chars().collect::<Vec<char>>());
        for (i,c) in line.chars().enumerate().into_iter() {
            ret.cols[i].push(c);
        }
    }

    ret
}

fn platform_rotate(in_: &Vec<Vec<char>>, out_: &mut Vec<Vec<char>>) {    
    for (i,l) in in_.iter().enumerate() {
        for (j,&c) in l.iter().enumerate() {
            out_[j][i] = c;
        }
    }
}


fn print_platform(p: &Platform) {
    for i in p.rows.iter() {
        println!("{} ", i.iter().collect::<String>());
    };
    println!("");
}

fn tilt_platform(p: &Platform, dir: usize, ret: &mut Platform) {       
    let (matrice, start, end, step, in_, out_) = match dir {
        0 =>  (&p.cols, 0i32, p.size as i32, 1i32, &mut ret.cols, &mut ret.rows), // north
        1 =>  (&p.rows, 0, p.size as i32, 1i32, &mut ret.rows, &mut ret.cols), // west
        2 =>  (&p.cols, (p.size-1) as i32, -1i32, -1, &mut ret.cols, &mut ret.rows),// south
        3 =>  (&p.rows, (p.size-1) as i32, -1i32, -1, &mut ret.rows, &mut ret.cols),// east
        _ => {panic!("oh noes /o\\");}
    };

    for (i,l) in matrice.iter().enumerate() {
        let mut j = start;
        let mut rock = start;
        loop {
            let c = l[j as usize];
            in_[i as usize][j as usize] = c;

            match c {
                'O' => {
                    in_[i as usize][j as usize] = '.';
                    in_[i as usize][rock as usize] = 'O';
                    rock += step;
                }
                '#' => {
                    rock = j as i32 + step;
                },
                _ => {}
            }
        
            j += step;
            if j == end {
                break;
            }
        }    
    }
   
    platform_rotate(in_, out_);
}

fn weight(p: &Platform) -> usize {
    let h = p.rows.len();
    p.rows.iter().enumerate().map(|(i,line)| {
        (h-i)*line.iter().filter(|&c| *c == 'O').count()
    })
    .sum()
}


fn main() {
    let input = "day14/assets/input";
    let p = load_platform(input);
    print_platform(&p);


    /* part 1 
    println!("----------\n");
    let t = tilt_platform(&p,0);
    print_platform(&t);
    println!("weight: {}",weight(&t));
    */

    /* part 2 */
    let mut t1 = p.clone();
    let mut t2 = p.clone();
    let mut cycles: HashMap<Platform, usize> = HashMap::new();
    let mut idx = 0;
    let mut it = 0;
    loop {
        let (i,j) = match it % 2 {
            0 => (&t1, &mut t2),
            1 => (&t2, &mut t1),
            _ => {panic!("oh noes /o\\")}
        };

        tilt_platform(i,(it % 4) as usize,  j);

        if let Some(&value) = cycles.get(i) {
            idx = value;
            break;
        } else {
            cycles.insert(i.clone(), it);
        }
        
        it += 1;
    }

    // we found a cycle!
    let cycle_size = it-idx;
    let cycling_part = 4000000000 - idx;
    let falls_at = idx + (cycling_part % cycle_size);
    let solution = cycles
    .iter()
    .find(|(_,&v)| {v == falls_at})
    .map(|(p,_)| weight(p))
    .unwrap();

    println!("found {}-cycle from idx={}, at={} solution={}",cycle_size, idx, falls_at, solution);
}
 
