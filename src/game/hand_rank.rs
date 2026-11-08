use crate::*;

// ——— Hand Rank ——————————————————————————————————————————————————————————————————————————————————————————————————————

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
            RoyalFlush         => 0,
        }
    }
}

pub use HandRank::*;


// ——— Algorithm ——————————————————————————————————————————————————————————————————————————————————————————————————————

fn hand_rank(cards: Vec<Card>) -> HandRank {
    let rhr = |mut cards: Vec<Card>| -> HandRank {
        cards.sort();

        let (pairs, trips, quads, straight, flush): (Vec<usize>, Vec<usize>, Vec<usize>, bool, bool) = (
            cards.windows(2).enumerate().filter(|(_, window)| window[0] == window[1]).map(|(i, _)| i).collect(),
            cards.windows(3).enumerate().filter(|(_, window)| window.windows(2).all(|w| w[0] == w[1])).map(|(i, _)| i).collect(),
            cards.windows(4).enumerate().filter(|(_, window)| window.windows(2).all(|w| w[0] == w[1])).map(|(i, _)| i).collect(),
            cards[0].rank < Jack && cards.windows(2).all(|w| w[0].rank.next() == w[1].rank) ||
            (cards.windows(2).take(3).all(|w| w[0].rank.next() == w[1].rank) && cards[0].rank == Two && cards[4].rank == Ace),        // ace low straight
            cards.windows(2).all(|w| w[0].suit == w[1].suit),
        );

        if straight && flush && cards[4].rank == Ace { RoyalFlush }
        else if straight && flush { StraightFlush(cards[4].rank) }
        else if quads.len() > 0 { FourOfAKind(cards[quads[0]].rank, cards[(quads[0] + 4) % 5].rank) }
        else if trips.len() > 0 && pairs.len() > 2 {
            let pair_rank = pairs.iter().filter(|&&i| cards[i].rank != cards[trips[0]].rank).copied().collect::<Vec<_>>();
            FullHouse(cards[trips[0]].rank, cards[pair_rank[0]].rank)
        }
        else if flush { Flush(array::from_fn(|i| cards[4 - i].rank)) }
        else if straight {
            if cards[0].rank == Two && cards[4].rank == Ace { Straight(cards[3].rank) }
            else { Straight(cards[4].rank) }
        }
        else if trips.len() > 0 { 
            let kickers: Vec<Rank> = (3..5).map(|i| cards[(trips[0] + i) % 5].rank).sorted_by(|a, b| b.cmp(a)).collect();
            ThreeOfAKind(cards[trips[0]].rank, [kickers[0], kickers[1]])
        }
        else if pairs.len() > 1 {
            let pairs_ranks: Vec<Rank> = pairs.iter().map(|&i| cards[i].rank).sorted_by(|a, b| b.cmp(a)).collect();
            TwoPair(pairs_ranks[0], pairs_ranks[1], cards[(pairs[0] + 4 + (pairs[1] - (pairs[0] + 2)) * 3) % 5].rank)
        }
        else if pairs.len() > 0 {
            let kickers: Vec<Rank> = (2..5).map(|i| cards[(pairs[0] + i) % 5].rank).sorted_by(|a, b| b.cmp(a)).collect();
            OnePair(cards[pairs[0]].rank, [kickers[0], kickers[1], kickers[2]])
        }
        else { HighCard(array::from_fn(|i| cards[4 - i].rank)) }

    };

    cards.into_iter().combinations(5)
        .map(|hand| rhr(hand))
        .sorted_by(|a, b| b.cmp(a))
        .next().unwrap()
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

