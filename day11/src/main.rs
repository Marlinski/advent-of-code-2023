/*
--- Day 11: Cosmic Expansion ---
You continue following signs for "Hot Springs" and eventually come across an observatory. The Elf within turns out to be a researcher studying cosmic expansion using the giant telescope here.

He doesn't know anything about the missing machine parts; he's only visiting for this research project. However, he confirms that the hot springs are the next-closest area likely to have people; he'll even take you straight there once he's done with today's observation analysis.

Maybe you can help him with the analysis to speed things up?

The researcher has collected a bunch of data and compiled the data into a single giant image (your puzzle input). The image includes empty space (.) and galaxies (#). For example:

...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
The researcher is trying to figure out the sum of the lengths of the shortest path between every pair of galaxies. However, there's a catch: the universe expanded in the time it took the light from those galaxies to reach the observatory.

Due to something involving gravitational effects, only some space expands. In fact, the result is that any rows or columns that contain no galaxies should all actually be twice as big.

In the above example, three columns and two rows contain no galaxies:

   v  v  v
 ...#......
 .......#..
 #.........
>..........<
 ......#...
 .#........
 .........#
>..........<
 .......#..
 #...#.....
   ^  ^  ^
These rows and columns need to be twice as big; the result of cosmic expansion therefore looks like this:

....#........
.........#...
#............
.............
.............
........#....
.#...........
............#
.............
.............
.........#...
#....#.......
Equipped with this expanded universe, the shortest path between every pair of galaxies can be found. It can help to assign every galaxy a unique number:

....1........
.........2...
3............
.............
.............
........4....
.5...........
............6
.............
.............
.........7...
8....9.......
In these 9 galaxies, there are 36 pairs. Only count each pair once; order within the pair doesn't matter. For each pair, find any shortest path between the two galaxies using only steps that move up, down, left, or right exactly one . or # at a time. (The shortest path between two galaxies is allowed to pass through another galaxy.)

For example, here is one of the shortest paths between galaxies 5 and 9:

....1........
.........2...
3............
.............
.............
........4....
.5...........
.##.........6
..##.........
...##........
....##...7...
8....9.......
This path has length 9 because it takes a minimum of nine steps to get from galaxy 5 to galaxy 9 (the eight locations marked # plus the step onto galaxy 9 itself). Here are some other example shortest path lengths:

Between galaxy 1 and galaxy 7: 15
Between galaxy 3 and galaxy 6: 17
Between galaxy 8 and galaxy 9: 5
In this example, after expanding the universe, the sum of the shortest path between all 36 pairs of galaxies is 374.

Expand the universe, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

--- Part Two ---
The galaxies are much older (and thus much farther apart) than the researcher initially estimated.

Now, instead of the expansion you did before, make each empty row or column one million times larger. That is, each empty row should be replaced with 1000000 empty rows, and each empty column should be replaced with 1000000 empty columns.

(In the example above, if each empty row or column were merely 10 times larger, the sum of the shortest paths between every pair of galaxies would be 1030. If each empty row or column were merely 100 times larger, the sum of the shortest paths between every pair of galaxies would be 8410. However, your universe will need to expand far beyond these values.)

Starting with the same initial image, expand the universe according to these new rules, then find the length of the shortest path between every pair of galaxies. What is the sum of these lengths?

 */

use std::{fs::read_to_string, vec};

type Universe = Vec<Vec<char>>;

fn load_universe(input: &str) -> Universe  {
  let mut universe = Vec::new();

  let lines = read_to_string(input).unwrap();
  for line in lines.lines() {  
    let mut vline = Vec::new();
    for c in line.chars() {
      vline.push(c);
    }
    universe.push(vline);
  }

  universe
}

fn expand_universe(u: Universe) -> Universe {
  let mut expanded = Vec::new();

  // expand horizontally
  for line in u.iter() {
    expanded.push(line.clone());
    if line.iter().all(|c| *c == '.') {
      expanded.push(line.clone());
    }
  }

  // expand vertically
  let mut added = 0;
  for j in 0..u.len() {
    // if vertical of i is all '.'
    if (0..u.len()).into_iter()
    .map(|i| u[i][j])
    .all(|c| c == '.') {
      (0..expanded.len()).into_iter()
      .for_each(|i| {
        expanded[i].insert(j+added, '.');
      });
      added += 1;
    }
  }

  expanded
}


fn print_universe(u: &Universe) {
  for i in u.iter() {  
    for j in i.iter() {
      print!("{}", j);
    }
    println!("");
  }
  println!("");
}

fn puzzle_one(u: &Universe) {
  let mut galaxies = Vec::new();
  for (i,line) in u.iter().enumerate() {
    for (j,c) in line.iter().enumerate() {
      if c == &'#' {
        galaxies.push((i,j));
      }
    }
  }

  let mut sum = 0;
  let mut pairs = 0;
  for i in 0..galaxies.len() {
    for j in i..galaxies.len() {
      if galaxies[i] == galaxies[j] {
        continue;
      }
      let (i1,j1) = galaxies[i];
      let (i2,j2) = galaxies[j];
      let d = usize::max(i1,i2) - usize::min(i1,i2) + usize::max(j1,j2) - usize::min(j1,j2);
      //println!("pair ({},{}) {:?} {:?} -> {:?} parsec", i,j,galaxies[i], galaxies[j], d);
      sum += d;
      pairs += 1;
    }
  }

  println!("{} pairs, sum of distance is {}", pairs, sum);
}

/* ------- part two --------- */



fn expand_universe_2(u: Universe) -> Universe {
  let mut expanded = Vec::new();

  // expand horizontally
  for line in u.iter() {
    if line.iter().all(|c| *c == '.') {
      expanded.push(vec!['X';line.len()]);
    } else {
      expanded.push(line.clone());
    }
  }

  // expand vertically
  for j in 0..u.len() {
    // if vertical of i is all '.'
    if (0..u.len()).into_iter()
    .map(|i| u[i][j])
    .all(|c| c == '.') {
      (0..expanded.len()).into_iter()
      .for_each(|i| {
        expanded[i][j] = 'X';
      });
    }
  }

  expanded
}


fn puzzle_two(u: &Universe, factor: usize) {
  let mut galaxies = Vec::new();
  for (i,line) in u.iter().enumerate() {
    for (j,c) in line.iter().enumerate() {
      if c == &'#' {
        galaxies.push((i,j));
      }
    }
  }

  let mut sum = 0;
  for i in 0..galaxies.len() {
    for j in i..galaxies.len() {
      if galaxies[i] == galaxies[j] {
        continue;
      }

      let (i1,j1) = galaxies[i];
      let (i2,j2) = galaxies[j];
      for wi in usize::min(i1,i2)..usize::max(i1,i2) {
        if u[wi][0] == 'X' {
          sum += factor
        } else {
          sum += 1
        }
      }

      for wj in usize::min(j1,j2)..usize::max(j1,j2) {
        if u[0][wj] == 'X' {
          sum += factor
        } else {
          sum += 1
        }
      }
    }
  }

  println!("sum of distance is {}", sum);
}


/* part one
fn main() {
  let universe = load_universe("day11/assets/input");
  let universe = expand_universe(universe);
  puzzle_one(&universe);  
}
*/

/* part two */
fn main() {
  let universe = load_universe("day11/assets/input");
  let universe = expand_universe_2(universe);
  puzzle_two(&universe, 1000000);  
}