use crate::*;

// ——— Hand Rank ——————————————————————————————————————————————————————————————————————————————————————————————————————

fn hand_rank(cards: &[Option<Card>; 7]) -> HandRank {

    // build hand combinations
    let mut hand_combinations: [Option<[Card; 5]>; 21] = [None; 21];

    match cards.iter().filter(|card| card.is_some()).count() {

        // C(5, 5)
        5 => {
            let mut hand = [Card::default(); 5];

            cards.iter()
                .filter(|card| card.is_some())
                .map(|card| card.unwrap())
                .enumerate()
                .for_each(|(j, card)| hand[j] = card);

            hand_combinations[0] = Some(hand);
        },

        // C(6, 5)
        6 => {
            let mut hc_i = 0;

            for card_1 in cards.iter().filter(|card| card.is_some()).map(|card| card.unwrap()) {
                let mut hand = [Card::default(); 5];

                cards.iter()
                    .filter(|card| card.is_some())
                    .map(|card| card.unwrap())
                    .filter(|&card| !card.strictly_eq(&card_1))
                    .enumerate()
                    .for_each(|(j, card)| hand[j] = card);

                hand_combinations[hc_i] = Some(hand);
                hc_i += 1;
            }
        },

        // C(7, 5)
        7 => {
            let mut hc_i = 0;

            for (i, card_1) in cards.iter().enumerate().map(|(i, card)| (i, card.unwrap())) {
                for card_2 in cards.iter().skip(i+1).map(|card| card.unwrap()) {
                    let mut hand = [Card::default(); 5];

                    cards.iter()
                        .map(|card| card.unwrap())
                        .filter(|&card| !card.strictly_eq(&card_1) && !card.strictly_eq(&card_2))
                        .enumerate()
                        .for_each(|(j, card)| hand[j] = card);

                    hand_combinations[hc_i] = Some(hand);
                    hc_i += 1;
                }
            }
        },
        _ => panic!("unreachable")
    }


    // iterate over all hand combinations
    hand_combinations.iter().filter(|hand| hand.is_some()).map(|hand| {
        let mut hand = hand.unwrap();
        hand.sort();

        // residuals
        let mut rank_counts = [0; 13];
        hand.iter().for_each(|&card| rank_counts[card.rank as usize] += 1);

        let pairs_count = rank_counts.iter().filter(|&&rank_count| rank_count == 2).count();
        let trips_count = rank_counts.iter().filter(|&&rank_count| rank_count == 3).count();
        let quads_count = rank_counts.iter().filter(|&&rank_count| rank_count == 4).count();

        let straight: bool =
            (
                hand[0].rank < Jack &&
                hand.windows(2).all(|w| w[0].rank.next() == w[1].rank)
            ) ||
            (
                hand.windows(2).take(3).all(|w| w[0].rank.next() == w[1].rank) &&
                hand[0].rank == Two &&
                hand[4].rank == Ace
            );

        let flush: bool = hand.windows(2).all(|w| w[0].suit == w[1].suit);


        // (10) royal house
        if straight && flush && hand[4].rank == Ace { RoyalFlush }

        // (9) straight flush
        else if straight && flush {
            if hand[0].rank == Two && hand[4].rank == Ace { StraightFlush(hand[3].rank) }
            else { StraightFlush(hand[4].rank) }
        }

        // (8) quads
        else if quads_count == 1 {
            let quad = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 4).unwrap());
            let kicker = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 1).unwrap());
            FourOfAKind(quad, kicker)
        }

        // (7) full house
        else if pairs_count == 1 && trips_count == 1 {
            let pair = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 2).unwrap());
            let trip = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 3).unwrap());
            FullHouse(pair, trip)
        }

        // (6) flush
        else if flush { Flush(array::from_fn(|i| hand[4 - i].rank)) }

        // (5) straight
        else if straight {
            if hand[0].rank == Two && hand[4].rank == Ace { Straight(hand[3].rank) }
            else { Straight(hand[4].rank) }
        }

        // (4) trips
        else if trips_count == 1 {
            let trip = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 3).unwrap());

            let mut kickers = [Ace; 2];
            rank_counts.iter().enumerate().rev()
                .filter(|&(_, &rank_count)| rank_count == 1)
                .enumerate()
                .for_each(|(i, (rank, _))| { kickers[i] = Rank::from(rank); });

            ThreeOfAKind(trip, kickers)
        }

        // (3) two pair
        else if pairs_count == 2 {
            let mut pairs = [Ace; 2];
            rank_counts.iter().enumerate().rev()
                .filter(|&(_, &rank_count)| rank_count == 2)
                .enumerate()
                .for_each(|(i, (rank, _))| { pairs[i] = Rank::from(rank); });


            let kicker = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 1).unwrap());

            TwoPair(pairs[0], pairs[1], kicker)
        }

        // (2) one pair
        else if pairs_count == 1 {
            let pair = Rank::from(rank_counts.iter().position(|&rank_count| rank_count == 2).unwrap());

            let mut kickers = [Ace; 3];
            rank_counts.iter().enumerate().rev()
                .filter(|&(_, &rank_count)| rank_count == 1)
                .enumerate()
                .for_each(|(i, (rank, _))| { kickers[i] = Rank::from(rank); });

            OnePair(pair, kickers)
        }

        // (1) high card
        else { HighCard(array::from_fn(|i| hand[4 - i].rank)) }

    }).max().unwrap()
}

