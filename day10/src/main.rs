/*
--- Day 10: Pipe Maze ---
You use the hang glider to ride the hot air from Desert Island all the way up to the floating metal island. This island is surprisingly cold and there definitely aren't any thermals to glide on, so you leave your hang glider behind.

You wander around for a while, but you don't find any people or animals. However, you do occasionally find signposts labeled "Hot Springs" pointing in a seemingly consistent direction; maybe you can find someone at the hot springs and ask them where the desert-machine parts are made.

The landscape here is alien; even the flowers and trees are made of metal. As you stop to admire some metal grass, you notice something metallic scurry away in your peripheral vision and jump into a big pipe! It didn't look like any animal you've ever seen; if you want a better look, you'll need to get ahead of it.

Scanning the area, you discover that the entire field you're standing on is densely packed with pipes; it was hard to tell at first because they're the same metallic silver color as the "ground". You make a quick sketch of all of the surface pipes you can see (your puzzle input).

The pipes are arranged in a two-dimensional grid of tiles:

| is a vertical pipe connecting north and south.
- is a horizontal pipe connecting east and west.
L is a 90-degree bend connecting north and east.
J is a 90-degree bend connecting north and west.
7 is a 90-degree bend connecting south and west.
F is a 90-degree bend connecting south and east.
. is ground; there is no pipe in this tile.
S is the starting position of the animal; there is a pipe on this tile, but your sketch doesn't show what shape the pipe has.
Based on the acoustics of the animal's scurrying, you're confident the pipe that contains the animal is one large, continuous loop.

For example, here is a square loop of pipe:

.....
.F-7.
.|.|.
.L-J.
.....
If the animal had entered this loop in the northwest corner, the sketch would instead look like this:

.....
.S-7.
.|.|.
.L-J.
.....
In the above diagram, the S tile is still a 90-degree F bend: you can tell because of how the adjacent pipes connect to it.

Unfortunately, there are also many pipes that aren't connected to the loop! This sketch shows the same loop as above:

-L|F7
7S-7|
L|7||
-L-J|
L|-JF
In the above diagram, you can still figure out which pipes form the main loop: they're the ones connected to S, pipes those pipes connect to, pipes those pipes connect to, and so on. Every pipe in the main loop connects to its two neighbors (including S, which will have exactly two pipes connecting to it, and which is assumed to connect back to those two pipes).

Here is a sketch that contains a slightly more complex main loop:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here's the same example sketch with the extra, non-main-loop pipe tiles also shown:

7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
If you want to get out ahead of the animal, you should find the tile in the loop that is farthest from the starting position. Because the animal is in the pipe, it doesn't make sense to measure this by direct distance. Instead, you need to find the tile that would take the longest number of steps along the loop to reach from the starting point - regardless of which way around the loop the animal went.

In the first example with the square loop:

.....
.S-7.
.|.|.
.L-J.
.....
You can count the distance each tile in the loop is from the starting point like this:

.....
.012.
.1.3.
.234.
.....
In this example, the farthest point from the start is 4 steps away.

Here's the more complex loop again:

..F7.
.FJ|.
SJ.L7
|F--J
LJ...
Here are the distances for each tile on that loop:

..45.
.236.
01.78
14567
23...
Find the single giant loop starting at S. How many steps along the loop does it take to get from the starting position to the point farthest from the starting position?

--- Part Two ---
You quickly reach the farthest point of the loop, but the animal never emerges. Maybe its nest is within the area enclosed by the loop?

To determine whether it's even worth taking the time to search for such a nest, you should calculate how many tiles are contained within the loop. For example:

...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........
The above loop encloses merely four tiles - the two pairs of . in the southwest and southeast (marked I below). The middle . tiles (marked O below) are not in the loop. Here is the same loop again with those regions marked:

...........
.S-------7.
.|F-----7|.
.||OOOOO||.
.||OOOOO||.
.|L-7OF-J|.
.|II|O|II|.
.L--JOL--J.
.....O.....
In fact, there doesn't even need to be a full tile path to the outside for tiles to count as outside the loop - squeezing between pipes is also allowed! Here, I is still within the loop and O is still outside the loop:

..........
.S------7.
.|F----7|.
.||OOOO||.
.||OOOO||.
.|L-7F-J|.
.|II||II|.
.L--JL--J.
..........
In both of the above examples, 4 tiles are enclosed by the loop.

Here's a larger example:

.F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
The above sketch has many random bits of ground, some of which are in the loop (I) and some of which are outside it (O):

OF----7F7F7F7F-7OOOO
O|F--7||||||||FJOOOO
O||OFJ||||||||L7OOOO
FJL7L7LJLJ||LJIL-7OO
L--JOL7IIILJS7F-7L7O
OOOOF-JIIF7FJ|L7L7L7
OOOOL7IF7||L7|IL7L7|
OOOOO|FJLJ|FJ|F7|OLJ
OOOOFJL-7O||O||||OOO
OOOOL---JOLJOLJLJOOO
In this larger example, 8 tiles are enclosed by the loop.

Any tile that isn't part of the main loop can count as being enclosed by the loop. Here's another example with many bits of junk pipe lying around that aren't connected to the main loop at all:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
Here are just the tiles that are enclosed by the loop marked with I:

FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJIF7FJ-
L---JF-JLJIIIIFJLJJ7
|F|F-JF---7IIIL7L|7|
|FFJF7L7F-JF7IIL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
In this last example, 10 tiles are enclosed by the loop.

Figure out whether you have time to search for the nest by calculating the area within the loop. How many tiles are enclosed by the loop?

*/

