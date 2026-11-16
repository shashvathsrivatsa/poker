use crate::*;

// ——— Equity —————————————————————————————————————————————————————————————————————————————————————————————————————————

pub fn equity(your_cards: &[Option<Card>; 2], community: &[Option<Card>; 5]) -> f64 {
    let your_cards: [Card; 2] = [your_cards[0].unwrap(), your_cards[1].unwrap()];
    let community_count = community.iter().filter(|c| c.is_some()).count();
    let deck: Vec<Card> = Deck::new().draw_pile;

    // Enumerate all community outcomes
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

    // Enumerate all outcomes
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

    // Remove redundant outcomes
    let mut outcome_count = bitvec!(0; 392_705_170);

    let encode = |community_pot: &[Card; 5], opp_cards_pot: &[Card; 2]| -> usize {
        // let cards: Vec<Card> = community_pot.iter()
        //     .chain(opp_cards_pot.iter())
        //     .chain(your_cards.iter())
        //     .cloned().collect();

        // Build suits count
        let mut suit_counts = [0; 4];

        community_pot.iter()
            .chain(opp_cards_pot.iter())
            .chain(your_cards.iter())
            .for_each(|c| { suit_counts[c.suit as usize] += 1; });

        // Sort suits count
        let suits_sorted: Vec<usize> = suit_counts.into_iter().enumerate()
            .sorted_by(|(i, a), (j, b)| b.cmp(a).then(i.cmp(j)))
            .map(|(i, _)| i)
            .collect();

        // Build suit groups
        let mut your_groups: [Vec<Rank>; 4] = std::array::from_fn(|_| Vec::new());
        let mut comm_groups: [Vec<Rank>; 4] = std::array::from_fn(|_| Vec::new());
        let mut opp_groups: [Vec<Rank>; 4] = std::array::from_fn(|_| Vec::new());

        let relabel_suit = |card: Card| -> usize { suits_sorted.iter().position(|&suit| (card.suit as usize) == suit).unwrap() };

        your_cards.iter().for_each(|&card| { your_groups[relabel_suit(card)].push(card.rank); });
        community_pot.iter().for_each(|&card| { comm_groups[relabel_suit(card)].push(card.rank); });
        opp_cards_pot.iter().for_each(|&card| { opp_groups[relabel_suit(card)].push(card.rank); });

        your_groups.iter_mut()
            .chain(comm_groups.iter_mut())
            .chain(opp_groups.iter_mut())
            .for_each(|group| { group.sort_by(|a, b| b.cmp(a)); });

        // Find individual indices (colex)
        let mut your_indices = [0_usize; 4];
        let mut comm_indices = [0_usize; 4];
        let mut opp_indices = [0_usize; 4];

        let colex_index = |group: &[Rank]| -> usize {
            group.iter().enumerate().fold(0, |sum, (j, &rank)| sum + choose(rank as usize, j + 1))
        };

        your_groups.iter().enumerate().for_each(|(i, group)| { your_indices[i] = colex_index(group); });
        comm_groups.iter().enumerate().for_each(|(i, group)| { comm_indices[i] = colex_index(group); });
        opp_groups.iter().enumerate().for_each(|(i, group)| { opp_indices[i] = colex_index(group); });

        // Combine to overall local index (mixed-radix formula)
        let local_index = |indices: [usize; 4], groups: &[Vec<Rank>; 4]| -> usize {
            indices.iter()
                .zip(groups.iter().map(|group| group.len()))
                .rev().skip(1)
                .fold(*indices.last().unwrap(), |acc, (c_i, n_i)| {
                    c_i + choose(13, n_i) * acc
                })
        };

        let your_index = local_index(your_indices, &your_groups);
        let comm_index = local_index(comm_indices, &comm_groups);
        let opp_index = local_index(opp_indices, &opp_groups);

        // Add partition offset to get global index
        let your_offset = match your_groups.map(|g| g.len()) {
            [2,0,0,0] => 0,
            [1,1,0,0] => 78,
            _ => unreachable!()
        };

        let comm_offset = match comm_groups.map(|g| g.len()) {
            [5,0,0,0] => 0,
            [4,1,0,0] => 1_287,
            [3,2,0,0] => 10_582,
            [3,1,1,0] => 32_890,
            [2,2,1,0] => 81_224,
            [2,1,1,1] => 160_316,
            _ => unreachable!()
        };

        let opp_offset = match opp_groups.map(|g| g.len()) {
            [2,0,0,0] => 0,
            [1,1,0,0] => 78,
            _ => unreachable!()
        };

        todo!();
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
            }
        }).collect::<Vec<_>>();

        (community_pot, all_opp_cards)
    }).filter(|(_, all_opp_cards)| all_opp_cards.len() > 0);

    unsafe {
        let raw: *const _ = &all_outcomes_filtered;
        let clone = std::ptr::read(raw);
        let count = clone.map(|(_, all_opp_cards)| all_opp_cards.iter().count()).sum::<usize>();
        println!("Total filtered outcomes: {}", count.separate_with_commas());
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

        win_rate

    }).collect::<Vec<_>>();

    histogram(&wins, 15);

    wins.iter().sum::<f64>() / wins.len() as f64
}

