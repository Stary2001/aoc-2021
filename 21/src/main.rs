
struct Dice {
    value: usize,
    pub rolls: usize
}

impl Dice {
    fn roll(&mut self) -> usize {
        self.rolls += 1;

        let old = self.value;
        self.value += 1;
        if self.value == 101 {
            self.value = 1;
        }

        return old;
    }
}

#[derive(Debug, Clone)]
struct Player {
    space: usize,
    score: usize
}

impl Player {
    fn step(&mut self, dice_rolls: usize) {
        self.space = (self.space - 1 + dice_rolls) % 10 + 1;
        self.score += self.space;
    }
}

fn part_1(player_1_space: usize, player_2_space: usize) -> usize {
    // do it
    let mut dice = Dice { rolls: 0, value: 1 };
    let mut player_1 = Player { space: player_1_space, score: 0 };
    let mut player_2 = Player { space: player_2_space, score: 0 };

    loop {
        let player_1_dice = dice.roll() + dice.roll() + dice.roll();
        player_1.step(player_1_dice);
        if player_1.score >= 1000 {
            break;
        }

        let player_2_dice = dice.roll() + dice.roll() + dice.roll();
        player_2.step(player_2_dice);
        if player_2.score >= 1000 {
            break;
        }
    }

    [player_1.score, player_2.score].iter().min().unwrap() * dice.rolls
}

// generated with list(itertools.product([1,2,3], [1,2,3], [1,2,3]))
// then deduplicated by hand lmao

const DICE_ROLL_TABLE: [(usize, usize); 7] = [(3, 1), (4,3), (5,6), (6,7), (7,6), (8, 3), (9, 1)];
fn dirac_player_1(player_1: Player, player_2: Player, print: bool) -> (usize, usize) {
    let mut new_wins: (usize, usize) = (0, 0);

    for (dice, mult) in DICE_ROLL_TABLE {
        let mut player_1 = player_1.clone();

        player_1.step(dice);
        if player_1.score >= 21 {
            new_wins.0 += mult;
        } else {
            let future_wins = dirac_player_2(player_1, player_2.clone(), false);
            if print {
                println!("{:?}", future_wins);
            }
            new_wins.0 += future_wins.0 * mult;
            new_wins.1 += future_wins.1 * mult;
        }
    }

    new_wins
}

fn dirac_player_2(player_1: Player, player_2: Player, print: bool) -> (usize, usize) {
    let mut new_wins: (usize, usize) = (0, 0);

    for (dice, mult) in DICE_ROLL_TABLE {
        let mut player_2 = player_2.clone();

        player_2.step(dice);
        if player_2.score >= 21 {
            new_wins.1 += mult;
        } else {
            let future_wins = dirac_player_1(player_1.clone(), player_2, false);
            new_wins.0 += future_wins.0 * mult;
            new_wins.1 += future_wins.1 * mult;

            if print {
                println!("{:?}", future_wins);
            }
        }
    }

    new_wins
}

fn part_2(player_1_space: usize, player_2_space: usize) -> usize {
    let player_1 = Player { space: player_1_space, score: 0 };
    let player_2 = Player { space: player_2_space, score: 0 };

    let (player_1_results, player_2_results) = dirac_player_1(player_1, player_2, true);

    *[player_1_results, player_2_results].iter().max().unwrap()
}


#[test]
fn test() {
    assert_eq!(part_1(4, 8), 739785);
    assert_eq!(part_2(4, 8), 444356092776315);
}

fn main() {
    println!("Part 1: {:?}", part_1(10, 4));
    println!("Part 2: {:?}", part_2(10, 4));
}