use std::fs::read_to_string;
use std::ops::Index;
use std::ops::IndexMut;

#[derive(Debug,PartialEq, Eq)]
enum Direction {
  UP,
  DOWN,
  LEFT,
  RIGHT
}

impl Direction {
  fn rotate(&self, c: char) -> Direction {
    match (self,c) {
      (Direction::UP,'|') => Direction::UP,
      (Direction::DOWN,'|') => Direction::DOWN,
      (Direction::LEFT,'-') => Direction::LEFT,
      (Direction::RIGHT,'-') => Direction::RIGHT,
      (Direction::DOWN,'L') => Direction::RIGHT,
      (Direction::LEFT,'L') => Direction::UP,
      (Direction::RIGHT,'J') => Direction::UP,
      (Direction::DOWN,'J') => Direction::LEFT,
      (Direction::RIGHT,'7') => Direction::DOWN,
      (Direction::UP,'7') => Direction::LEFT,
      (Direction::UP,'F') => Direction::RIGHT,
      (Direction::LEFT,'F') => Direction::DOWN,
      _ => panic!("wrong piping")
    }
  }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Coord {
  x: i32,
  y: i32,
}

impl Coord {
  fn up(&self) -> Coord {
    Coord{x:self.x, y:self.y-1}
  }
  fn down(&self) -> Coord {
    Coord{x:self.x, y:self.y+1}
  }
  fn left(&self) -> Coord {
    Coord{x:self.x-1, y:self.y}
  }
  fn right(&self) -> Coord {
    Coord{x:self.x+1, y:self.y}
  }
}

#[derive(Debug)]
struct Maze {
  start: Coord,
  map: Vec<Vec<char>>
} 

impl Maze {
  fn exists(&self, coord: &Coord) -> bool {
    coord.x >= 0 && coord.y >= 0 && coord.x < (self.map.len() as i32 - 1) && coord.y < (self.map.len() as i32 - 1)
  }
}

impl Index<&Coord> for Maze {
  type Output = char;

