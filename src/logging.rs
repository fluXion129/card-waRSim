#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Round {
    Loss,
    War,
    Victory,
}

#[derive(PartialEq, Eq)]
pub struct Streak {
    typ: Round,
    length: usize,
}
impl From<&Round> for Streak {
    fn from(round: &Round) -> Self {
        Self {
            typ: *round,
            length: 1,
        }
    }
}
impl From<&Streak> for Round {
    fn from(streak: &Streak) -> Self {
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
        if self.typ != rhs.typ {
            return None;
        }
        Some(self.length.cmp(&rhs.length))
    }
}
impl Streak {
    pub fn from_rounds(rounds: &[Round]) -> Vec<Self> {
        let mut streaks: Vec<Self> = vec![];
        for round in rounds.iter() {
            match streaks.last_mut() {
                Some(streak) if streak.typ == *round => {
                    streak.length += 1;
                }
                _ => streaks.push(round.into()),
            }
        }
        streaks
    }

    pub fn typ(&self) -> Round {
        self.typ
    }
    pub fn length(&self) -> usize {
        self.length
    }
}
