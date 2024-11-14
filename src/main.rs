mod cards;
use cards::Card;

mod logging;
use logging::{Round, Streak};

fn shuffle<T>(vec: &mut Vec<T>) {
    let mut new_vec = vec![];
    while !vec.is_empty() {
        new_vec.push(vec.swap_remove(rand::random::<usize>() % vec.len()));
    }
    *vec = new_vec;
}

struct Player {
    deck: Vec<Card>,
    spoils: Vec<Card>,
}
impl Player {
    pub fn new(deck: Vec<Card>) -> Self {
        Self {
            deck,
            spoils: vec![],
        }
    }

    pub fn num_cards(&self) -> (usize, usize) {
        (self.deck.len(), self.spoils.len())
    }

    pub fn play_card(&mut self) -> Option<Card> {
        match self.deck.pop() {
            None => {
                self.recycle_spoils();
                self.deck.pop()
            }
            some => some,
        }
    }

    pub fn recycle_spoils(&mut self) -> bool {
        if self.spoils.is_empty() {
            return false;
        }
        shuffle(&mut self.spoils);
        self.deck.append(&mut self.spoils);
        true
    }

    pub fn take_spoils(&mut self, cards: &mut Vec<Card>) {
        self.spoils.append(cards)
    }
}

fn play_war_game(num_war_hostages: usize, rounds: &mut Vec<Round>) -> bool {
    let mut deck = cards::deck();
    shuffle(&mut deck);
    let other_deck = deck.split_off(deck.len() / 2);

    let mut player = Player::new(deck);
    let mut com = Player::new(other_deck);

    let mut hostages = vec![];
    rounds.clear();

    // making more than 2-player game is difficult
    // let mut players = vec![player, com];
    // let mut rounds_cards = get_rounds_cards(&mut players);

    loop {
        let Some(player_card) = player.play_card() else {
            return false;
        };
        let Some(com_card) = com.play_card() else {
            return true;
        };

        println!("{player_card} vs {com_card}");
        match player_card.cmp(&com_card) {
            std::cmp::Ordering::Equal => {
                println!("War!");
                hostages.push(player_card);
                hostages.push(com_card);
                for _ in 0..num_war_hostages {
                    let Some(card) = player.play_card() else {
                        return false;
                    };
                    hostages.push(card);
                    let Some(card) = com.play_card() else {
                        return true;
                    };
                    hostages.push(card);
                }

                print!("Hostages: [");
                for card in hostages.iter() {
                    print!(" {card} ");
                }
                println!("]");
                rounds.push(Round::War);
            }
            std::cmp::Ordering::Less => {
                println!("Loss!");
                com.take_spoils(&mut vec![player_card, com_card]);
                com.take_spoils(&mut hostages);
                rounds.push(Round::Loss);
            }
            std::cmp::Ordering::Greater => {
                println!("Victory!");
                player.take_spoils(&mut vec![player_card, com_card]);
                player.take_spoils(&mut hostages);
                rounds.push(Round::Victory);
            }
        }
        println!(
            "You: {:?}, com: {:?}\n",
            player.num_cards(),
            com.num_cards()
        );
    }
}

fn main() {
    let mut rounds = vec![];
    let mut num_games = 0;
    let num_wars_search = 0;
    let max_games = 1;
    loop {
        let victory = play_war_game(2, &mut rounds);
        let streaks = Streak::from_rounds(&rounds);
        let longest_war = streaks
            .iter()
            .filter(|streak| streak.typ() == Round::War)
            .max_by(|x, y| x.partial_cmp(y).unwrap())
            .map(|x| x.length())
            .unwrap_or(0);
        num_games += 1;
        if longest_war >= num_wars_search {
            println!("Took {num_games} games to find {num_wars_search}+ war");
            if victory {
                println!("You win!");
            } else {
                println!("You lost...");
            }
            let victories = rounds
                .iter()
                .filter(|round| **round == Round::Victory)
                .count();
            let losses = rounds.iter().filter(|round| **round == Round::Loss).count();
            let wars = rounds.iter().filter(|round| **round == Round::War).count();
            let percent_victories = victories as f32 / rounds.len() as f32 * 100.0;
            let percent_losses = losses as f32 / rounds.len() as f32 * 100.0;
            let percent_wars = wars as f32 / rounds.len() as f32 * 100.0;

            println!("The game lasted {} rounds:", rounds.len());
            println!("{victories} ({percent_victories}%) victories, {losses} ({percent_losses}%) losses, and {wars} ({percent_wars}%) wars.");
            println!("Your longest winning streak was {}, longest losing streak was {}, and longest war was {} round(s)",
                streaks.iter().filter(|streak| streak.typ() == Round::Victory).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length()).unwrap_or(0),
                streaks.iter().filter(|streak| streak.typ() == Round::Loss).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length()).unwrap_or(0),
                streaks.iter().filter(|streak| streak.typ() == Round::War).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length()).unwrap_or(0),
            );
            break;
        }

        if num_games > max_games {
            println!("Even after {max_games} games, there was no {num_wars_search} war.");
            break;
        }
    }
    // this code is for when searching for multi-wars
    //     if play_war_game(2, &mut rounds) {
    //         println!("You win!");
    //     } else {
    //         println!("You lost...");
    //     }
    //     println!("The game lasted {} rounds:", rounds.len());
    //     println!("{} victories, {} losses, and {} wars.",
    //         rounds.iter().filter(|round| **round == Round::Victory).count(),
    //         rounds.iter().filter(|round| **round == Round::Loss).count(),
    //         rounds.iter().filter(|round| **round == Round::War).count()
    //     );
    //     let streaks = Streak::from_rounds(&rounds);
    //     println!("Your longest winning streak was {}, longest losing streak was {}, and longest war was {} round(s)",
    //         streaks.iter().filter(|streak| streak.typ == Round::Victory).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
    //         streaks.iter().filter(|streak| streak.typ == Round::Loss).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
    //         streaks.iter().filter(|streak| streak.typ == Round::War).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
    //    );
}
