use std::collections::HashMap;
use std::hash::Hash;

/// TODO: Make it more fuzzy and just better. This will just
/// take the value, which is most often
pub fn most_common<T: Hash + Eq + Clone>(values: impl IntoIterator<Item = T>) -> Option<T> {
    let mut counts: HashMap<T, usize> = HashMap::new();
    let mut order: Vec<T> = Vec::new();
    for v in values {
        let c = counts.entry(v.clone()).or_insert(0);
        if *c == 0 {
            order.push(v);
        }
        *c += 1;
    }
    let mut best: Option<(T, usize)> = None;
    for v in order {
        let n = counts[&v];
        if best.as_ref().map_or(true, |(_, bn)| n > *bn) {
            best = Some((v, n));
        }
    }
    best.map(|(v, _)| v)
}
