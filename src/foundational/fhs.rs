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

    // remove redundant outcomes
    let mut outcome_count = bitvec!(0; 62_748_517);    // 13^7

    let all_filtered_outcomes = all_outcomes.clone().map(|(community_pot, all_opp_cards)| {        // remove clone later
        // let index: usize = community_pot.iter().enumerate().map(|(i, c)| c.rank as usize * 13_usize.pow(i as u32)).sum();

        let all_opp_cards = all_opp_cards.filter(|opp_cards_pot| {
            let index: usize = opp_cards_pot.iter().enumerate().map(|(i, c)| c.rank as usize * 13_usize.pow(i as u32)).sum();

            // match outcome_count[0] {
            //     false => {
            //         outcome_count.set(0, true);
            //         true
            //     },
            //     true => false
            // }
            true
        });

        (community_pot, all_opp_cards)
    })
    // .filter()
    ;

    let mut i = 0;
    let all_filtered_outcomes = all_outcomes.clone().for_each(|(community_pot, all_opp_cards)| {
        let all_opp_cards = all_opp_cards.filter(|opp_cards_pot| {
            outcome_count.set(0, true);
            let index: usize = community_pot.iter().chain(opp_cards_pot.iter()).enumerate()
                .map(|(i, c)| c.rank as usize * 13_usize.pow(i as u32))
                .sum();

            match outcome_count[index] {
                false => {
                    outcome_count.set(index, true);
                    true
                },
                true => false
            }
        });

        // (community_pot, all_opp_cards)
        i += 1;
    });

    println!("{i}");

    // compare
    // println!(
    //     "Total possible outcomes: {}",
    //     all_outcomes.clone().map(|(_, all_opp_cards)| all_opp_cards.count()).sum::<usize>().separate_with_commas(),
    // );
    // println!(
    //     "Total filtered outcomes: {}",
    //     all_filtered_outcomes.map(|(_, all_opp_cards)| all_opp_cards.len()).sum::<usize>().separate_with_commas(),
    // );


    // count how many outcomes you win
    let wins: Vec<f64> = all_outcomes.map(|(community_pot, all_opp_cards)| {
        all_opp_cards.fold(0.0, |sum, opp_cards_pot| {
            let opp_rank = hand_rank_all(&opp_cards_pot, &community_pot);
            let your_rank = hand_rank_all(&your_cards, &community_pot);
            sum + match opp_rank.cmp(&your_rank) {
                Ordering::Less => 1.0,
                Ordering::Equal => 0.5,
                Ordering::Greater => 0.0,
            }
        }) / 990.0
    }).collect();

    wins.iter().sum::<f64>() / wins.len() as f64
}

