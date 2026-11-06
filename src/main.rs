use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::create(2);
    state.start();

    state.turn(Call);
    state.turn(Call);

    state.turn(Raise(20));
    state.turn(Call);

    state.turn(Raise(30));
    state.turn(Call);

    state.turn(Call);
    state.turn(Raise(60));
    state.turn(Call);

    state.table();
}

