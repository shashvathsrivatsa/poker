use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::create(2);
    state.start();

    state.turn(Call);
    state.turn(Call);

    state.turn(Call);
    state.turn(Call);

    state.turn(Call);
    state.turn(Call);

    state.turn(Call);
    state.turn(Call);

    state.table();
}

