use crate::*;

// ——— Full Hand Strength —————————————————————————————————————————————————————————————————————————————————————————————

pub fn fhs(your_cards: &[Option<Card>; 2], community: &[Option<Card>; 5]) -> f64 {
    let your_cards: [Card; 2] = [your_cards[0].unwrap(), your_cards[1].unwrap()];
    let community_count = community.iter().filter(|c| c.is_some()).count();
    let deck: Vec<Card> = Deck::new().draw_pile;

    // enumerate all community outcomes
    let all_community: Vec<[Card; 5]> = deck.iter()
        .filter(|c| your_cards.iter().all(|o| !c.strictly_eq(&o)))
        .filter(|c| community.iter().filter(|c| c.is_some()).all(|o| !c.strictly_eq(&o.unwrap())))
        .combinations(5 - community_count)
        .map(|community_rem_pot| {
            let mut community_pot = [Card::default(); 5];

            community.iter()
                .filter(|c| c.is_some()).map(|c| c.unwrap())
                .chain(community_rem_pot.iter().map(|&&c| c))
                .enumerate()
                .for_each(|(i, c)| community_pot[i] = c);

            community_pot
        })
    .collect();

    // enumerate all outcomes
    let all_outcomes = all_community.iter().map(|&community_pot| {
        let all_opp_cards = deck.iter()
            .filter(|c| your_cards.iter().all(|o| !c.strictly_eq(&o)))
            .filter(move |c| community_pot.iter().all(|o| !c.strictly_eq(o)))
            .combinations(2)
            .map(|opp_cards_pot| [ opp_cards_pot[0].clone(), opp_cards_pot[1].clone() ]);

        (community_pot, all_opp_cards)
    });

    println!(
        "Total possible outcomes: {}",
        all_outcomes.clone().map(|(_, all_opp_cards)| all_opp_cards.count()).sum::<usize>().separate_with_commas(),
    );

    // remove redundant outcomes
    let mut outcome_count = bitvec!(0; 62_748_517);    // 13^7

    let encode = |community_pot: &[Card; 5], opp_cards_pot: &[Card; 2]| -> usize {
        community_pot.iter().chain(opp_cards_pot.iter()).enumerate()
            .map(|(i, c)| c.rank as usize * 13_usize.pow(i as u32))
            .sum()
    };

    let all_outcomes_filtered = all_outcomes.map(|(community_pot, all_opp_cards)| {
        let all_opp_cards = all_opp_cards.filter(|opp_cards_pot| {
            let index = encode(&community_pot, opp_cards_pot);

            match outcome_count[index] {
                false => {
                    outcome_count.set(index, true);
                    true
                },
                true => false
            };

            true
        }).collect::<Vec<_>>();

        (community_pot, all_opp_cards)
    }).filter(|(_, all_opp_cards)| all_opp_cards.len() > 0);

    unsafe {
        let raw: *const _ = &all_outcomes_filtered;
        let clone = std::ptr::read(raw);
        let count = clone.map(|(_, all_opp_cards)| all_opp_cards.iter().count()).sum::<usize>();
        println!("Total filtered outcomes: {}", count.separate_with_commas());
    }

    // count how many outcomes you win
    let wins = all_outcomes_filtered.map(|(community_pot, all_opp_cards)| {
        let win_rate = all_opp_cards.iter().fold(0.0, |sum, opp_cards_pot| {
            let opp_rank = hand_rank_all(&opp_cards_pot, &community_pot);
            let your_rank = hand_rank_all(&your_cards, &community_pot);
            sum + match opp_rank.cmp(&your_rank) {
                Ordering::Less => 1.0,
                Ordering::Equal => 0.5,
                Ordering::Greater => 0.0,
            }
        }) / all_opp_cards.len() as f64;

        win_rate

    }).collect::<Vec<_>>();

    histogram(&wins, 15);

    wins.iter().sum::<f64>() / wins.len() as f64
}

