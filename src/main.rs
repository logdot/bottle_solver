use std::collections::HashSet;

#[derive(Eq, PartialEq, Debug, Clone, Hash)]
enum Color {
    Red,
    Green,
    DGreen,
    Blue,
    DBlue,
    Pink,
    Yellow,
    Grey,
    Brown,
    Orange,
    LOrange
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct Bottle {
    contents: Vec<Color>,
    size: usize,
}

impl Bottle {
    fn new(size: usize) -> Bottle {
        Bottle {
            contents: Vec::with_capacity(size),
            size: size,
        }
    }

    fn uni_bottle(size: usize, color: Color) -> Bottle {
        Bottle {
            contents: Vec::from(std::iter::repeat(color).take(size).collect::<Vec<Color>>()),
            size: size,
        }
    }

    fn rep_bottle(size: usize, color: Color, count: usize) -> Bottle {
        Bottle {
            contents: Vec::from(std::iter::repeat(color).take(count).collect::<Vec<Color>>()),
            size: size,
        }
    }

    fn solved(&self) -> bool {
        // If contents of bottle are all the same color, then it's solved
        let mut seen = None;

        for color in &self.contents {
            match seen {
                None => seen = Some(color.clone()),
                Some(ref seen) => {
                    if color != seen {
                        return false;
                    }
                }
            }
        }
        return true;
    }

    fn is_full(&self) -> bool {
        return self.contents.len() == self.size;
    }

    fn is_empty(&self) -> bool {
        return self.contents.len() == 0;
    }

    fn pour(pourer: Bottle, pouree: Bottle) -> (Bottle, Bottle) {
        let mut pourer = pourer.clone();
        let mut pouree = pouree.clone();

        if pouree.size == pouree.contents.len() {
            return (pourer, pouree);
        }

        // A full solved bottle isn't allowed to be poured
        if pourer.size == pourer.contents.len() && pourer.solved() {
            return (pourer, pouree);
        }

        while pouree.contents.len() < pouree.size && pourer.contents.len() > 0 {
            let pourer_head = match pourer.contents.last() {
                Some(head) => head,
                None => break,
            };

            let pouree_head = match pouree.contents.last() {
                Some(head) => head,
                None => {
                    pouree.contents.push(pourer.contents.pop().unwrap());
                    continue;
                }
            };

            if *pouree_head == *pourer_head {
                pouree.contents.push(pourer.contents.pop().unwrap());
            } else {
                break;
            }
        }

        return (pourer, pouree);
    }
}

#[derive(Eq, PartialEq, Debug, Hash, Clone)]
struct SolvedGame {
    solve: Game,
    steps: usize,
    path: Vec<Game>,
}

impl SolvedGame {
    fn new(solve: Game, steps: usize, path: Vec<Game>) -> SolvedGame {
        SolvedGame {
            solve: solve,
            steps: steps,
            path: path,
        }
    }
}

type Game = Vec<Bottle>;

fn main() {
    // Level 119 of sortpuz for android
    let game = Vec::from([
        Bottle {
            contents: Vec::from([Color::LOrange, Color::Pink, Color::Pink, Color::Brown]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::LOrange, Color::Blue, Color::Green, Color::Green]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::Pink, Color::Orange, Color::Blue, Color::DBlue]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::Brown, Color::DBlue, Color::Brown, Color::DGreen]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::Blue, Color::DBlue, Color::DGreen, Color::Orange]),
            size: 4,
        },

        Bottle {
            contents: Vec::from([Color::LOrange, Color::Green, Color::Green, Color::Orange]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::LOrange, Color::DGreen, Color::Blue, Color::DBlue]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Color::Orange, Color::Pink, Color::DGreen, Color::Brown]),
            size: 4,
        },
        Bottle::new(4),
        Bottle::new(4),
    ]);

    println!("Solving!!!");
    game.iter().for_each(|bottle| {
        println!("{:?}", bottle);
    });
    println!("");
    println!("");
    let mut default_solved_game = SolvedGame {
        solve: Vec::new(),
        steps: 0,
        path: Vec::new(),
    };
    solve(
        game,
        &mut HashSet::new(),
        &mut Vec::new(),
        &mut default_solved_game,
    );

    default_solved_game.path.iter().for_each(|game| {
        game.iter().for_each(|bottle| {
            println!("{:?}", bottle);
        });
        println!("");
    });

    println!("Done!!!!!!");
}

