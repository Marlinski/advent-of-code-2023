/*
--- Day 13: Point of Incidence ---
With your help, the hot springs team locates an appropriate spring which launches you neatly and precisely up to the edge of Lava Island.

There's just one problem: you don't see any lava.

You do see a lot of ash and igneous rock; there are even what look like gray mountains scattered around. After a while, you make your way to a nearby cluster of mountains only to discover that the valley between them is completely full of large mirrors. Most of the mirrors seem to be aligned in a consistent way; perhaps you should head in that direction?

As you move through the valley of mirrors, you find that several of them have fallen from the large metal frames keeping them in place. The mirrors are extremely flat and shiny, and many of the fallen mirrors have lodged into the ash at strange angles. Because the terrain is all one color, it's hard to tell where it's safe to walk or where you're about to run into a mirror.

You note down the patterns of ash (.) and rocks (#) that you see as you walk (your puzzle input); perhaps by carefully analyzing these patterns, you can figure out where the mirrors are!

For example:

#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
To find the reflection in each pattern, you need to find a perfect reflection across either a horizontal line between two rows or across a vertical line between two columns.

In the first pattern, the reflection is across a vertical line between two columns; arrows on each of the two columns point at the line between the columns:

123456789
    ><
#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.
    ><
123456789
In this pattern, the line of reflection is the vertical line between columns 5 and 6. Because the vertical line is not perfectly in the middle of the pattern, part of the pattern (column 1) has nowhere to reflect onto and can be ignored; every other column has a reflected column within the pattern and must match exactly: column 2 matches column 9, column 3 matches 8, 4 matches 7, and 5 matches 6.

The second pattern reflects across a horizontal line instead:

1 #...##..# 1
2 #....#..# 2
3 ..##..### 3
4v#####.##.v4
5^#####.##.^5
6 ..##..### 6
7 #....#..# 7
This pattern reflects across the horizontal line between rows 4 and 5. Row 1 would reflect with a hypothetical row 8, but since that's not in the pattern, row 1 doesn't need to match anything. The remaining rows match: row 2 matches row 7, row 3 matches row 6, and row 4 matches row 5.

To summarize your pattern notes, add up the number of columns to the left of each vertical line of reflection; to that, also add 100 multiplied by the number of rows above each horizontal line of reflection. In the above example, the first pattern's vertical line has 5 columns to its left and the second pattern's horizontal line has 4 rows above it, a total of 405.

Find the line of reflection in each of the patterns in your notes. What number do you get after summarizing all of your notes?
 */

use std::fs::read_to_string;

type Pattern = Vec<Vec<char>>;
type Valley = Vec<Pattern>;

fn load_valley(input: &str) -> Valley {
    let lines = read_to_string(input).unwrap();
    lines.split("\n\n").into_iter().map(|pattern| {
        pattern.split("\n").into_iter()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Pattern>()
    }).collect::<Valley>()
}

fn rotate(p: &Pattern) -> Pattern {
    let mut r: Pattern = Vec::new();
    for i in 0..p[0].len() {
        r.push(p.iter().fold(Vec::new(), |mut acc,v| {acc.push(v[i]); acc}));
    }
    r
}

fn find_horizontal_mirror(p: &Pattern) -> Vec<usize> {
    let mut ret = Vec::new();
    let h = p.len();
    for i in 1..=h/2 {
        // upper half
        if (0..i).all(|j| p[i-j-1] == p[i+j]) {
            ret.push(i);
        }
        // bottom half
        if (0..i).all(|j| p[h-1-i-j] == p[h-i+j]) {
            ret.push(h-i);
        }
    }
    ret
}

/*

fn main() {
    let valley = load_valley("day13/assets/input");
    
    let sum_h: usize = valley.iter()
    .map(|p| find_horizontal_mirror(p))
    .flatten()
    .sum();

    let sum_v: usize = valley.iter()
    .map(|p| rotate(p))
    .map(|p| find_horizontal_mirror(&p))
    .flatten()
    .sum();

    println!("sum {}",100*sum_h+sum_v);
}

*/

/* ----------- part two ---------- */

use std::collections::HashSet;
use itertools::Itertools;

#[derive(Debug)]
enum Smudge {
    Horizontal(usize),
    Vertical(usize)
}

fn find_horizontal_smudge(mut p: Pattern) -> Vec<usize> {
    let h = p.len();
    let w = p[0].len();
    
    let without_smudge = find_horizontal_mirror(&p);
    (0..h).cartesian_product(0..w).flat_map(|(i,j)| {
        let c = p[i][j];
        p[i][j] = if c == '.' { '#' } else { '.' };
        let h = find_horizontal_mirror(&p);
        p[i][j] = c;
        h
    })
    .collect::<HashSet<_>>().into_iter()
    .filter(|h| !without_smudge.contains(h))
    .collect()
}

fn find_smudge(p: &Pattern) -> Smudge {
    let smudge = find_horizontal_smudge(p.clone());
    if smudge.len() == 1 {
        return Smudge::Horizontal(smudge[0]);
    }

    let smudge = find_horizontal_smudge(rotate(&p));
    if smudge.len() == 1 {
        return Smudge::Vertical(smudge[0]);
    }

    panic!("no smudges found!");
}

fn main() {
    let valley = load_valley("day13/assets/input");
    let sum: usize = valley.iter()
    .map(|p| find_smudge(p))
    .inspect(|s| println!("{:?}",s))
    .fold(0, |acc,s| {
        match s {
            Smudge::Horizontal(h) => acc + 100*h,
            Smudge::Vertical(v) => acc + v,
        }
    });

    println!("sum {}",sum);
}