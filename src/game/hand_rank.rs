use crate::*;

// ——— Hand Rank ——————————————————————————————————————————————————————————————————————————————————————————————————————

fn hand_rank(cards: Vec<Card>) -> HandRank {
    cards.into_iter().combinations(5).map(|mut hand| {
        hand.sort();

        // residuals
        let mut pairs = [0; 5];
        hand.windows(2).enumerate()
            .filter(|(_, window)| window[0] == window[1])
            .for_each(|(i, _)| pairs[i] = 1);

        let mut trips = [0; 5];
        hand.windows(3).enumerate()
            .filter(|(_, window)| window.windows(2).all(|w| w[0] == w[1]))
            .for_each(|(i, _)| trips[i] = 1);

        let mut quads = [0; 5];
        hand.windows(4).enumerate()
            .filter(|(_, window)| window.windows(2).all(|w| w[0] == w[1]))
            .for_each(|(i, _)| quads[i] = 1);

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
        else if straight && flush { StraightFlush(hand[4].rank) }

        // (8) quads
        else if quads.len() > 0 { FourOfAKind(hand[quads[0]].rank, hand[(quads[0] + 4) % 5].rank) }

        // (7) full house
        else if trips.len() > 0 && pairs.len() > 2 {
            let pair_rank = pairs.iter().filter(|&&i| hand[i].rank != hand[trips[0]].rank).copied().collect::<Vec<_>>();
            FullHouse(hand[trips[0]].rank, hand[pair_rank[0]].rank)
        }

        // (6) flush
        else if flush { Flush(array::from_fn(|i| hand[4 - i].rank)) }

        // (5) straight
        else if straight {
            if hand[0].rank == Two && hand[4].rank == Ace { Straight(hand[3].rank) }
            else { Straight(hand[4].rank) }
        }

        // (4) trips
        else if trips.len() > 0 { 
            let kickers: Vec<Rank> = (3..5).map(|i| hand[(trips[0] + i) % 5].rank).sorted_by(|a, b| b.cmp(a)).collect();
            ThreeOfAKind(hand[trips[0]].rank, [kickers[0], kickers[1]])
        }

        // (3) two pair
        else if pairs.iter().filter(|&&k| k == 1).count() > 1 {
            // let pairs_indices = [Rank; 2];
            pairs.iter().enumerate()
                .filter(|&(_, &k)| k == 1)
                .map(|(kicker_i, _)| hand[kicker_i].rank)
                .;

            let pairs_ranks: Vec<Rank> = pairs.iter().map(|&i| hand[i].rank).sorted_by(|a, b| b.cmp(a)).collect();
            TwoPair(pairs_ranks[0], pairs_ranks[1], hand[(pairs[0] + 4 + (pairs[1] - (pairs[0] + 2)) * 3) % 5].rank)
        }

        // (2) one pair
        else if pairs.iter().any(|&k| k == 1) {
            let pair_index = pairs.iter().position(|&k| k == 1).unwrap();
            let mut kickers = [0; 3];

            (2..5).enumerate()
                .map(|(i, kicker_i)| (i, hand[(pair_index + kicker_i) % 5].rank))
                .sorted_by(|(_, a), (_, b)| b.cmp(a))
                .for_each(|(i, _)| kickers[i] = 1);

            OnePair(hand[pair_index].rank, [hand[kickers[0]].rank, hand[kickers[1]].rank, hand[kickers[2]].rank])
        }

        // (1) high card
        else { HighCard(array::from_fn(|i| hand[4 - i].rank)) }

    }).max().unwrap()
}

// only some community cards known
pub fn hand_rank_some(hole_cards: &[Option<Card>; 2], community_cards: &[Option<Card>; 5]) -> Option<HandRank> {
    if community_cards.iter().filter(|c| c.is_some()).count() == 0 { return None; }

    let cards: Vec<Card> = community_cards.iter()
        .filter(|card| card.is_some())
        .map(|card| card.unwrap())
        .chain(hole_cards.iter().map(|card| card.unwrap()))
        .collect();

    Some(hand_rank(cards))
}

// all community cards known
pub fn hand_rank_all(hole_cards: &[Card; 2], community_cards: &[Card; 5]) -> HandRank {
    let cards: Vec<Card> = community_cards.iter()
        .chain(hole_cards.iter())
        .copied().collect();

    hand_rank(cards)
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

