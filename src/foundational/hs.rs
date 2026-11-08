use crate::*;

// ——— HS —————————————————————————————————————————————————————————————————————————————————————————————————————————————

// NOTE: could add folded cards to incorporate more information
pub fn hs(hole_cards: &[Option<Card>; 2], community_cards: &[Option<Card>; 5]) -> f64 {

    // find remaining cards (assuming fresh deck)
    let mut cards_seen: Vec<Card> = Vec::with_capacity(52);
    hole_cards.iter().for_each(|&c| cards_seen.push(c.unwrap()));
    community_cards.iter().filter(|c| c.is_some()).for_each(|&c| cards_seen.push(c.unwrap()));
    let num_of_known_cards = cards_seen.len();

    let mut deck_temp = Deck::new();
    cards_seen.iter().for_each(|&c| { deck_temp.pick_out(c); });
    let cards_unknown: Vec<Card> = deck_temp.draw_pile;

    // find every combination of cards your opponent could have
    let opp_hole_cards: Vec<Vec<Card>> = cards_unknown.clone().into_iter()
        .combinations(2)
        .collect();

    // add all future hand cards possibilities
    let all_future_posibilities: Vec<([Card; 2], [Card; 5])> = match community_cards.iter().filter(|c| c.is_some()).count() {

        // Turn & River
        // C(47, 2) * C(45, 1) * C(44, 1)
        3 => {
            let mut x: Vec<([Card; 2], [Card; 5])> = Vec::with_capacity(opp_hole_cards.len() * 45 * 44);

            for opp_hole_cards in opp_hole_cards.iter() {
                let cards_unknown_iter = cards_unknown.iter().filter(|&c| {
                    opp_hole_cards.iter().all(|o| !c.strictly_eq(o))
                });

                for &turn in cards_unknown_iter {
                    let cards_unknown_iter = cards_unknown.iter().filter(|&c| {
                        opp_hole_cards.iter().chain(std::iter::once(&turn)).all(|o| !c.strictly_eq(o))
                    });

                    for &river in cards_unknown_iter {
                        x.push((
                            [ opp_hole_cards[0], opp_hole_cards[1] ],
                            [
                                community_cards[0].unwrap(),
                                community_cards[1].unwrap(),
                                community_cards[2].unwrap(),
                                turn,
                                river,
                            ]
                        ));
                    }
                }
            }

            x
        },

        // River
        // C(47, 2) * C(45, 1)
        4 => {
            let mut x: Vec<([Card; 2], [Card; 5])> = Vec::with_capacity(opp_hole_cards.len() * 45);

            for opp_hole_cards in opp_hole_cards.iter() {
                let cards_unknown_iter = cards_unknown.iter().filter(|&c| {
                    opp_hole_cards.iter().all(|o| !c.strictly_eq(o))
                });

                for &river in cards_unknown_iter {
                    x.push((
                        [ opp_hole_cards[0], opp_hole_cards[1] ],
                        [
                            community_cards[0].unwrap(),
                            community_cards[1].unwrap(),
                            community_cards[2].unwrap(),
                            community_cards[3].unwrap(),
                            river,
                        ]
                    ))
                }
            }

            x
        },

        // None
        // C(47, 2)
        5 => {
            opp_hole_cards.iter().map(|opp_hole_cards| (
                [ opp_hole_cards[0], opp_hole_cards[1] ],
                [
                    community_cards[0].unwrap(),
                    community_cards[1].unwrap(),
                    community_cards[2].unwrap(),
                    community_cards[3].unwrap(),
                    community_cards[4].unwrap(),
                ]
            )).collect()
        },

        // Flop & Turn & River
        _ => panic!("not handling preflop")
    };


    // for each combination, calculate their hand rank
    println!("Total future possibilities: {}", all_future_posibilities.len().separate_with_commas());

    let opp_ranks: Vec<HandRank> = all_future_posibilities.iter()
        .map(|(opp_hole_cards, future_community_cards)| hand_rank_all(opp_hole_cards, future_community_cards))
        .collect();

    // count how many times you win
    let cur_rank = hand_rank_some(hole_cards, community_cards).unwrap();

    let wins = opp_ranks.iter().fold(0, |sum, &opp_rank| {
        if cur_rank > opp_rank { sum + 1 } else { sum }
    });

    // hs = wins / total
    wins as f64 / opp_ranks.len() as f64
}

