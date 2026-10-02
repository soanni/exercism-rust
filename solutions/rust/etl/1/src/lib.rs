use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut new_t: BTreeMap<char, i32> = BTreeMap::new();
    for (k, chars) in h {
        for ch in chars {
            new_t.entry((*ch).to_ascii_lowercase()).or_insert(*k);
        }
    }
    new_t
}
