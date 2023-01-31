use std::collections::HashMap;

use ::bottle::bottle::Bottle;
use ::bottle::color::Color::*;
use ::bottle::*;

fn main() {
    // Level 119 of sortpuz for android
    let game = Vec::from([
        Bottle {
            contents: Vec::from([LOrange, Pink, Pink, Brown]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([LOrange, Blue, Green, Green]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Pink, Orange, Blue, DBlue]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Brown, DBlue, Brown, DGreen]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Blue, DBlue, DGreen, Orange]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([LOrange, Green, Green, Orange]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([LOrange, DGreen, Blue, DBlue]),
            size: 4,
        },
        Bottle {
            contents: Vec::from([Orange, Pink, DGreen, Brown]),
            size: 4,
        },
        Bottle::new(4),
        Bottle::new(4),
    ]);

    /*
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
    */

    println!("Solving!!!");
    game.iter().for_each(|bottle| {
        println!("{:?}", bottle);
    });
    println!();
    println!();

    let best = find_memo(&game, &mut HashMap::new()).unwrap();

    best.iter().for_each(|game| {
        game.iter().for_each(|bottle| {
            println!("{:?}", bottle);
        });
        println!();
    });

    println!("Done!!!!!!");
}
