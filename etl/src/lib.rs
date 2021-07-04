use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    let mut tree: BTreeMap<char, i32> = BTreeMap::new();

    for n in h {
        for letter in n.1 {
            tree.insert(
                *letter.to_lowercase().collect::<Vec<_>>().first().unwrap(),
                *n.0,
            );
        }
    }

    return tree;
}
