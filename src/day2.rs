use std::fs;
use std::process;
use std::usize;
use regex::Regex;

#[derive(Debug)]
enum Cube {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Pull {
    red: Vec<Cube>,
    blue: Vec<Cube>,
    green: Vec<Cube>,
}

impl Pull {
    fn new() -> Self {
        Self {
            red: Vec::new(),
            blue: Vec::new(),
            green: Vec::new(),
        }
    }

    fn add_by_color(&mut self, color: &str) {
         match color {
            "red" => self.red.push(Cube::Red),
            "blue" => self.blue.push(Cube::Blue),
            "green" => self.green.push(Cube::Green),
            _ => panic!("Did not recognize color: {color}"),
        }
    }
    
    fn check_if_possible(&self, bag: &Bag) -> bool {
        self.red.len() <= bag.reds.len() &&
           self.green.len() <= bag.greens.len() &&
           self.blue.len() <= bag.blues.len()
    }
}

#[derive(Debug)]
struct Bag {
    reds: Vec<Cube>,
    greens: Vec<Cube>,
    blues: Vec<Cube>,
}

impl Bag {
    fn new() -> Self {
        return Self {
            reds: Vec::new(),
            greens: Vec::new(),
            blues: Vec::new(),
        }
    }
    
    fn add_blue(&mut self, amount: usize) -> &mut Self {
        for _ in 0..amount {
            self.blues.push(Cube::Blue);
        }
        self
    }
    
    fn add_green(&mut self, amount: usize) -> &mut Self {
        for _ in 0..amount {
            self.greens.push(Cube::Green);
        }
        self
    }
    
    fn add_red(&mut self, amount: usize) -> &mut Self {
        for _ in 0..amount {
            self.reds.push(Cube::Red);
        }
        self
    }
    

}

#[derive(Debug)]
struct Game {
    id: usize,
    pulls: Vec<Pull>,
}

impl Game {
    fn new(id: usize) -> Self {
        return Self {
            id,
            pulls: Vec::new(),
        }
    }
    
    fn add_pull(&mut self, pull: Pull) {
        self.pulls.push(pull);
    }
    
    fn check_if_possible(&self, bag: &Bag) -> Option<usize> {
        for pull in self.pulls.iter() {
            if !pull.check_if_possible(&bag) {
                return None;
            }
        }
        Some(self.id)
    }
}

pub fn run() {
    let path = "./cubes.txt";
    let content = fs::read_to_string(path).unwrap_or_else(|err| {
        eprint!("Could not read from file: {}", err.to_string());
        process::exit(1);
    });
    
    let mut bag = Bag::new();
    bag.add_red(12);
    bag.add_green(13);
    bag.add_blue(14);
    
    let capture_number = Regex::new(r"(?<id>\d+)").unwrap();
    
    let mut games: Vec<Game> = Vec::new();
    for game in content.split("\n") {
        let (game_id_string, pulls) = game.split_once(":").unwrap();
        let result = capture_number.captures(game_id_string).unwrap_or_else(|| {
            eprintln!("Could not find a number");
            process::exit(1);
        });
        let game_id: &usize = &result["id"].parse::<usize>().unwrap();
        let mut game = Game::new(game_id.to_owned());
        for pulls in pulls.split(";") {
            let mut pull = Pull::new();
            for cube_pull in pulls.split(",") {
                let cube_pull = cube_pull.trim();
                let (amount, color) = cube_pull.split_once(" ").unwrap();
                let amount = amount.parse::<usize>().unwrap();
                for _ in 0..amount {
                    pull.add_by_color(color);
                }
            }
            game.add_pull(pull);
        }
        games.push(game);
    }
    
    
    let mut sum = 0;
    for game in games {
        if let Some(game_id) = game.check_if_possible(&bag) {
            println!("{game_id}");
            sum += game_id;
        }
    }

    println!("Sum: {sum}");

    // println!("{:?}", bag)
}