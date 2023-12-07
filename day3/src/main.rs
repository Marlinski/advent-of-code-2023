/*
--- Day 3: Gear Ratios ---
You and the Elf eventually reach a gondola lift station; he says the gondola lift will take you up to the water source, but this is as far as he can bring you. You go inside.

It doesn't take long to find the gondolas, but there seems to be a problem: they're not moving.

"Aaah!"

You turn around to see a slightly-greasy Elf with a wrench and a look of surprise. "Sorry, I wasn't expecting anyone! The gondola lift isn't working right now; it'll still be a while before I can fix it." You offer to help.

The engineer explains that an engine part seems to be missing from the engine, but nobody can figure out which one. If you can add up all the part numbers in the engine schematic, it should be easy to work out which part is missing.

The engine schematic (your puzzle input) consists of a visual representation of the engine. There are lots of numbers and symbols you don't really understand, but apparently any number adjacent to a symbol, even diagonally, is a "part number" and should be included in your sum. (Periods (.) do not count as a symbol.)

Here is an example engine schematic:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, two numbers are not part numbers because they are not adjacent to a symbol: 114 (top right) and 58 (middle right). Every other number is adjacent to a symbol and so is a part number; their sum is 4361.

Of course, the actual engine schematic is much larger. What is the sum of all of the part numbers in the engine schematic?

--- Part Two ---
The engineer finds the missing part and installs it in the engine! As the engine springs to life, you jump in the closest gondola, finally ready to ascend to the water source.

You don't seem to be going very fast, though. Maybe something is still wrong? Fortunately, the gondola has a phone labeled "help", so you pick it up and the engineer answers.

Before you can explain the situation, she suggests that you look out the window. There stands the engineer, holding a phone in one hand and waving with the other. You're going so slowly that you haven't even left the station. You exit the gondola.

The missing part wasn't the only issue - one of the gears in the engine is wrong. A gear is any * symbol that is adjacent to exactly two part numbers. Its gear ratio is the result of multiplying those two numbers together.

This time, you need to find the gear ratio of every gear and add them all up so that the engineer can figure out which gear needs to be replaced.

Consider the same engine schematic again:

467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
In this schematic, there are two gears. The first is in the top left; it has part numbers 467 and 35, so its gear ratio is 16345. The second gear is in the lower right; its gear ratio is 451490. (The * adjacent to 617 is not a gear because it is only adjacent to one part number.) Adding up all of the gear ratios produces 467835.

What is the sum of all of the gear ratios in your engine schematic?
*/

use std::collections::HashMap;
use std::fs::read_to_string;
use std::cmp;

type Engine = [[char; 140]; 140];
type Coord = (i32,i32);

fn load_engine(input: &str) ->  Engine {
   let mut ret:  Engine = [['\0'; 140]; 140];

   let lines = read_to_string(input).unwrap();
   let mut i = 0;
   for line in lines.lines() {
      let mut j = 0;
      for c in line.chars() {
         ret[i][j] = c;
         j += 1;
      }
      i += 1;
   }

   ret   
}

fn get_int_at(engine: &Engine, coord: Coord, size: usize) -> u32 {
   let (i,j) = coord;
   let line: &[char] = &engine[i as usize];
   let slice: &[char] = &line[j as usize..(j as usize+size)];
   let string: String = slice.iter().collect();
   string.parse().unwrap()
}

fn is_symbol_adjacent(engine: &Engine, coord: Coord, size: usize) -> bool {
   let (i,j) = coord;
   let toply = cmp::max(i-1,0);
   let topry = cmp::min(i+1,139);
   let toplx = cmp::max(j-1,0);
   let toprx = cmp::min(j+size as i32+1,139);
   //println!("searching k in {}..{} and l in {}..{}", toply, topry, toplx, toprx);
   for k in toply..=topry {
      for l in toplx..toprx {
         let s = engine[k as usize][l as usize];      
         //println!("searching {} {} -> {}", k,l,s);
         if !s.is_digit(10) && s != '.' {
            return true
         }
      }
   }
   false
}

fn process_engine(engine: &Engine) {
   let mut coord: Coord = (-1,-1);
   let mut size = 0;
   let mut sum = 0;
   for i in 0..140 {
      for j in 0..140 {
          let element = engine[i][j];
          if element.is_digit(10) {
            if coord == (-1,-1) {
               coord = (i as i32,j as i32);
            }
            size += 1
          } else {
            if coord != (-1,-1) && is_symbol_adjacent(engine, coord, size)  {
               let value = get_int_at(engine, coord, size);
               //println!("found value {}", value);
               sum += value;
            } 
            coord = (-1,-1);
            size = 0;
          }
      }
  }
  println!("sum is {}", sum);
}


/* -------- part two ----------- */

fn adjacent_stars(engine: &Engine, coord: Coord, size: usize) -> Vec<Coord> {
   let (i,j) = coord;
   let toply = cmp::max(i-1,0);
   let topry = cmp::min(i+1,139);
   let toplx = cmp::max(j-1,0);
   let toprx = cmp::min(j+size as i32+1,139);
   let mut ret: Vec<Coord> = Vec::new();
   for k in toply..=topry {
      for l in toplx..toprx {
         let s = engine[k as usize][l as usize];      
         if s == '*' {
            ret.push((k,l))
         }
      }
   }
   ret
}

fn process_engine_part_two(engine: &Engine) {
   let mut coord: (i32,i32) = (-1,-1);
   let mut size = 0;
   let mut star_map: HashMap<Coord, Vec<u32>> = HashMap::new();
   for i in 0..140 {
      for j in 0..140 {
          let element = engine[i][j];
          if element.is_digit(10) {
            if coord == (-1,-1) {
               coord = (i as i32,j as i32);
            }
            size += 1
          } else {
            if coord != (-1,-1) {
               let value = get_int_at(engine, coord, size);

               adjacent_stars(engine, coord, size)
               .into_iter()
               .for_each(|star| {
                  star_map.entry(star).or_insert(Vec::new()).push(value);
               }) 
            } 
            coord = (-1,-1);
            size = 0;
          }
      }
  }

  let sum = star_map.into_iter()
  .filter(|(_, v)| v.len() == 2)
  .map(|(_k, ints)| {
      ints.into_iter().fold(1, |acc,v| acc*v)
  })
  .fold(0, |acc, v| acc + v);
  
  println!("sum is {}", sum);
}

fn main() {
   let input = "day3/assets/input";
   let engine = load_engine(input);
   process_engine(&engine);
   process_engine_part_two(&engine);
}