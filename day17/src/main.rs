/*
--- Day 17: Clumsy Crucible ---
The lava starts flowing rapidly once the Lava Production Facility is operational. As you leave, the reindeer offers you a parachute, allowing you to quickly reach Gear Island.

As you descend, your bird's-eye view of Gear Island reveals why you had trouble finding anyone on your way up: half of Gear Island is empty, but the half below you is a giant factory city!

You land near the gradually-filling pool of lava at the base of your new lavafall. Lavaducts will eventually carry the lava throughout the city, but to make use of it immediately, Elves are loading it into large crucibles on wheels.

The crucibles are top-heavy and pushed by hand. Unfortunately, the crucibles become very difficult to steer at high speeds, and so it can be hard to go in a straight line for very long.

To get Desert Island the machine parts it needs as soon as possible, you'll need to find the best way to get the crucible from the lava pool to the machine parts factory. To do this, you need to minimize heat loss while choosing a route that doesn't require the crucible to go in a straight line for too long.

Fortunately, the Elves here have a map (your puzzle input) that uses traffic patterns, ambient temperature, and hundreds of other parameters to calculate exactly how much heat loss can be expected for a crucible entering any particular city block.

For example:

2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
Each city block is marked by a single digit that represents the amount of heat loss if the crucible enters that block. The starting point, the lava pool, is the top-left city block; the destination, the machine parts factory, is the bottom-right city block. (Because you already start in the top-left block, you don't incur that block's heat loss unless you leave that block and then return to it.)

Because it is difficult to keep the top-heavy crucible going in a straight line for very long, it can move at most three blocks in a single direction before it must turn 90 degrees left or right. The crucible also can't reverse direction; after entering each city block, it may only turn left, continue straight, or turn right.

One way to minimize heat loss is this path:

2>>34^>>>1323
32v>>>35v5623
32552456v>>54
3446585845v52
4546657867v>6
14385987984v4
44578769877v6
36378779796v>
465496798688v
456467998645v
12246868655<v
25465488877v5
43226746555v>
This path never moves more than three consecutive blocks in the same direction and incurs a heat loss of only 102.

Directing the crucible from the lava pool to the machine parts factory, but not moving more than three consecutive blocks in the same direction, what is the least heat loss it can incur?

--- Part Two ---
The crucibles of lava simply aren't large enough to provide an adequate supply of lava to the machine parts factory. Instead, the Elves are going to upgrade to ultra crucibles.

Ultra crucibles are even more difficult to steer than normal crucibles. Not only do they have trouble going in a straight line, but they also have trouble turning!

Once an ultra crucible starts moving in a direction, it needs to move a minimum of four blocks in that direction before it can turn (or even before it can stop at the end). However, it will eventually start to get wobbly: an ultra crucible can move a maximum of ten consecutive blocks without turning.

In the above example, an ultra crucible could follow this path to minimize heat loss:

2>>>>>>>>1323
32154535v5623
32552456v4254
34465858v5452
45466578v>>>>
143859879845v
445787698776v
363787797965v
465496798688v
456467998645v
122468686556v
254654888773v
432267465553v
In the above example, an ultra crucible would incur the minimum possible heat loss of 94.

Here's another example:

111111111111
999999999991
999999999991
999999999991
999999999991
Sadly, an ultra crucible would need to take an unfortunate path like this one:

1>>>>>>>1111
9999999v9991
9999999v9991
9999999v9991
9999999v>>>>
This route causes the ultra crucible to incur the minimum possible heat loss of 71.

Directing the ultra crucible from the lava pool to the machine parts factory, what is the least heat loss it can incur?

 */


 use std::{fs::read_to_string, collections::HashMap};
 use priority_queue::PriorityQueue;
 use std::cmp::Reverse;



 #[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
 enum Direction {
     UP,
     DOWN,
     LEFT,
     RIGHT
 }

 impl Direction {
    fn char(&self) -> &str {
        match self {
            Direction::UP => "U",
            Direction::DOWN => "D",
            Direction::LEFT => "L",
            Direction::RIGHT => "R",
        }
    }
 }
 
 fn next_tile(i: usize, j: usize, dir: Direction, map: &Vec<Vec<u8>>) -> Option<(usize,usize,Direction)> {
     match dir {
         Direction::UP    => if i == 0              { None } else { Some((i-1, j, Direction::UP))   },
         Direction::DOWN  => if i == map.len()-1    { None } else { Some((i+1, j, Direction::DOWN)) },
         Direction::LEFT  => if j == 0              { None } else { Some((i, j-1, Direction::LEFT)) },
         Direction::RIGHT => if j == map[0].len()-1 { None } else { Some((i, j+1, Direction::RIGHT)) },
     }
}
 
fn load_map(input: &str) -> Vec<Vec<u8>>{
     read_to_string(input).unwrap().split("\n")
     .map(|s| s.chars().map(|c|  c.to_digit(10).unwrap() as u8).collect::<Vec<u8>>())
     .collect::<Vec<Vec<u8>>>()
}

fn find_min_loss(entry: (usize,usize,Direction), map: &Vec<Vec<u8>>) -> usize{
    let mut state: HashMap<(usize,usize,Direction,usize),usize> = HashMap::new();
    let mut pq = PriorityQueue::new();
    pq.push((String::from("R"),Some(entry), 0, 0),Reverse(0));
    
    loop {
        if pq.is_empty() {
            break;
        }

        let (( walker_id, next, loss, straight), _) = pq.pop().unwrap();       
        if next.is_none() {
            continue;
        }
        
        let (i,j,dir) = next.unwrap();
        let loss = loss + map[i][j] as usize;
        
        if state.contains_key(&(i,j,dir,straight)) {
            continue;
        }
        
        state.insert((i,j,dir,straight), loss);

        if (i,j) == (map.len()-1, map[0].len()-1) && straight >= 3{
            break;
        }

        if straight < 9 {
            pq.push((walker_id.clone()+dir.char(), next_tile(i,j,dir,map), loss, straight+1),Reverse(loss));
        }
        
        if straight >= 3 || (i,j) == (0,0) {
            match dir {
                Direction::DOWN | Direction::UP => {
                    //println!("turning right");
                    pq.push((walker_id.clone()+"R", next_tile(i,j,Direction::RIGHT, map), loss, 0),Reverse(loss));
                    //println!("turning left");
                    pq.push((walker_id.clone()+"L", next_tile(i,j,Direction::LEFT,map), loss, 0),Reverse(loss));
                },
                Direction::LEFT | Direction::RIGHT => {
                    //println!("turning down");
                    pq.push((walker_id.clone()+"D", next_tile(i,j,Direction::DOWN,map), loss, 0),Reverse(loss));
                    //println!("turning up");
                    pq.push((walker_id.clone()+"U", next_tile(i,j,Direction::UP,map), loss, 0),Reverse(loss));
                }
            }
        }
    }

    *state.iter()
    .filter(|(&(i,j,_,s),_)| (i,j) == (map.len()-1,map[0].len()-1) && s >= 3)
    .map(|(_,v)| v)
    .min()
    .unwrap()
}

fn main() {
    let input = "day17/assets/input";
    let map = load_map(input);
    let min = find_min_loss((0,0,Direction::RIGHT), &map);    
    println!("min loss is {}", min - map[0][0] as usize);
}