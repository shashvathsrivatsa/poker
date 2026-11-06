use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::create(2);
    state.start();

    state.turn(Call);
    state.turn(Call);

    state.table();

    let community_cards = vec![
        Some(Card::new(Three, Hearts)),
        Some(Card::new(Ace, Hearts)),
        Some(Card::new(Two, Hearts)),
        None,
        None,
    ];

    let hole_cards = vec![
        Some(Card::new(Four, Hearts)),
        Some(Card::new(Five, Spades)),
    ];

    println!("{:?}, {:?}", hole_cards, community_cards);
    let hs = hs(&hole_cards, &community_cards);
    println!("{hs:?}");
}

