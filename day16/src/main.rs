/*
--- Day 16: The Floor Will Be Lava ---
With the beam of light completely focused somewhere, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing empty space (.), mirrors (/ and \), and splitters (| and -).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into heat to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

.|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
The beam enters in the top-left corner from the left and heading to the right. Then, its behavior depends on what it encounters as it moves:

If the beam encounters empty space (.), it continues in the same direction.
If the beam encounters a mirror (/ or \), the beam is reflected 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a / mirror would continue upward in the mirror's column, while a rightward-moving beam that encounters a \ mirror would continue downward from the mirror's column.
If the beam encounters the pointy end of a splitter (| or -), the beam passes through the splitter as if the splitter were empty space. For instance, a rightward-moving beam that encounters a - splitter would continue in the same direction.
If the beam encounters the flat side of a splitter (| or -), the beam is split into two beams going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a | splitter would split into two beams: one that continues upward from the splitter's column and one that continues downward from the splitter's column.
Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is energized if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

>|<<<\....
|v-.\^....
.v...|->>>
.v...v^.|.
.v...v^...
.v...v^..\
.v../2\\..
<->-/vv|..
.|<<<2-|.\
.v//.|.v..
Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is energized (#) or not (.):

######....
.#...#....
.#...#####
.#...##...
.#...##...
.#...##...
.#..####..
########..
.#######..
.#...#.#..
Ultimately, in this example, 46 tiles become energized.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, how many tiles end up being energized?

Your puzzle answer was 8098.

The first half of this puzzle is complete! It provides one gold star: *

--- Part Two ---
As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from any edge tile and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that energizes as many tiles as possible.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

.|<2<\....
|v-v\^....
.v.v.|->>>
.v.v.v^.|.
.v.v.v^...
.v.v.v^..\
.v.v/2\\..
<-2-/vv|..
.|<<<2-|.\
.v//.|.v..
Using this configuration, 51 tiles are energized:

.#####....
.#.#.#....
.#.#.#####
.#.#.##...
.#.#.##...
.#.#.##...
.#.#####..
########..
.#######..
.#...#.#..
Find the initial beam configuration that energizes the largest number of tiles; how many tiles are energized in that configuration?
 */

use std::fs::read_to_string;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    UP,
    DOWN,
    WEST,
    EAST
}

fn next_tile(i: usize, j: usize, dir: Direction, size: usize) -> Option<(usize,usize,Direction)> {
    match dir {
        Direction::UP  => if i == 0      { None } else { Some((i-1, j, Direction::UP))   },
        Direction::DOWN=> if i == size-1 { None } else { Some((i+1, j, Direction::DOWN)) },
        Direction::WEST=> if j == 0      { None } else { Some((i, j-1, Direction::WEST)) },
        Direction::EAST=> if j == size-1 { None } else { Some((i, j+1, Direction::EAST)) },
    }
}

#[derive(Debug, Clone)]
struct Tile {
    c: char,
    energy: Vec<Direction>
}

fn load_contraption(input: &str) -> Vec<Vec<Tile>>{
    read_to_string(input).unwrap().split("\n")
    .map(|s| s.chars().map(|c| Tile{c, energy: Vec::new()}).collect::<Vec<Tile>>())
    .collect::<Vec<Vec<Tile>>>()
}

fn energizised(contraption: &mut Vec<Vec<Tile>>, entry_point: (usize, usize, Direction)) {
    let size = contraption.len();
    let mut stack = vec![entry_point];
    loop {
        if stack.len() == 0 {
            break;
        }

        let (i,j,dir) = stack.pop().unwrap();
        let tile = &mut contraption[i][j];
        if tile.energy.contains(&dir) {
            continue;
        }

        tile.energy.push(dir);
        match tile.c {
            '.'  => vec![next_tile(i,j,dir,contraption.len())],
            '\\' => match dir {
                        Direction::UP   => vec![next_tile(i,j,Direction::WEST,size)],
                        Direction::DOWN => vec![next_tile(i,j,Direction::EAST,size)],
                        Direction::WEST => vec![next_tile(i,j,Direction::UP,size)],
                        Direction::EAST => vec![next_tile(i,j,Direction::DOWN,size)],
            },
            '/' => match dir {
                        Direction::UP   => vec![next_tile(i,j,Direction::EAST,size)],
                        Direction::DOWN => vec![next_tile(i,j,Direction::WEST,size)],
                        Direction::WEST => vec![next_tile(i,j,Direction::DOWN,size)],
                        Direction::EAST => vec![next_tile(i,j,Direction::UP,size)],
            },
            '-' => match dir {
                    Direction::UP | Direction::DOWN  => vec![next_tile(i,j,Direction::WEST,size),next_tile(i,j,Direction::EAST,size)],
                    Direction::EAST | Direction::WEST => vec![next_tile(i,j,dir,size)],
            },
            '|' => match dir {
                Direction::UP | Direction::DOWN  => vec![next_tile(i,j,dir,size)],
                Direction::EAST | Direction::WEST => vec![next_tile(i,j,Direction::UP,size),next_tile(i,j,Direction::DOWN,size)],
            },
            _ => {panic!("oh noes /o\\");}
        }.into_iter().flatten().for_each(|beam| stack.push(beam));
    }
}

fn main() {
    let input = "day16/assets/input";
    let contraption = load_contraption(input);

    /* step 1
    energizised(&mut contraption, (0usize, 0usize, Direction::EAST));
    let energy_level = contraption.iter().flat_map(|r| r).filter(|t| t.energy.len() > 0).count();
    println!("sum is {}", energy_level);
    */

    /* step 2 */
    let mut configuration = Vec::new();
    (0..contraption.len()).into_iter().for_each(|i| {
        configuration.push((0,i,Direction::DOWN));
        configuration.push((i,0,Direction::EAST));
        configuration.push((contraption.len()-1,i,Direction::UP));
        configuration.push((i,contraption.len()-1,Direction::WEST));
    });

    let max = configuration.into_iter().map(|entry| {
        let mut copy = contraption.clone();
        energizised(&mut copy, entry);
        copy.iter().flat_map(|r| r).filter(|t| t.energy.len() > 0).count()
    }).max().unwrap();
    println!("max is {}", max);
}