// only some community cards known
pub fn hand_rank_some(hole_cards: &[Option<Card>; 2], community_cards: &[Option<Card>; 5]) -> Option<HandRank> {
    if community_cards.iter().filter(|c| c.is_some()).count() == 0 { return None; }

    let mut cards: [Option<Card>; 7] = [None; 7];

    community_cards.iter()
        .filter(|card| card.is_some())
        .map(|card| card.unwrap())
        .chain(hole_cards.iter().map(|card| card.unwrap()))
        .enumerate()
        .for_each(|(i, card)| cards[i] = Some(card));

    Some(hand_rank(&cards))
}

// all community cards known
pub fn hand_rank_all(hole_cards: &[Card; 2], community_cards: &[Card; 5]) -> HandRank {
    let mut cards: [Option<Card>; 7] = [None; 7];

    community_cards.iter()
        .chain(hole_cards.iter())
        .enumerate()
        .for_each(|(i, &card)| cards[i] = Some(card));

    hand_rank(&cards)
}


// ——— Enum ———————————————————————————————————————————————————————————————————————————————————————————————————————————

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[allow(dead_code)]
pub enum HandRank {
    HighCard([Rank; 5]),            // top 5 high cards
    OnePair(Rank, [Rank; 3]),       // pair rank + 3 kickers
    TwoPair(Rank, Rank, Rank),      // high pair + low pair + kicker
    ThreeOfAKind(Rank, [Rank; 2]),  // trips rank + 2 kickers
    Straight(Rank),                 // highest card
    Flush([Rank; 5]),               // all ranks
    FullHouse(Rank, Rank),          // trips rank, pair rank
    FourOfAKind(Rank, Rank),        // quads rank, kicker
    StraightFlush(Rank),            // highest card
    RoyalFlush,                     // nothing to be said here
}

impl HandRank {
    pub fn value(&self) -> usize {
        match self {
            HighCard(_)        => 1,
            OnePair(_, _)      => 2,
            TwoPair(_, _, _)   => 3,
            ThreeOfAKind(_, _) => 4,
            Straight(_)        => 5,
            Flush(_)           => 6,
            FullHouse(_, _)    => 7,
            FourOfAKind(_, _)  => 8,
            StraightFlush(_)   => 9,
            RoyalFlush         => 10,
        }
    }
}

pub use HandRank::*;

