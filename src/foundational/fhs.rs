use crate::*;

// ——— Full Hand Strength —————————————————————————————————————————————————————————————————————————————————————————————

// NOTE: could add folded cards to incorporate more information
pub fn fhs(hole_cards: &[Option<Card>; 2], community_cards: &[Option<Card>; 5]) -> f64 {

    // find remaining cards (assuming fresh deck)
    let cards_unknown: Vec<Card> = Deck::new().draw_pile.into_iter().filter(|&c| {
        hole_cards.iter().chain(community_cards.iter().filter(|c| c.is_some())).all(|d| !c.strictly_eq(&d.unwrap()))
    }).collect();

    // find every combination of cards your opponent could have
    let all_opp_hole_cards: Vec<Vec<Card>> = cards_unknown.clone().into_iter()
        .combinations(2)
        .collect();

    // add all future hand cards possibilities
    let all_outcomes_pot: Vec<([Card; 2], [Card; 5])> = all_opp_hole_cards.iter().flat_map(|opp_hole_cards_pot| {
        let all_community_cards_rem_pot = cards_unknown.iter()
            .filter(|c| opp_hole_cards_pot.iter().all(|o| !c.strictly_eq(o)))
            .combinations(community_cards.iter().filter(|card| card.is_none()).count());

        all_community_cards_rem_pot.map(move |future_community_cards_rem| {
            let mut community_cards_pot = [Card::default(); 5];

            community_cards.iter()
                .filter(|c| c.is_some()).map(|c| c.unwrap())
                .chain(future_community_cards_rem.iter().map(|&&c| c))
                .enumerate()
                .for_each(|(i, c)| community_cards_pot[i] = c);

            ([opp_hole_cards_pot[0], opp_hole_cards_pot[1]], community_cards_pot)
        })
    }).collect();

    // for each combination, count how many times you win
    let wins: f64 = all_outcomes_pot.par_iter().map(|(opp_hole_cards, future_community_cards)| {
        let your_rank = hand_rank_all(&hole_cards.map(|c| c.unwrap()), future_community_cards);
        let opp_rank = hand_rank_all(opp_hole_cards, future_community_cards);

        match opp_rank.cmp(&your_rank) {
            Ordering::Less => 1.0,
            Ordering::Equal => 0.5,
            Ordering::Greater => 0.0,
        }
    }).sum();

    // fhs = wins / total
    // println!("{} / {}", wins.separate_with_commas(), all_outcomes_pot.len().separate_with_commas());
    wins / (all_outcomes_pot.len() as f64)
}

