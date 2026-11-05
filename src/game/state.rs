use crate::*;

// ——— State ——————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct State {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub big_blind: usize,

    community_cards: [Option<Card>; 5],
    pot: usize,
    dealer: usize,

    cur_street: Street,
    cur_player: usize,
    cur_alpha: usize,
    cur_bet: usize,
    cur_raise: usize,
}

impl State {
    pub fn create(players: usize) -> Self {
        let dealer = thread_rng().gen_range(0..players);
        let big_blind = 20;        // 10/20 game

        Self {
            players: vec![Player::new(1000); players],    // TODO: implement buy-in later
            deck: Deck::new(),
            big_blind,
            community_cards: [None; 5],
            pot: 0,
            dealer, 
            cur_street: Preflop,
            cur_player: (dealer + 3) % players,    // TODO: not handling 2-player game (i think)
            cur_alpha: (dealer + 3) % players,
            cur_bet: 0,
            cur_raise: big_blind,
        }
    }

    pub fn start(&mut self) {
        self.deck.shuffle();

        // bb / sb post blind
        let players_len = self.players.len();

        let mut post = |player_idx: usize, bet_amount: usize| {
            let player = &mut self.players[player_idx];
            player.bet(bet_amount);
            self.pot += bet_amount;
            self.cur_bet = player.bet;
        };

        post((self.dealer + 1) % players_len, self.big_blind / 2);
        post((self.dealer + 2) % players_len, self.big_blind);
        self.cur_bet = self.big_blind;

        // deal hole cards
        self.players.iter_mut().for_each(|p| p.hole_cards[0] = Some(self.deck.draw()));
        self.players.iter_mut().for_each(|p| p.hole_cards[1] = Some(self.deck.draw()));
    }

    pub fn turn(&mut self, action: Action) {
        let player = &mut self.players[self.cur_player];

        match action {
            Fold => {
                player.hole_cards = [None, None];
                if self.players.iter().filter(|player| player.hole_cards[0].is_some()).count() == 1 {
                    // TODO: win condition (last remaining player wins)
                }
            },
            Call => {
                let bet_amount = self.cur_bet - player.bet;
                player.bet(bet_amount);
                self.pot += bet_amount;
                // TODO: handle case where he can't afford current bet and needs to go all in (side pot dynamics)
            },
            Raise(bet_total) => {
                if bet_total < self.cur_bet + self.cur_raise { panic!("bet is too small"); }
                let bet_amount = bet_total - player.bet;
                player.bet(bet_amount);
                self.pot += bet_amount;
                self.cur_raise = bet_total - self.cur_bet;
                self.cur_bet = player.bet;
                self.cur_alpha = self.cur_player;
            }
        }

        // next player
        loop {
            self.cur_player = (self.cur_player + 1) % self.players.len();
            if self.players[self.cur_player].hole_cards[0].is_some() { break; }
        }

        // next street
        if self.cur_player == self.cur_alpha {
            self.cur_bet = 0;
            self.cur_raise = self.big_blind;
            self.players.iter_mut().for_each(|p| p.bet = 0);
            self.cur_street.next_street();

            // reset cur player & cur alpha
            self.cur_player = self.dealer;

            loop {
                self.cur_player = (self.cur_player + 1) % self.players.len();
                if self.players[self.cur_player].hole_cards[0].is_some() { break; }
            }

            self.cur_alpha = self.cur_player;

            // fill community cards
            match self.cur_street {
                Flop => (0..3).for_each(|i| self.community_cards[i] = Some(self.deck.draw())),
                Turn => self.community_cards[3] = Some(self.deck.draw()),
                River => self.community_cards[4] = Some(self.deck.draw()),
                Showdown => panic!("game is done"),
                _ => panic!("cannot fill that street"),
            }
        }
    }

    pub fn table(&self) {
        println!(
            "{} \nCommunity cards: {} {} {} {} {} \nPot: {} \nCurrent bet: {} \nMinimum raise: {} \n",
            self.cur_street,
            if let Some(card) = self.community_cards[0] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[1] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[2] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[3] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[4] { card.raw() } else { "".to_string() },
            self.pot,
            self.cur_bet,
            self.cur_bet + self.cur_raise,
        );

        (0..self.players.len()).for_each(|i| {
            let c = (self.dealer + i) % self.players.len();
            let p = self.players[c];
            let cur_player_text = if self.cur_player == c { "(cur)" } else { "" };
            let cur_alpha_text = if self.cur_alpha == c { "(alpha)" } else { "" };
            let space = if self.cur_player == c && self.cur_alpha == c { " " } else { "" };

            println!(
                "Player {} {}{}{} \nHole cards: {} {} \nBet: {} \nStack: {} \n",
                i,
                cur_player_text,
                space,
                cur_alpha_text,
                if let Some(card) = p.hole_cards[0] { card.raw() } else { "X".to_string() },
                if let Some(card) = p.hole_cards[1] { card.raw() } else { "X".to_string() },
                p.bet,
                p.stack,
            );
        });
    }
}


// ——— Enums ——————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
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

impl fmt::Display for Street {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Preflop => write!(f, "Preflop"),
            Flop => write!(f, "Flop"),
            Turn => write!(f, "Turn"),
            River => write!(f, "River"),
            Showdown => write!(f, "Showdown"),
        }
    }
}

pub enum Action { Fold, Call, Raise(usize) }
pub use Action::*;


// ——— Player —————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy)]
pub struct Player {
    pub stack: usize,
    pub hole_cards: [Option<Card>; 2],
    pub bet: usize,
}

impl Player {
    fn new(stack: usize) -> Self {
        Self { stack, hole_cards: [None, None], bet: 0 }
    }

    fn bet(&mut self, bet_amount: usize) {
        if self.stack < bet_amount { panic!("cannot afford") }
        self.stack -= bet_amount;
        self.bet += bet_amount;
    }
}

