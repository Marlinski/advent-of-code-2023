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
*/


use std::fs::read_to_string;
use std::cmp;

type Engine = [[char; 140]; 140];

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

fn get_int_at(engine: &Engine, coord: (i32, i32), size: usize) -> u32 {
   let (i,j) = coord;
   let line: &[char] = &engine[i as usize];
   let slice: &[char] = &line[j as usize..(j as usize+size)];
   let string: String = slice.iter().collect();
   string.parse().unwrap()
}

fn is_symbol_adjacent(engine: &Engine, coord: (i32, i32), size: usize) -> bool {
   
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
   let mut coord: (i32,i32) = (-1,-1);
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

fn main() {
   let input = "day3/assets/input";
   let engine = load_engine(input);
   process_engine(&engine);
}