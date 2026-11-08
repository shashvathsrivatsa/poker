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

    let fhs = fhs(&state.players[0].hole_cards, &state.community_cards);
    println!("{}", fhs.percent());
}

