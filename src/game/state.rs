use crate::*;

// ——— State ——————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug)]
pub struct State {
    pub players: Vec<Player>,                  // game-scope
    pub deck: Deck,
    pub big_blind: usize,

    pub community_cards: [Option<Card>; 5],    // hand-scope
    pub pot: usize,
    pub dealer: usize,

    pub cur_street: Street,                    // street-scope
    pub cur_player: usize,
    pub cur_alpha: usize,
    pub cur_bet: usize,
    pub cur_raise: usize,
}

impl State {
    fn create_std(players: usize, deck: Deck) -> Self {
        // let dealer = thread_rng().gen_range(0..players);
        let dealer = 0;
        let big_blind = 400;
        let cur_player = if players == 2 { 0 } else { (dealer + 3) % players };

        Self {
            players: vec![Player::new(10_000); players],
            deck,
            big_blind,
            community_cards: [None; 5],
            pot: 0,
            dealer, 
            cur_street: Preflop,
            cur_player,
            cur_alpha: cur_player,
            cur_bet: big_blind,
            cur_raise: big_blind,
        }

    }

    pub fn create(players: usize) -> Self {
        let mut deck = Deck::new();
        deck.shuffle();
        State::create_std(players, deck)
    }

    pub fn create_sim(players: usize) -> Self {
        State::create_std(players, Deck::from_sim())
    }

    pub fn start(&mut self) {

        // posting blinds
        let players_len = self.players.len();

        let mut post = |player_idx: usize, bet_amount: usize| {
            let player = &mut self.players[player_idx];
            player.bet(bet_amount);
            self.pot += bet_amount;
            self.cur_bet = player.bet;
        };

        if players_len == 2 {
            post((self.dealer + 0) % players_len, self.big_blind / 2);
            post((self.dealer + 1) % players_len, self.big_blind);
        } else {
            post((self.dealer + 1) % players_len, self.big_blind / 2);
            post((self.dealer + 2) % players_len, self.big_blind);
        }

        // deal hole cards
        self.players.iter_mut().for_each(|p| p.hole_cards[0] = Some(self.deck.draw()));
        self.players.iter_mut().for_each(|p| p.hole_cards[1] = Some(self.deck.draw()));
    }

    pub fn turn(&mut self, action: Action) {
        if self.cur_street == Showdown { panic!("game is over"); }

        let player = &mut self.players[self.cur_player];

        match action {
            Fold => {
                player.hole_cards = [None, None];
                if self.players.iter().filter(|player| player.hole_cards[0].is_some()).count() == 1 {
                    self.showdown(true);
                    return;
                }
            },
            Call => {
                let bet_amount = self.cur_bet - player.bet;
                player.bet(bet_amount);
                self.pot += bet_amount;
            },
            Raise(bet_total) => {
                if bet_total < self.cur_bet + self.cur_raise { panic!("bet is too small"); }
                let bet_amount = bet_total - player.bet;
                player.bet(bet_amount);
                self.pot += bet_amount;
                self.cur_raise = bet_total - self.cur_bet;
                self.cur_bet = player.bet;
                self.cur_alpha = self.cur_player;
            },
            AllIn => {
                let all_in_amount = player.stack;
                player.all_in();
                self.pot += all_in_amount;
                
                if all_in_amount - self.cur_bet >= self.cur_raise {
                    self.cur_raise = all_in_amount - self.cur_bet;
                    self.cur_alpha = self.cur_player;
                }

                self.cur_bet = usize::max(player.bet, self.cur_bet);

                if self.players.iter().all(|player| player.stack == 0) { self.showdown(true); }
            },
        }

        // next player
        loop {
            self.cur_player = (self.cur_player + 1) % self.players.len();
            if self.players[self.cur_player].hole_cards[0].is_some() { break; }
        }

        // next street
        if self.cur_player == self.cur_alpha {
            self.cur_bet = 0;
            // self.cur_raise = self.big_blind;
            self.cur_raise = 0;        // TODO: just for current hand
            self.players.iter_mut().for_each(|p| p.bet = 0);
            self.cur_street.next_street();

            // reset cur player & cur alpha
            self.cur_player = self.dealer;

            loop {
                self.cur_player = (self.cur_player + 1) % self.players.len();
                if self.players[self.cur_player].hole_cards[0].is_some() { break; }
            }

            self.cur_alpha = self.cur_player;

            // prep for next street
            match self.cur_street {
                Preflop => panic!("not reachable (nothing before preflop)"),
                Flop => {
                    self.deck.burn();
                    (0..3).for_each(|i| self.community_cards[i] = Some(self.deck.draw()));
                },
                Turn => {
                    self.deck.burn();
                    self.community_cards[3] = Some(self.deck.draw());
                },
                River => {
                    self.deck.burn();
                    self.community_cards[4] = Some(self.deck.draw());
                },
                Showdown => self.showdown(false),
            }
        }
    }

