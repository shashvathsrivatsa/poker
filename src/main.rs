use poker_bot::*;

// ——— Main ———————————————————————————————————————————————————————————————————————————————————————————————————————————

fn main() {
    let mut state = State::new(5);
    state.step(Call);
    state.step(Call);
    state.step(Call);
    state.step(Bet(30));
    state.step(Call);
    state.step(Call);
    state.step(Fold);
    state.step(Call);
    state.table();
}
