pub mod bottle;
pub mod color;

use crate::bottle::Bottle;
use std::collections::{HashMap, HashSet};

static mut SOLVED: usize = 0;

type Game = Vec<Bottle>;

/// This traits represents something that can be discerned between a is solved and is not solved state
trait Solvable {
    fn is_solved(&self) -> bool;
}

impl Solvable for Game {
    fn is_solved(&self) -> bool {
        self.iter().all(|b| b.is_solved() || b.is_empty())
    }
}

/// BestGame represents the best posible solution to a game
#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct BestGame {
    /// The final solution of the game
    pub solve: Game,
    /// How many steps are required to reach the goal
    pub steps: usize,
    /// The path taken to reach the final solution
    pub path: Vec<Game>,
}

impl BestGame {
    /// Returns a default BestGame
    pub fn new() -> BestGame {
        BestGame {
            solve: Vec::new(),
            steps: 0,
            path: Vec::new(),
        }
    }
}

/// Tries to find the best possible solution to a given name
pub fn solve(game: &Game) -> BestGame {
    let mut best = BestGame::new();

    find(
        game.clone(),
        &mut HashSet::new(),
        &mut Vec::new(),
        &mut best,
    );

    best
}

fn possible_moves(game: &Game) -> Vec<Game> {
    let mut queue = Vec::new();

    game.iter().enumerate().for_each(|(i, pourer)| {
        game.iter().enumerate().for_each(|(j, pouree)| {
            if i == j {
                return;
            }

            let (pourer, pouree) = Bottle::pour(pourer.clone(), pouree.clone());

            let mut new_game = game.clone();
            new_game[i] = pourer;
            new_game[j] = pouree;

            if new_game != *game {
                queue.push(new_game);
            }
        });
    });

    queue
}

pub fn find_memo(game: &Game, memo: &mut HashMap<Game, Option<Vec<Game>>>) -> Option<Vec<Game>> {
    let mut game = game.clone();
    game.sort();
    let game = &game;

    if memo.contains_key(game) {
        return memo[game].clone();
    }

    if game.is_solved() {
        return Some(Vec::new());
    }

    memo.insert(game.clone(), None);

    let queue = possible_moves(&game);

    let mut best = None;

    for new_game in queue {
        let result = find_memo(&new_game, memo);

        if result.is_some() {
            if best.is_none() {
                best = result;
                best.as_mut().unwrap().push(game.clone());
            } else {
                if result.as_ref().unwrap().len() < best.as_ref().unwrap().len() {
                    best = result;
                    best.as_mut().unwrap().push(game.clone());
                }
            }
        }
    }

    memo.insert(game.clone(), best.clone());
    best
}

fn find(game: Game, seen: &mut HashSet<Game>, path: &mut Vec<Game>, best: &mut BestGame) {
    let mut queue: Vec<Game> = Vec::new();

    seen.insert(game.clone());

    game.iter().enumerate().for_each(|(i, pourer)| {
        game.iter().enumerate().for_each(|(j, pouree)| {
            if i == j {
                return;
            }

            let (pourer, pouree) = Bottle::pour(pourer.clone(), pouree.clone());

            let mut new_game = game.clone();
            new_game[i] = pourer;
            new_game[j] = pouree;

            // As long as the game is changed from the original, we should keep it as a possible solution
            // If we consider the number of steps or if we've seen the game now, we would include solutions
            // that we shouldn't consider. This is because a previous game in the queue might affect these
            // variables.
            if new_game != game {
                queue.push(new_game);
            }
        });
    });

    queue.iter().for_each(|game| {
        if seen.contains(game) {
            return;
        }

        if path.len() + 1 >= best.steps && best.steps != 0 {
            return;
        }

        path.push(game.clone());

        // All bottles are solved (all filled with just one color)
        if game.iter().all(|b| b.is_solved()) {
            // All bottles are either empty or filled completely
            if game.iter().all(|b| (b.is_empty() || b.is_full())) {
                println!();

                //println!("{:?}", game);
                unsafe {
                    SOLVED += 1;
                    println!("Solved: {:?}", SOLVED);
                }
                println!("Steps: {:?}", path.len());
                println!("Seen games: {:?}", seen.len());

                // Update best
                best.solve = game.clone();
                best.steps = path.len();
                best.path = path.clone();

                path.pop();

                return;
            }
        }

        find(game.clone(), seen, path, best);

        path.pop();
    });
}

#[cfg(test)]
mod tests {
    use crate::color::Color::*;

    use super::*;

    #[test]
    fn find_memoized_finds_solution() {
        let game = Vec::from([
            Bottle {
                contents: Vec::from([Orange, Blue, Blue, Blue]),
                size: 4,
            },
            Bottle {
                contents: Vec::from([Green, Blue, Green, Green]),
                size: 4,
            },
            Bottle {
                contents: Vec::from([Orange, Orange, Orange, Green]),
                size: 4,
            },
            Bottle::new(4),
            Bottle::new(4),
            Bottle::new(4),
        ]);

        find_memo(&game, &mut HashMap::new()).unwrap();
    }
}
