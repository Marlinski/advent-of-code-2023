/*
--- Day 18: Lavaduct Lagoon ---
Thanks to your efforts, the machine parts factory is one of the first factories up and running since the lavafall came back. However, to catch up with the large backlog of parts requests, the factory will also need a large supply of lava for a while; the Elves have already started creating a large lagoon nearby for this purpose.

However, they aren't sure the lagoon will be big enough; they've asked you to take a look at the dig plan (your puzzle input). For example:

R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
The digger starts in a 1 meter cube hole in the ground. They then dig the specified number of meters up (U), down (D), left (L), or right (R), clearing full 1 meter cubes as they go. The directions are given as seen from above, so if "up" were north, then "right" would be east, and so on. Each trench is also listed with the color that the edge of the trench should be painted as an RGB hexadecimal color code.

When viewed from above, the above example dig plan would result in the following loop of trench (#) having been dug out from otherwise ground-level terrain (.):

#######
#.....#
###...#
..#...#
..#...#
###.###
#...#..
##..###
.#....#
.######
At this point, the trench could contain 38 cubic meters of lava. However, this is just the edge of the lagoon; the next step is to dig out the interior so that it is one meter deep as well:

#######
#######
#######
..#####
..#####
#######
#####..
#######
.######
.######
Now, the lagoon can contain a much more respectable 62 cubic meters of lava. While the interior is dug out, the edges are also painted according to the color codes in the dig plan.

The Elves are concerned the lagoon won't be large enough; if they follow their dig plan, how many cubic meters of lava could it hold?

--- Part Two ---
The Elves were right to be concerned; the planned lagoon would be much too small.

After a few minutes, someone realizes what happened; someone swapped the color and instruction parameters when producing the dig plan. They don't have time to fix the bug; one of them asks if you can extract the correct instructions from the hexadecimal codes.

Each hexadecimal code is six hexadecimal digits long. The first five hexadecimal digits encode the distance in meters as a five-digit hexadecimal number. The last hexadecimal digit encodes the direction to dig: 0 means R, 1 means D, 2 means L, and 3 means U.

So, in the above example, the hexadecimal codes can be converted into the true instructions:

#70c710 = R 461937
#0dc571 = D 56407
#5713f0 = R 356671
#d2c081 = D 863240
#59c680 = R 367720
#411b91 = D 266681
#8ceee2 = L 577262
#caa173 = U 829975
#1b58a2 = L 112010
#caa171 = D 829975
#7807d2 = L 491645
#a77fa3 = U 686074
#015232 = L 5411
#7a21e3 = U 500254
Digging out this loop and its interior produces a lagoon that can hold an impressive 952408144115 cubic meters of lava.

Convert the hexadecimal color codes into the correct instructions; if the Elves follow this new dig plan, how many cubic meters of lava could the lagoon hold?

*/

 use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT
}

impl From<char> for Direction {
    fn from(input: char) -> Self {
        match input {
            'U' => Direction::UP,
            'L' => Direction::LEFT,
            'D' => Direction::DOWN,
            'R' => Direction::RIGHT,
            _ => panic!("forbidden char"),
        }
    }
}


fn next_tile(i: usize, j: usize, dir: Direction, map: &Vec<Vec<char>>) -> Option<(usize,usize)> {
    match dir {
        Direction::UP    => if i == 0              { None } else { Some((i-1, j)) },
        Direction::DOWN  => if i == map.len()-1    { None } else { Some((i+1, j)) },
        Direction::LEFT  => if j == 0              { None } else { Some((i, j-1)) },
        Direction::RIGHT => if j == map[0].len()-1 { None } else { Some((i, j+1)) },
    }
}

type Plan = Vec<(Direction, usize, (u8,u8,u8))>;

fn load_plan(input: &str) -> Plan{
    read_to_string(input).unwrap().split("\n")
    .map(|line| {
        let s: Vec<&str> = line.split(" ").collect();
        let r = u8::from_str_radix(&s[2][2..4], 16).unwrap();
        let g = u8::from_str_radix(&s[2][4..6], 16).unwrap();
        let b = u8::from_str_radix(&s[2][6..8], 16).unwrap();
        (Direction::from(s[0].chars().nth(0).unwrap()), s[1].parse().unwrap(), (r,g,b))
    })
    .collect::<Vec<(Direction, usize, (u8,u8,u8))>>()
}

fn print_trenches(p: &Vec<Vec<char>>) {
    for i in p.iter() {
        println!("{}", i.iter().collect::<String>());
    };
    println!("");
}

