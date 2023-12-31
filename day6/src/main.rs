/*
--- Day 6: Wait For It ---
The ferry quickly brings you across Island Island. After asking around, you discover that there is indeed normally a large pile of sand somewhere near here, but you don't see anything besides lots of water and the small island where the ferry has docked.

As you try to figure out what to do next, you notice a poster on a wall near the ferry dock. "Boat races! Open to the public! Grand prize is an all-expenses-paid trip to Desert Island!" That must be where the sand comes from! Best of all, the boat races are starting in just a few minutes.

You manage to sign up as a competitor in the boat races just in time. The organizer explains that it's not really a traditional race - instead, you will get a fixed amount of time during which your boat has to travel as far as it can, and you win if your boat goes the farthest.

As part of signing up, you get a sheet of paper (your puzzle input) that lists the time allowed for each race and also the best distance ever recorded in that race. To guarantee you win the grand prize, you need to make sure you go farther in each race than the current record holder.

The organizer brings you over to the area where the boat races are held. The boats are much smaller than you expected - they're actually toy boats, each with a big button on top. Holding down the button charges the boat, and releasing the button allows the boat to move. Boats move faster if their button was held longer, but time spent holding the button counts against the total race time. You can only hold the button at the start of the race, and boats don't move until the button is released.

For example:

Time:      7  15   30
Distance:  9  40  200
This document describes three races:

The first race lasts 7 milliseconds. The record distance in this race is 9 millimeters.
The second race lasts 15 milliseconds. The record distance in this race is 40 millimeters.
The third race lasts 30 milliseconds. The record distance in this race is 200 millimeters.
Your toy boat has a starting speed of zero millimeters per millisecond. For each whole millisecond you spend at the beginning of the race holding down the button, the boat's speed increases by one millimeter per millisecond.

So, because the first race lasts 7 milliseconds, you only have a few options:

Don't hold the button at all (that is, hold it for 0 milliseconds) at the start of the race. The boat won't move; it will have traveled 0 millimeters by the end of the race.
Hold the button for 1 millisecond at the start of the race. Then, the boat will travel at a speed of 1 millimeter per millisecond for 6 milliseconds, reaching a total distance traveled of 6 millimeters.
Hold the button for 2 milliseconds, giving the boat a speed of 2 millimeters per millisecond. It will then get 5 milliseconds to move, reaching a total distance of 10 millimeters.
Hold the button for 3 milliseconds. After its remaining 4 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 4 milliseconds. After its remaining 3 milliseconds of travel time, the boat will have gone 12 millimeters.
Hold the button for 5 milliseconds, causing the boat to travel a total of 10 millimeters.
Hold the button for 6 milliseconds, causing the boat to travel a total of 6 millimeters.
Hold the button for 7 milliseconds. That's the entire duration of the race. You never let go of the button. The boat can't move until you let go of the button. Please make sure you let go of the button so the boat gets to move. 0 millimeters.
Since the current record for this race is 9 millimeters, there are actually 4 different ways you could win: you could hold the button for 2, 3, 4, or 5 milliseconds at the start of the race.

In the second race, you could hold the button for at least 4 milliseconds and at most 11 milliseconds and beat the record, a total of 8 different ways to win.

In the third race, you could hold the button for at least 11 milliseconds and no more than 19 milliseconds and still beat the record, a total of 9 ways you could win.

To see how much margin of error you have, determine the number of ways you can beat the record in each race; in this example, if you multiply these values together, you get 288 (4 * 8 * 9).

Determine the number of ways you could beat the record in each race. What do you get if you multiply these numbers together?

Puzzle Input:
Time:        61     67     75     71
Distance:   430   1036   1307   1150
*/

type Race = (i64,i64);
type Races = Vec<Race>;

fn load_test() -> Races {
   let mut ret = Vec::new();
   ret.push((7,9));
   ret.push((15,40));
   ret.push((30,200));
   ret
}


fn load_race() -> Races {
   let mut ret = Vec::new();
   ret.push((61,430));
   ret.push((67,1036));
   ret.push((75,1307));
   ret.push((71,1150));
   ret
}

/*
 * given x = time to hold the button down
 * and   y = total time
 * you get (y-x) * x as distance travelled
 * 
 * given z a record distance we want
 * (y-x) * x >= z+1
 *  y*x - x*x >= z+1
 * -1.x^2 + y.x - (z+1) > 0 
 * 
 * (-1,y,-z-1) are the coefficient of our quadratic polynomial
 * we know that the root of a quadratic polynomial are:
 *  D = b2 - 4ac
 *  x1 = (-b + √ D )/2a
 *  x2 = (-b - √ D )/2a
 * 
 *  so it becomes:
 *  (x-x1)(x-x2) > 0
 */

fn discriminant(a: i64,b: i64,c: i64) -> i64 {
   return b*b - 4*a*c;
}

fn roots(a: i64,b: i64,c: i64) -> (i64,i64) {
   let d =discriminant(a, b, c); 
   if d < 0 {
      panic!("no real roots");
   }
   return ((-1*b+(f64::floor(f64::sqrt(d as f64)) as i64))/2*a,(-1*b-(f64::floor(f64::sqrt(d as f64)) as i64))/2*a)
}

fn distance(x: i64, y:i64) -> i64 {
   (y-x)*x
}

fn puzzle_part_one(races: &Races) {
   let mul = races
   .into_iter()
   .map(|(y,z)| {
      let y = y.clone();
      let z = z.clone();

      let a = -1;
      let b = y;
      let c = -1*z-1;

      let (r1,r2) = roots(a.clone(),b.clone(),c.clone());
      let (d1,d2) = (distance(r1,b), distance(r2, b));
      println!("must beat: {} /// {} -> {}   {} -> {}", z, r1, d1, r2, d2);
      
      if d1 <= z {
         r2-r1
      } else {
         r2-r1+1
      }
   })
   .fold(1, |acc,v| acc*v);
   println!("mul is {}",mul);
}

/* ------------- part two ------------- */


fn load_test_part_two() -> Races {
   let mut ret = Vec::new();
   ret.push((71530i64,940200i64));
   ret
}


fn load_race_part_two() -> Races {
   let mut ret = Vec::new();
   ret.push((61677571i64,430103613071150i64));
   ret
}

fn puzzle_part_two(races: &Races) {
   let mul = races
   .into_iter()
   .map(|(y,z)| {
      let y = y.clone();
      let z = z.clone();

      let a = -1;
      let b = y;
      let c = -1*z-1;

      let (r1,r2) = roots(a.clone(),b.clone(),c.clone());
      let (d1,d2) = (distance(r1,b), distance(r2, b));
      println!("must beat: {} /// {} -> {}   {} -> {}", z, r1, d1, r2, d2);
      
      if d1 <= z {
         r2-r1
      } else {
         r2-r1+1
      }
   })
   .fold(0, |acc,v| acc+v);
   println!("sum is {}",mul);
}

fn main() {
   let races = load_race_part_two();
   puzzle_part_two(&races)
}