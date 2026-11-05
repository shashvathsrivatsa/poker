use crate::*;

// ——— Primitves ——————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy)]
pub enum Rank { Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King }
pub use Rank::*;

#[derive(Debug, Clone, Copy)]
pub enum Suit { Clubs, Diamonds, Hearts, Spades }
pub use Suit::*;


// ——— Card ——————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Self { rank, suit }
    }

    pub fn default() -> Self {
        Self { rank: Ace, suit: Spades }
    }

    pub fn raw(&self) -> String {
        let rank = match self.rank {
            Ace   => "A",
            Two   => "2",
            Three => "3",
            Four  => "4",
            Five  => "5",
            Six   => "6",
            Seven => "7",
            Eight => "8",
            Nine  => "9",
            Ten   => "10",
            Jack  => "J",
            Queen => "Q",
            King  => "K",
        };

        let suit = match self.suit {
            Clubs    => "♣️",
            Diamonds => "♦️",
            Hearts   => "♥️",
            Spades   => "♠️",
        };

        format!("{rank}{suit}")
    }
}


// ——— Deck ——————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct Deck {
    pub draw_pile: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let draw_pile: Vec<Card> = [ Clubs, Diamonds, Hearts, Spades ].iter().flat_map(|&suit| {
            [ Ace, Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King ].iter().map(move |&rank| {
                Card::new(rank, suit)
            })
        }).collect();

        Self { draw_pile }
    }

    pub fn shuffle(&mut self) {
        let mut rng = StdRng::seed_from_u64(42);
        self.draw_pile.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Card {
        match self.draw_pile.pop() {
            Some(card) => card,
            None => panic!("out of cards"),
        }
    }
}