  fn index(&self, coord: &Coord) -> &Self::Output {
      if coord.x < 0 || coord.y < 0 || coord.x >= self.map.len() as i32 || coord.y >= self.map[coord.y as usize].len() as i32 {
        &'.'
      } else {
        &self.map[coord.y as usize][coord.x as usize]
      }
  }
}

impl IndexMut<&Coord> for Maze {
  fn index_mut(&mut self, coord: &Coord) -> &mut Self::Output {
    &mut self.map[coord.y as usize][coord.x as usize]
  }
}

fn load_maze(input: &str) -> Maze {
  let mut maze = Maze{
    start: Coord{x:-1,y:-1},
    map: Vec::new(),
  };

  let lines = read_to_string(input).unwrap();
  for (i,line) in lines.lines().enumerate() {  
    let mut vline = Vec::new();
    for (j,c) in line.chars().enumerate() {
      vline.push(c);
      if c == 'S' {
        maze.start = Coord{x:j as i32,y:i as i32};
      }
    }
    maze.map.push(vline);
  }
  maze
}

fn walk_maze(maze: &Maze) -> Vec<Coord> {
  let mut path = vec![maze.start.clone()];

  let mut now = maze.start.clone();
  let mut dir = Direction::UP;
  loop {
    //println!("now at: {:?} ({}) looking {:?}", now, maze[&now], dir);
    now = match dir {
      Direction::UP => now.up(),
      Direction::DOWN => now.down(),
      Direction::LEFT => now.left(),
      Direction::RIGHT => now.right(),
    };
    
    if now == maze.start {
      break;
    }
    
    dir = dir.rotate(maze[&now]);
    path.push(now.clone());
  }

  println!("size: {:?} mid: {}", path.len(), path.len()/2);

  path
}

/*  ----------- part 2 ------------- */

fn print_maze(maze: &Maze) {
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      print!("{}",maze.map[i][j]);
    }
    println!("")
  }
}

fn upscale_maze(maze: &Maze) -> Maze {
  let mut upscaled_maze = Maze{
    start: Coord{x:2*maze.start.x,y:2*maze.start.y},
    map: vec![vec!['.'; maze.map.len()*2];maze.map.len()*2],
  };

  // upscale the maze
  for i in 0..maze.map.len() {
    for j in 0..maze.map[i].len() {
      upscaled_maze.map[2*i][2*j] = maze.map[i][j];
      upscaled_maze.map[2*i][2*j+1] = '.';
      upscaled_maze.map[2*i+1][2*j] = '.';
      upscaled_maze.map[2*i+1][2*j+1] = '.';
    }
  }

  // fill the gap horizontally
  for i in 0..maze.map.len() {
    maze.map[i].windows(2).enumerate().for_each(|(j,w)| {
      if (w[0] == '-' || w[0] == 'F' || w[0] == 'L' || w[0] == 'S') && (w[1] == 'S' || w[1] == '-' || w[1] == '7' || w[1] == 'J') {
        upscaled_maze.map[2*i][2*j+1] = '-';
      }
    })
  }  

  // fill the gap vertically (the maze is squared)
  for i in 0..maze.map.len()-1 {
    for j in 0..maze.map.len() {
      let u = maze.map[i][j];
      let d = maze.map[i+1][j];
      if (u == '|' || u == 'F' || u == '7' || u == 'S') && (d == 'S' || d == '|' || d == 'J' || d == 'L') {
        upscaled_maze.map[2*i+1][2*j] = '|';
     }
    }
  }
  
  upscaled_maze
}  


fn downscale_maze(maze: &Maze) -> Maze {
  let mut downscaled = Maze{
    start: Coord{x:maze.start.x/2,y:maze.start.y/2},
    map: vec![vec!['.'; maze.map.len()/2];maze.map.len()/2],
  };

  for i in 0..maze.map.len()/2 {
    for j in 0..maze.map[i].len()/2 {
      downscaled.map[i][j] = maze.map[2*i][2*j]
    }
  }

  downscaled
}

