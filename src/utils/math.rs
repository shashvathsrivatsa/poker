// use crate::*;

// ——— Math ———————————————————————————————————————————————————————————————————————————————————————————————————————————

pub fn choose(n: usize, k: usize) -> usize {
    if k > n { return 0; }
    let k = k.min(n - k);
    (0..k).fold(1, |acc, i| acc * (n - i) / (i + 1))
}

