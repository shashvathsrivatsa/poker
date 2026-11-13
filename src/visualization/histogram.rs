// use crate::*;

// ——— Histogram ——————————————————————————————————————————————————————————————————————————————————————————————————————

const MAX_WIDTH: usize = 30;

pub fn histogram(data: &[f64], buckets_count: usize) {
    println!();

    let mut data = data.to_vec();
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let buckets: Vec<usize> = (0..buckets_count).map(|i| {
        let checkpoint = |i| -> f64 { data[0] + (i as f64 / buckets_count as f64) * (data.last().unwrap() - data[0]) };
        let (start, end) = (checkpoint(i), checkpoint(i+1));
        data.iter().skip_while(|&&k| k < start).take_while(|&&k| k < end || k == *data.last().unwrap()).count()
    }).collect();

    let scale = MAX_WIDTH as f64 / *buckets.iter().max().unwrap() as f64;

    buckets.iter()
        .map(|&bucket| ((bucket as f64 * scale).round() as usize, bucket))
        .for_each(|(bar_size, bucket)| println!(
            "│{}{}{}",
            "▇".repeat(bar_size),
            if bar_size == 0 { "" } else { " " },
            bucket,
        ));

    println!();
}