fn color_maze(maze: &Maze, path: &Vec<Coord>) -> Maze {
  let mut colored = Maze{
    start: Coord{x:maze.start.x,y:maze.start.y},
    map: vec![vec!['.'; maze.map.len()];maze.map.len()],
  };

  // color the path
  for p in path {
    colored[p] = 'X';
  }

  // color all the borders
  let mut to_color: Vec<Coord> = (0..colored.map.len()-1).into_iter().flat_map(|i| {
    vec![
      Coord{x: 0 as i32, y:i as i32},
      Coord{x: i as i32, y:0},
      Coord{x: colored.map.len() as i32 - 1, y:i as i32},
      Coord{x: i as i32, y:colored.map.len() as i32 - 1}
    ]
  })
  .filter(|c| { colored[c] == '.' })
  .collect();
  
  loop {
    if to_color.len() == 0 {
      break;
    }

    let c = to_color.pop().unwrap();
    colored[&c] = 'C';

    vec![c.up(), c.down(), c.left(), c.right()]
    .into_iter()
    .filter(|c| colored.exists(c))
    .filter(|c| { colored[c] == '.' })
    .for_each(|c| to_color.push(c));
  }

  colored
}

/*  ----------- part 2  - second and simpler approach ------------- */

fn color_right(maze: &Maze) {
  // first we walk the maze as before
  let mut path = vec![maze.start.clone()];
  let mut now = maze.start.clone();
  let mut dir = Direction::UP;
  let mut to_color= Vec::new();
  loop {
    now = match dir {
      Direction::UP => now.up(),
      Direction::DOWN => now.down(),
      Direction::LEFT => now.left(),
      Direction::RIGHT => now.right(),
    };
    
    if now == maze.start {
      break;
    }
    
    // schedule to color right of border (before rotation)
    match dir {
      Direction::UP => to_color.push(now.right()),
      Direction::DOWN => to_color.push(now.left()),
      Direction::LEFT => to_color.push(now.up()),
      Direction::RIGHT => to_color.push(now.down()),
    };

    dir = dir.rotate(maze[&now]);
    path.push(now.clone());

    // schedule to color right of border (after rotation)
    match dir {
      Direction::UP => to_color.push(now.right()),
      Direction::DOWN => to_color.push(now.left()),
      Direction::LEFT => to_color.push(now.up()),
      Direction::RIGHT => to_color.push(now.down()),
    };
  }
   
  // then we color the maze
  let mut colored = Maze{
    start: Coord{x:maze.start.x,y:maze.start.y},
    map: vec![vec!['.'; maze.map.len()];maze.map.len()],
  };

  for p in path.iter() {
    colored[p] = 'X';
  }

  let mut to_color: Vec<Coord> = to_color.into_iter()
  .filter(|c| colored.exists(c))
  .filter(|c| { colored[c] == '.' })
  .collect();

  loop {
    if to_color.len() == 0 {
      break;
    }

    let c = to_color.pop().unwrap();
    colored[&c] = 'C';
    
    vec![c.up(), c.down(), c.left(), c.right()]
    .into_iter()
    .filter(|c| colored.exists(c))
    .filter(|c| { colored[c] == '.' })
    .for_each(|c| to_color.push(c));
  }

  // count the colored cells
  let mut inside = 0;
  let mut outside = 0;
  for i in 0..colored.map.len() {
    for j in 0..colored.map[i].len() {
      match colored.map[i][j] {
        '.' => outside += 1,
        'C' => inside += 1,
        _ => {}
      }
    }
  }
  println!("inside: {} outside: {}", inside, outside);
}


/* part 1
fn main() {
  let input = "day10/assets/input";
  let maze = load_maze(input);
  let path = walk_maze(&maze);
}
*/

/* part 2 
fn main() {
  let input = "day10/assets/input";
  let maze = load_maze(input);
  let path = walk_maze(&maze);

  let upscaled = upscale_maze(&maze);
  let path = walk_maze(&upscaled);
  let colored = color_maze(&upscaled, &path);
  let downscaled = downscale_maze(&colored);

  let sum: i32 = downscaled.map.iter().flat_map(|l| l).filter(|c| **c == '.').map(|_| 1).sum();
  println!("sum is {}", sum);
}
*/

/* part 2 - second and simpler take*/
fn main() {
  let input = "day10/assets/input";
  let maze = load_maze(input);
  color_right(&maze);
}