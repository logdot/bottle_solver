use std::cmp::Ordering;

use crate::color::Color;

/// Represents a single bottle in a game
#[derive(Eq, PartialEq, Debug, Hash, Clone)]
pub struct Bottle {
    /// The contents inside the bottle
    pub contents: Vec<Color>,
    /// How much the bottle can hold
    pub size: usize,
}

impl PartialOrd for Bottle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Bottle {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_empty() {
            return Ordering::Less;
        }
        if other.is_empty() {
            return Ordering::Greater;
        }

        if self.contents.last() < other.contents.last() {
            return Ordering::Less;
        }

        if self.contents.last() > other.contents.last() {
            return Ordering::Greater;
        }

        if self.contents.last() == other.contents.last() {
            return Ordering::Equal;
        }

        Ordering::Equal
    }
}

impl Bottle {
    /// Returns a new empty bottle with given size
    pub fn new(size: usize) -> Bottle {
        Bottle {
            contents: Vec::with_capacity(size),
            size,
        }
    }

    /// Returns a new bottle of a given size filled with just one color
    fn uni_bottle(size: usize, color: Color) -> Bottle {
        Bottle {
            contents: std::iter::repeat(color).take(size).collect::<Vec<Color>>(),
            size,
        }
    }

    /// Returns a new bottle of a given size filled with a certain amount of one color
    fn rep_bottle(size: usize, color: Color, count: usize) -> Bottle {
        Bottle {
            contents: std::iter::repeat(color).take(count).collect::<Vec<Color>>(),
            size,
        }
    }

    /// Wether a bottle is considered solved
    ///
    /// A bottle is solved if it is both full and all it's content is one single color
    pub fn is_solved(&self) -> bool {
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

        self.is_full()
    }

    /// Wether a bottle is full
    pub fn is_full(&self) -> bool {
        self.contents.len() == self.size
    }

    /// Wether a bottle is empty, or has no contents
    pub fn is_empty(&self) -> bool {
        self.contents.len() == 0
    }

    /// Pour takes the contents of one bottle and attempts to pour it into another bottle
    ///
    /// The mechanism for pouring between bottles is actually pretty complex.
    /// These are the requirements:
    /// - the pourer can't be empty
    /// - the poureee can't be full
    /// - the coloured being poured from the pourer must match the topmost color in the pouree, except if the pouree is empty
    ///
    /// As long as all the requirements are met the pourer will pour until the requirements are no longer met.
    /// That is to say that after one transfer of liquid, all the requirements are still met, another transfer of liquid will happen.
    /// This happens until the requirements are no longer met.
    pub fn pour(mut pourer: Bottle, mut pouree: Bottle) -> (Bottle, Bottle) {
        if pouree.size == pouree.contents.len() {
            return (pourer, pouree);
        }

        // A full solved bottle isn't allowed to be poured
        if pourer.size == pourer.contents.len() && pourer.is_solved() {
            return (pourer, pouree);
        }

        while pouree.contents.len() < pouree.size && !pourer.contents.is_empty() {
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

        (pourer, pouree)
    }
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

    #[test]
    fn a_bottle_is_solved_only_if_its_full_and_the_same_color() {
        let full_bottle = Bottle {
            contents: vec![Color::Red, Color::Red, Color::Red, Color::Red],
            size: 4,
        };

        let half_bottle = Bottle::rep_bottle(4, Color::Red, 3);

        let mixed_bottle = Bottle {
            contents: vec![Color::Red, Color::Blue, Color::Red, Color::Blue],
            size: 4,
        };

        assert!(full_bottle.is_solved());
        assert!(!half_bottle.is_solved());
        assert!(!mixed_bottle.is_solved());
    }
}
