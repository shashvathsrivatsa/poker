use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::create(2);
    state.start();

    [
        Call,
        Call,

    ].into_iter().for_each(|action| state.turn(action));

    state.table();

    let hole_cards = [
        Card::new(Ace, Clubs),
        Card::new(Queen, Clubs),
    ];

    let community_cards = [
        Card::new(Jack, Clubs),
        Card::new(Ten, Clubs),
        Card::new(Jack, Spades),
        Card::new(King, Clubs),
        Card::new(Ace, Diamonds),
    ];

    let hs = hs(&state.players[0].hole_cards, &state.community_cards);
    println!("{}", hs.percent());
}

