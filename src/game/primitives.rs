use crate::*;

// ——— Primitves ——————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Rank { Two, Three, Four, Five, Six, Seven, Eight, Nine, Ten, Jack, Queen, King, Ace }
pub use Rank::*;

impl Rank {
    pub fn next(&self) -> Rank {
        match self {
            Two => Three,
            Three => Four,
            Four => Five,
            Five => Six,
            Six => Seven,
            Seven => Eight,
            Eight => Nine,
            Nine => Ten,
            Ten => Jack,
            Jack => Queen,
            Queen => King,
            King => Ace,
            Ace => panic!("nothing higher than ace"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Suit { Clubs, Diamonds, Hearts, Spades }
pub use Suit::*;


// ——— Card ——————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Clone, Copy, Eq, PartialOrd)]
pub struct Card {
    pub rank: Rank,
    pub suit: Suit,
}

impl PartialEq for Card {
    fn eq(&self, other: &Self) -> bool {
        self.rank == other.rank
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank.cmp(&other.rank)
    }
}


impl Debug for Card {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.raw())
    }
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

    pub fn strictly_eq(&self, other: &Card) -> bool {
        return self.rank == other.rank && self.suit == other.suit
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

    pub fn from_sim() -> Self {
        let file = File::open("sim.txt").unwrap();
        let reader = BufReader::new(file);
        let mut deck = Deck::new();

        let cards_parsed: Vec<Card> = reader.lines()
            .map(|line| line.unwrap())
            .filter(|line| !line.is_empty())
            .map(|line| {
                let mut line = line.chars();
                let (r, s) = (line.next().unwrap(), line.next().unwrap());
                if r == '?' { return None };
                
                let rank = match r {
                    '2' => Two,
                    '3' => Three,
                    '4' => Four,
                    '5' => Five,
                    '6' => Six,
                    '7' => Seven,
                    '8' => Eight,
                    '9' => Nine,
                    '0' => Ten,
                    'a' | 'A' => Ace,
                    'j' | 'J' => Jack,
                    'q' | 'Q' => Queen,
                    'k' | 'K' => King,
                    _ => panic!("error parsing rank txt")
                };

                let suit = match s {
                    'c' | 'C' => Clubs,
                    'd' | 'D' => Diamonds,
                    'h' | 'H' => Hearts,
                    's' | 'S' => Spades,
                    _ => panic!("error parsing suit in txt")
                };

                Some(Card::new(rank, suit))
            })
            .map(|card| {
                match card {
                    Some(card) => deck.pick_out(card),
                    None => deck.draw()
                }
            })
            .collect();

        deck.draw_pile = deck.draw_pile.into_iter().chain(cards_parsed.into_iter().rev()).collect();

        deck
    }

    pub fn shuffle(&mut self) {
        let mut rng = StdRng::seed_from_u64(1);
        self.draw_pile.shuffle(&mut rng);
    }

    pub fn draw(&mut self) -> Card {
        match self.draw_pile.pop() {
            Some(card) => card,
            None => panic!("out of cards"),
        }
    }

    pub fn burn(&mut self) {
        self.draw_pile.pop();
    }

    pub fn pick_out(&mut self, target: Card) -> Card {
        self.draw_pile.retain(|&card| card != target);
        target
    }
}

