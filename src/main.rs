extern crate rand;
use rand::{thread_rng, sample};
use std::collections::HashMap;
use std::io::{self, Read};

#[derive(Debug,Clone,PartialEq)]
pub enum Colour {
    Red,
    Orange,
    Yellow,
    Blue,
    Green,
    Brown,
    White,
    Black,
}

#[derive(Debug,Clone)]
pub struct Sequence {
    first: Colour,
    second: Colour,
    third: Colour,
    fourth: Colour,
}

impl Sequence {
    pub fn new(first: &Colour, second: &Colour, third: &Colour, fourth: &Colour) -> Sequence {
        Sequence {
            first: first.clone(),
            second: second.clone(),
            third: third.clone(),
            fourth: fourth.clone(),
        }
    }

    pub fn generate() -> Sequence {
        let mut rng = thread_rng();
        let sample = sample(&mut rng, vec!['r', 'o', 'y', 'b', 'g', 'B', 'w', 'x'], 4);
        Sequence::from(sample)
    }

    pub fn colours(&self) -> Vec<Colour> {
        vec![self.first.clone(), self.second.clone(), self.third.clone(), self.fourth.clone()]
    }
}

impl PartialEq for Sequence {
    fn eq(&self, other: &Sequence) -> bool {
        self.first == other.first && self.second == other.second && self.third == other.third &&
        self.fourth == other.fourth
    }
}

impl<'a> From<&'a str> for Sequence {
    fn from(from: &'a str) -> Sequence {
        Sequence::from(from.chars().collect::<Vec<char>>())
    }
}

impl From<Vec<char>> for Sequence {
    fn from(from: Vec<char>) -> Sequence {
        let chars: Vec<Colour> = from.iter()
                                     .map(|c| {
                                         match *c {
                                             'r' => Colour::Red,
                                             'o' => Colour::Orange,
                                             'y' => Colour::Yellow,
                                             'b' => Colour::Blue,
                                             'g' => Colour::Green,
                                             'B' => Colour::Brown,
                                             'w' => Colour::White,
                                             _ => Colour::Black,
                                         }
                                     })
                                     .collect();
        Sequence::new(&chars[0], &chars[1], &chars[2], &chars[3])
    }
}

#[derive(Debug,Clone)]
pub struct Game {
    code: Sequence,
    turn: u32,
    previous: HashMap<u32, Sequence>,
    won: bool,
}

impl Game {
    pub fn new() -> Game {
        Game {
            code: Sequence::generate(),
            turn: 0,
            previous: HashMap::new(),
            won: false,
        }
    }

    pub fn guess(&mut self, guess: &Sequence) -> Game {
        let mut game = self.clone();
        let guess = guess.clone();
        game.turn += 1;
        if self.code == guess {
            game.previous.insert(game.turn, guess);
            game.won = true;
        } else {
            game.previous.insert(game.turn, guess);
        }
        game
    }

    // returns the count of white (colour right but place wrong) and black (colour and place
    // correct) clues
    pub fn clues(&mut self) -> (usize, usize) {
        let colours = &self.code.colours();
        let guess = self.previous[&self.turn].colours();

        let blacks = colours.iter()
                            .zip(&guess)
                            .filter(|&(a, b)| a == b)
                            .count();

        let whites = colours.iter()
                            .filter(|&a| guess.iter().find(|&b| b == a).is_some())
                            .count();

        (whites, blacks)
    }
}

fn main() {
    println!("Welcome to MasterMind!");
    println!("");
    println!("To guess, enter a single character.\n r for Red, b for Blue, B for brown, y for \
              yellow, w for White, o for Orange, g for Green and x for Black");
    let mut game = Game::new();
    // println!("{:?}", game.code);

    while game.turn < 12 {
        println!("\nPlease enter your guess");
        let mut buffer = String::new();
        io::stdin().read_line(&mut buffer).unwrap();
        let guess = Sequence::from(buffer.as_ref());
        println!("{:?}", guess);
        game = game.guess(&guess);
        if game.won {
            break;
        } else {
            let (white, black) = game.clues();
            if white == 0 && black == 0 {
                println!("Your guess was completely wrong!");
            } else if black == 0 {
                println!("You got {} white clues", white);
            } else if black == white {
                println!("You got {} black clues", black);
            } else {
                assert!(white > black);
                let white = white - black;
                println!("You got {} white clues and {} black clues", white, black);
            }
        }
    }

    if game.won {
        println!("You won on turn {}", game.turn);
    } else {
        println!("You lost! The code was {:?}", game.code);
    }
}
