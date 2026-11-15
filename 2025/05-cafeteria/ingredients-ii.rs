use std::cmp;
use std::io;
use std::ops::RangeInclusive;

type Range = RangeInclusive<u64>;

fn merge_ranges(r1: &Range, r2: &Range) -> Option<Range> {
    if r1.contains(r2.start()) || r2.contains(r1.start()) {
        let min = cmp::min(*r1.start(), *r2.start());
        let max = cmp::max(*r1.end(), *r2.end());
        Some(min..=max)
    } else {
        None
    }
}

fn compact_ranges(ranges: &mut [Range]) -> Vec<Range> {
    let mut compacted = Vec::new();
    ranges.sort_unstable_by_key(|r| (*r.start(), *r.end()));

    let mut current = ranges[0].clone();
    for range in &ranges[1..] {
        if let Some(r) = merge_ranges(&current, range) {
            current = r;
        } else {
            compacted.push(current);
            current = range.clone();
        }
    }

    compacted.push(current);
    compacted
}

fn sum_ranges_len(ranges: &[Range]) -> u64 {
    ranges.iter().map(|r| r.end() - r.start() + 1).sum()
}

fn main() {
    let mut lines = io::stdin().lines();

    let mut ranges: Vec<_> = lines
        .by_ref()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .filter_map(|line| {
            let (start, end) = line.split_once('-')?;
            let start = start.parse::<u64>().ok()?;
            let end = end.parse::<u64>().ok()?;
            Some(start..=end)
        })
        .collect();

    println!("{}", sum_ranges_len(&compact_ranges(&mut ranges)));
}