fn dig_trenches(plan: &Plan) -> (Vec<Vec<char>>, Vec<(usize,usize,Direction)>) {
    // first we get the dimension
    let (mut i, mut j) = (0,0);
    let (mut min_u,mut max_d,mut min_l,mut max_r) = (0,0,0,0);
    plan.iter().for_each(|&(dir,length,_)| {
        match dir {
            Direction::UP   => i -= length as i32,
            Direction::DOWN => i += length as i32,
            Direction::LEFT => j -= length as i32,
            Direction::RIGHT=> j += length as i32,           
        }
        (min_u,max_d,min_l,max_r) = (i32::min(min_u,i),i32::max(max_d,i),i32::min(min_l,j),i32::max(max_r,j));
    });

    // then we dig
    let mut ret_map = vec![vec!['.';(max_r-min_l+1) as usize];(max_d-min_u+1) as usize];
    let mut ret_path = Vec::new();
    let (mut i, mut j) = (-1*min_u,-1*min_l);
    plan.iter().for_each(|&(dir,length,_)| {
        (0..length).into_iter().for_each(|_| {
            ret_path.push((i as usize,j as usize, dir));
            ret_map[i as usize][j as usize] = '#';
            match dir {
                Direction::UP   => i -= 1,
                Direction::DOWN => i += 1,
                Direction::LEFT => j -= 1,
                Direction::RIGHT=> j += 1, 
            }
        });
    });
    (ret_map,ret_path)
}

fn paint_trenches((map,path): &(Vec<Vec<char>>, Vec<(usize,usize, Direction)>)) -> Vec<Vec<char>> {
    let mut map = map.clone();
    let mut stack = Vec::new();
    
    path.iter().for_each(|&(i,j, dir)| {
        if let Some((inside_i, inside_j)) = match dir {
            Direction::UP   => next_tile(i, j, Direction::RIGHT, &map),
            Direction::DOWN => next_tile(i, j, Direction::LEFT, &map),
            Direction::LEFT => next_tile(i, j, Direction::UP, &map),
            Direction::RIGHT=> next_tile(i, j, Direction::DOWN, &map), 
        } {
            if map[inside_i][inside_j] == '.' {
                stack.push(Some((inside_i, inside_j)))
            }
        }
    });

    loop {
        if stack.len() == 0 {
            break;
        }

        let o = stack.pop().unwrap();
        if o.is_none() {
            continue;
        }

        let (i,j) = o.unwrap();
        if map[i][j] != '.' {
            continue;
        }

        map[i][j] = '#';
        stack.push( next_tile(i, j, Direction::UP, &map));
        stack.push( next_tile(i, j, Direction::DOWN, &map));
        stack.push( next_tile(i, j, Direction::LEFT, &map));
        stack.push( next_tile(i, j, Direction::RIGHT, &map));                
    }
    map
}

/* part 2 */

fn load_plan_2(input: &str) -> Vec<(i64, i64)>{
    let (mut x,mut y) = (0i64,0i64);
    read_to_string(input).unwrap().split("\n")
    .map(|line| {
        let s: Vec<&str> = line.split(" ").collect();
        let dst = u32::from_str_radix(&s[2][2..7], 16).unwrap();
        let dir = s[2].chars().nth(7).unwrap();
        (x,y) = match dir {
            '0' => (x+dst as i64, y),
            '1' => (x,y+dst as i64),
            '2' => (x-dst as i64,y),
            '3' => (x,y-dst as i64),
            _ => { panic!("panic /o\\"); }
        };
        (x,y)
    })
    .collect::<Vec<(i64, i64)>>()
}


fn main() {
    /* part 1 */
    let input = "day18/assets/input";    
    let plan = load_plan(input);
    let trenches = dig_trenches(&plan);
    //print_trenches(&trenches.0);
    let painted = paint_trenches(&trenches);
    //print_trenches(&painted);
    let cubic = painted.iter().flat_map(|v| v).filter(|&c| c == &'#').count();
    println!("cubic is {}", cubic);

    /* part 2 */
    let plan = load_plan_2(input);
    let cubic = plan.iter().fold((0,0,0), |(acc,x1,y1),&(x2,y2)| {
        println!("{} {} {} {}",x1,y1,x2,y2);
        // shoelace formulae + width of the line (distance)
        let acc = acc + x1*y2-x2*y1 + ((((x2-x1)*(x2-x1)+(y2-y1)*(y2-y1)) as f64).sqrt()) as i64;
        (acc,x2,y2)
    }).0/2 + 1;
    println!("cubic is {}", cubic);
}