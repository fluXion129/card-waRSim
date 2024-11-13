
mod cards {
    use strum::IntoEnumIterator;
    use strum_macros::EnumIter;

    pub fn deck() -> Vec<Card> {
        let mut deck = vec![];
        for suite in Suite::iter() {
            for value in Value::iter() {
                deck.push(Card::new(value, suite));
            }
        }
        deck
    }
    
    #[derive(Clone, Debug, PartialEq, Eq)]
    pub struct Card {
        value: Value,
        suite: Suite
    }
    impl Card {
        pub fn new(value: Value, suite: Suite) -> Self {
            Self { suite, value }
        }
    }
    impl std::fmt::Display for Card {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
            write!(f, "'{:?} of {:?}'", self.value, self.suite)
        }
    }
    impl std::cmp::PartialOrd for Card {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    impl std::cmp::Ord for Card {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.value.cmp(&other.value)
        }
    }

    #[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
    pub enum Value {
       Ace = 14, Two = 2, Three = 3, Four = 4, Five = 5, Six = 6, Seven = 7, Eight = 8, Nine = 9, Ten = 10, Jack = 11, Queen = 12, King = 13
    }

    #[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
    pub enum Suite {
        Clubs, Spades, Hearts, Diamonds
    }
}

use cards::Card;

use rand::Rng;

fn shuffle<T>(vec: &mut Vec<T>) {
    let mut new_vec = vec![];
    let mut rng = rand::thread_rng();
    while !vec.is_empty() {
        new_vec.push(vec.swap_remove(rng.gen::<usize>() % vec.len()));
    }
    *vec = new_vec;
}

struct Player {
    deck: Vec<Card>,
    spoils: Vec<Card>
}
impl Player {
    pub fn new(deck: Vec<Card>) -> Self {
        Self {
            deck,
            spoils: vec![]
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
            some => some
        }
    }

    pub fn recycle_spoils(&mut self) -> bool {
        if self.spoils.is_empty() { return false; }
        shuffle(&mut self.spoils);
        self.deck.append(&mut self.spoils);
        true
    }

    pub fn take_spoils(&mut self, cards: &mut Vec<Card>) {
        self.spoils.append(cards)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Round {
    Loss,
    War,
    Victory
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
        let Some(player_card) = player.play_card() else { return false; };
        let Some(com_card) = com.play_card() else { return true; };

        println!("{player_card} vs {com_card}");
        match player_card.cmp(&com_card) {
            std::cmp::Ordering::Equal => {
                println!("War!");
                hostages.push(player_card);
                hostages.push(com_card);
                for _ in 0..num_war_hostages {
                    let Some(card) = player.play_card() else { return false; };
                    hostages.push(card);
                    let Some(card) = com.play_card() else { return true; };
                    hostages.push(card);
                }
                
                print!("Hostages: [");
                for card in hostages.iter() {
                    print!(" {card} ");
                }
                println!("]");
                rounds.push(Round::War);
            },
            std::cmp::Ordering::Less => {
                println!("Loss!");
                com.take_spoils(&mut vec![player_card, com_card]);
                com.take_spoils(&mut hostages);
                rounds.push(Round::Loss);
            },
            std::cmp::Ordering::Greater => {
                println!("Victory!");
                player.take_spoils(&mut vec![player_card, com_card]);
                player.take_spoils(&mut hostages);
                rounds.push(Round::Victory);
            }
        }
        println!("You: {:?}, com: {:?}\n", player.num_cards(), com.num_cards());
    }
}

#[derive(PartialEq, Eq)]
struct Streak {
    typ: Round,
    length: usize
}
impl From<&Round> for Streak {
    fn from(round: &Round) -> Self {
        Self {
            typ: *round,
            length: 1
        }
    }
}
impl From<Streak> for Round {
    fn from(streak: Streak) -> Self {
    streak.typ
    }
}
impl std::ops::AddAssign<usize> for Streak {
    fn add_assign(&mut self, rhs: usize) {
        self.length += rhs;
    }
}
impl std::cmp::PartialOrd for Streak {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        if self.typ != rhs.typ { return None; }
        Some(self.length.cmp(&rhs.length))
    }
}
impl Streak {
    fn from_rounds(rounds: &[Round]) -> Vec<Self> {
        let mut streaks: Vec<Self> = vec![];
        for round in rounds.iter() {
            match streaks.last_mut() {
                Some(streak) if streak.typ == *round => { streak.length += 1; },
                _ => streaks.push(round.into())
            }
        }
        streaks
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
        let longest_war = streaks.iter().filter(|streak| streak.typ == Round::War).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0);
        num_games += 1;
        if longest_war >= num_wars_search {
            println!("Took {num_games} games to find {num_wars_search}+ war");
            if victory {
                println!("You win!");
            } else {
                println!("You lost...");
            }
            let victories = rounds.iter().filter(|round| **round == Round::Victory).count();
            let losses = rounds.iter().filter(|round| **round == Round::Loss).count();
            let wars = rounds.iter().filter(|round| **round == Round::War).count();
            let percent_victories = victories as f32 / rounds.len() as f32 * 100.0;
            let percent_losses = losses as f32 / rounds.len() as f32 * 100.0;
            let percent_wars = wars as f32 / rounds.len() as f32 * 100.0;

            println!("The game lasted {} rounds:", rounds.len());
            println!("{victories} ({percent_victories}%) victories, {losses} ({percent_losses}%) losses, and {wars} ({percent_wars}%) wars.");
            println!("Your longest winning streak was {}, longest losing streak was {}, and longest war was {} round(s)",
                streaks.iter().filter(|streak| streak.typ == Round::Victory).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
                streaks.iter().filter(|streak| streak.typ == Round::Loss).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
                streaks.iter().filter(|streak| streak.typ == Round::War).max_by(|x, y| x.partial_cmp(y).unwrap()).map(|x| x.length).unwrap_or(0),
            );
            break;
        }
        
        if num_games > max_games {
            println!("Even after {max_games} games, there was no {num_wars_search} war.");
            break;
        }
    }
    
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
