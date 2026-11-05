use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::create(5);
    state.start();

    state.turn(Call);
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);

    state.turn(Call);
    state.turn(Raise(20));
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);

    state.turn(Raise(20));
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);

    state.turn(Raise(20));
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);
    state.turn(Raise(40));
    state.turn(Call);
    state.turn(Call);
    state.turn(Call);

    state.table();
}
