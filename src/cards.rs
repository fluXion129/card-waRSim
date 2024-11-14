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
    suite: Suite,
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
    Ace = 14,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
}

#[derive(Clone, Copy, Debug, EnumIter, PartialEq, Eq)]
pub enum Suite {
    Clubs,
    Spades,
    Hearts,
    Diamonds,
}
