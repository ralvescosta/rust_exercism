use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut tree: BTreeMap<char, i32> = BTreeMap::new();

    for (&point, vec) in h {
        for &letter in vec {
            tree.insert(letter.to_ascii_lowercase(), point);
        }
    }
    return tree;
}
