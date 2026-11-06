use crate::*;

// ——— Enums ——————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, PartialEq, Eq)]
pub enum Street { Preflop, Flop, Turn, River, Showdown }
pub use Street::*;

impl Street {
    pub fn next_street(&mut self) {
        *self = match self {
            Preflop => Flop,
            Flop => Turn,
            Turn => River,
            River => Showdown,
            Showdown => panic!("reset todo"),
        };
    }
}

impl Display for Street {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Preflop => write!(f, "Preflop"),
            Flop => write!(f, "Flop"),
            Turn => write!(f, "Turn"),
            River => write!(f, "River"),
            Showdown => write!(f, "Showdown"),
        }
    }
}

pub enum Action { Fold, Call, Raise(usize), AllIn }
pub use Action::*;


// ——— Player —————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Player {
    pub stack: usize,
    pub hole_cards: [Option<Card>; 2],
    pub bet: usize,
    pub stake: usize,
}

impl Player {
    pub fn new(stack: usize) -> Self {
        Self { stack, hole_cards: [None, None], bet: 0, stake: 0, }
    }

    pub fn bet(&mut self, bet_amount: usize) {
        if self.stack < bet_amount { panic!("cannot afford") }
        self.stack -= bet_amount;
        self.bet += bet_amount;
        self.stake += bet_amount;
    }

    pub fn all_in(&mut self) {
        if self.stack == 0 { panic!("ur broke") }
        self.bet += self.stack;
        self.stake += self.stack;
        self.stack = 0;
    }
}