static mut solved: usize = 0;

fn solve(game: Game, seen: &mut HashSet<Game>, history: &mut Vec<Game>, best: &mut SolvedGame) {
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
            if new_game != game
            {
                queue.push(new_game);
            }
        });
    });

    queue.iter().for_each(|game| {
        if seen.contains(game) {
            return;
        }

        if history.len() >= best.steps && best.steps != 0 {
            return;
        }

        history.push(game.clone());

        //new_game.iter().for_each(|bottle| {
        //    println!("{:?}", bottle);
        //});
        //println!("");

        // All bottles are solved (all filled with just one color)
        if game.iter().fold(true, |a, b| a && b.solved()) {
            // All bottles are either empty or filled completely
            if game
                .iter()
                .fold(true, |a, b| a && (b.is_empty() || b.is_full()))
            {
                //    history.iter().for_each(|step| {
                //        step.iter().for_each(|bottle| {
                //            println!("{:?}", bottle);
                //        });
                //        println!("");
                //    });

                println!("");

                //println!("{:?}", game);
                unsafe {
                    solved += 1;
                    println!("Solved: {:?}", solved);
                }
                println!("Steps: {:?}", history.len());
                println!("Seen games: {:?}", seen.len());

                // Update best
                best.solve = Vec::from(game.clone());
                best.steps = history.len();
                best.path = history.clone();

                history.pop();

                return;
            }
        }

        solve(game.clone(), seen, history, best);

        history.pop();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pour_cant_overfill_full_bottle() {
        let pourer = Bottle::uni_bottle(6, Color::Red);
        let pouree = Bottle::uni_bottle(6, Color::Red);

        let (pourer, pouree) = Bottle::pour(pourer, pouree);

        assert_eq!(
            (pourer, pouree),
            (
                Bottle::uni_bottle(6, Color::Red),
                Bottle::uni_bottle(6, Color::Red)
            )
        );
    }

    #[test]
    fn pour_transfers_all_contents_from_bottle_to_other() {
        let pourer = Bottle::rep_bottle(6, Color::Red, 5);
        let pouree = Bottle::new(6);

        let (pourer, pouree) = Bottle::pour(pourer, pouree);

        assert_eq!(
            (pourer, pouree),
            (Bottle::new(6), Bottle::rep_bottle(6, Color::Red, 5))
        );
    }

    #[test]
    fn pour_into_empty_bottle() {
        let pourer = Bottle::rep_bottle(6, Color::Red, 3);
        let pouree = Bottle::new(6);

        let (pourer, pouree) = Bottle::pour(pourer, pouree);

        assert_eq!(
            (pourer, pouree),
            (Bottle::new(6), Bottle::rep_bottle(6, Color::Red, 3))
        );
    }

    #[test]
    fn pour_only_transfers_same_colored_contents() {
        let pourer = Bottle {
            contents: vec![
                Color::Red,
                Color::Red,
                Color::Red,
                Color::Blue,
                Color::Blue,
                Color::Blue,
            ],
            size: 6,
        };
        let pouree = Bottle::new(6);

        let (pourer, pouree) = Bottle::pour(pourer, pouree);

        assert_eq!(
            (pourer, pouree),
            (
                Bottle::rep_bottle(6, Color::Red, 3),
                Bottle::rep_bottle(6, Color::Blue, 3)
            )
        );
    }

    #[test]
    fn pour_wont_transfer_contents_of_full_bottle_of_the_same_color() {
        let pourer = Bottle::uni_bottle(6, Color::Red);
        let pouree = Bottle::new(6);

        let (pourer, pouree) = Bottle::pour(pourer, pouree);

        assert_eq!(
            (pourer, pouree),
            (Bottle::uni_bottle(6, Color::Red), Bottle::new(6))
        );
    }
}
