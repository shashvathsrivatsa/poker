use crate::*;

// ——— Hand Strength ——————————————————————————————————————————————————————————————————————————————————————————————————

pub fn hand_strength(your_cards: &[Option<Card>; 2], community: &[Option<Card>; 5]) -> f64 {
    let your_cards: [Card; 2] = [your_cards[0].unwrap(), your_cards[1].unwrap()];
    let community_count = community.iter().filter(|c| c.is_some()).count();
    let deck: Vec<Card> = Deck::new().draw_pile;

    // Enumerate all community outcomes
    let all_community: Vec<[Card; 5]> = deck.iter()
        .filter(|c| your_cards.iter().all(|o| !c.strictly_eq(&o)))
        .filter(|c| community.iter().filter(|c| c.is_some()).all(|o| !c.strictly_eq(&o.unwrap())))
        .permutations(5 - community_count)
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

    // Enumerate all outcomes
    let all_outcomes = all_community.par_iter().map(|&community_pot| {
        let all_opp_cards = deck.iter()
            .filter(|c| your_cards.iter().all(|o| !c.strictly_eq(&o)))
            .filter(move |c| community_pot.iter().all(|o| !c.strictly_eq(o)))
            .permutations(2)
            .map(|opp_cards_pot| [ opp_cards_pot[0].clone(), opp_cards_pot[1].clone() ]);

        (community_pot, all_opp_cards)
    });

    let total_count = all_outcomes.clone().map(|(_, all_opp_cards)| all_opp_cards.count()).sum::<usize>();
    println!("Total possible outcomes: {}", total_count.separate_with_commas());

    // Remove redundant outcomes
    let mut indexer: hand_indexer_t = unsafe { std::mem::zeroed() };
    unsafe { hand_indexer_init(3, [2u8, 5, 2].as_ptr(), &mut indexer); }

    let indexer_size = unsafe { hand_indexer_size(&indexer, 2) } as usize;
    println!("yo");
    let outcome_count = std::sync::Mutex::new(bitvec![0u8; indexer_size]);
    println!("yo");

    let encode = |community_pot: &[Card; 5], opp_cards_pot: &[Card; 2]| -> usize {
        let cards: [u8; 9] = [
            your_cards[0], your_cards[1],
            community_pot[0], community_pot[1], community_pot[2], community_pot[3], community_pot[4],
            opp_cards_pot[0], opp_cards_pot[1],
        ].map(|c| (c.rank as u8) * 4 + (c.suit as u8));

        unsafe { hand_index_last(&indexer, cards.as_ptr()) as usize }
    };

    let all_outcomes_filtered = all_outcomes.map(|(community_pot, all_opp_cards)| {
        let all_opp_cards = all_opp_cards.filter(|opp_cards_pot| {
            let index = encode(&community_pot, opp_cards_pot);
            let mut oc = outcome_count.lock().unwrap();

            match oc[index] {
                false => { oc.set(index, true); true },
                true => false,
            }
        }).collect::<Vec<_>>();

        (community_pot, all_opp_cards)
    }).filter(|(_, all_opp_cards)| all_opp_cards.len() > 0);

    unsafe {
        let raw: *const _ = &all_outcomes_filtered;
        let clone = std::ptr::read(raw);
        let filtered_count = clone.map(|(_, all_opp_cards)| all_opp_cards.len()).sum::<usize>();
        println!("Total filtered outcomes: {}", filtered_count.separate_with_commas());
        println!("Shrinkage: {:.2}x", (total_count as f64 / filtered_count as f64));
        outcome_count.lock().unwrap().fill(false);
    }

    // Count how many outcomes you win
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

        (win_rate, all_opp_cards.len())

    }).collect::<Vec<_>>();

    histogram(&wins.iter().map(|&(x, _)| x).collect::<Vec<_>>(), 15);
    unsafe { hand_indexer_free(&mut indexer); }
    wins.iter().map(|&(x, y)| x * y as f64).sum::<f64>() / wins.iter().map(|&(_, x)| x).sum::<usize>() as f64
}

