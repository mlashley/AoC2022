use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Ok(lines) = read_lines("./input.txt") {

        let mut total_score = 0;
        for pair in lines.flatten() {

            let mut seen ='Q';
            let mut to_play = 'R';
            let mut  p = pair.chars();
            let mut pn = p.next();
            match pn {
                Some(x) => seen = x,
                None => println!("Error parsing {}",pair)
            }
            p.next();
            let pn = p.next();
            match pn {
                Some(x) => to_play = x,
                None => println!("Error parsing {}",pair)
            }

            let round_score = score_part1(seen,to_play);
            total_score += round_score;
            println!("{} => {} round score {} total score {}",seen,to_play,round_score, total_score)

        }
    } else {
        println!("Error openening input...");
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

// A = Rock, B = Paper, C = Scissors
// X = Rock, Y = Paper, Z = Scissors

fn score_part1(them: char, us: char) -> i32
{
    let mut score: i32 = 0;
    match us {
        'X' => score += 1,
        'Y' => score += 2,
        'Z' => score += 3,
        _ => println!("Warning - unexpected {} looking for X,Y,Z",us)
    }
    match them {
        'A' => match us { // Rock
            'X' => score += 3, // draw 
            'Y' => score += 6, // win
            'Z' => score += 0, // lose
            _ => println!("Warning - unexpected {} looking for X,Y,Z",us)
        },

        'B' => match us { // Paper
            'X' => score += 0, // lose 
            'Y' => score += 3, // draw 
            'Z' => score += 6, // win
            _ => println!("Warning - unexpected {} looking for X,Y,Z",us)
        },

        'C' => match us { // Scissors
            'X' => score += 6, // win
            'Y' => score += 0, // lose 
            'Z' => score += 3, // draw 
            _ => println!("Warning - unexpected {} looking for X,Y,Z",us)
        },


        _ => println!("Warning - unexpected {} looking for A,B,C",them)
    }

    score

}