    fn showdown(&mut self, early: bool) {
        if early {
            self.community_cards.iter_mut().enumerate()
                .filter(|(_, card)| card.is_none())
                .for_each(|(i, card)| {
                    if i == 0 || i == 3 || i == 4 { self.deck.burn(); }
                    *card = Some(self.deck.draw())
                });

            self.cur_street = Showdown;
        }

        let rankings: Vec<usize> = self.players.iter_mut().enumerate()
            .filter(|(_, player)| player.hole_cards[0].is_some())
            .sorted_by(|(_, p1), (_, p2)| {
                let p2_rank = hand_rank_some(&p2.hole_cards, &self.community_cards);
                let p1_rank = hand_rank_some(&p1.hole_cards, &self.community_cards);
                p2_rank.cmp(&p1_rank)
            })
            .map(|(i, _)| i)
            .collect();

        for &i in rankings.iter() {
            let winner_stake = self.players[i].stake;

            let winner_win_amount = self.players.iter_mut().enumerate()
                .filter(|&(j, _)| i != j)
                .fold(0, |acc, (_, player)| {
                    let rinse_amount = usize::min(player.stake, winner_stake);
                    player.stake -= rinse_amount;
                    acc + rinse_amount
                });

            self.players[i].stack += winner_win_amount + self.players[i].stake;
            self.players[i].stake = 0;
        }

        self.pot = 0;
    }

    pub fn table(&self) {
        println!(
            "{} \nCommunity cards: {} {} {} {} {} \nPot: ${} \nCurrent bet: ${} \nMinimum raise: ${} \n",
            self.cur_street,
            if let Some(card) = self.community_cards[0] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[1] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[2] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[3] { card.raw() } else { "".to_string() },
            if let Some(card) = self.community_cards[4] { card.raw() } else { "".to_string() },
            self.pot.separate_with_commas(),
            self.cur_bet.separate_with_commas(),
            (self.cur_bet + self.cur_raise).separate_with_commas(),
        );

        (0..self.players.len()).for_each(|i| {
            let c = (self.dealer + i) % self.players.len();
            let p = &self.players[c];
            let cur_player_text = if self.cur_player == c { "(cur)" } else { "" };
            let cur_alpha_text = if self.cur_alpha == c { "(alpha)" } else { "" };
            let space = if self.cur_player == c && self.cur_alpha == c { " " } else { "" };

            println!(
                "Player {} {}{}{} \nHole cards: {} {} \n{}Bet: ${} \nStake: {} \nStack: ${} \n",
                i,
                cur_player_text,
                space,
                cur_alpha_text,
                if let Some(card) = p.hole_cards[0] { card.raw() } else { "X".to_string() },
                if let Some(card) = p.hole_cards[1] { card.raw() } else { "X".to_string() },
                if p.hole_cards[0].is_some() && let Some(h) = hand_rank_some(&p.hole_cards, &self.community_cards) {
                    format!("({}) {:?}\n", h.value(), h)
                } else {
                    "".to_string()
                },
                p.bet.separate_with_commas(),
                p.stake.separate_with_commas(),
                p.stack.separate_with_commas(),
            );
        });
    }
}

