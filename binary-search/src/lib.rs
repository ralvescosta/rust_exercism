use std::cmp::Ordering;

pub fn find<T, R>(array: R, key: T) -> Option<usize>
where
    T: Ord,
    R: AsRef<[T]>,
{
    let mut mid: usize = 0;
    let mut array = array.as_ref();
    loop {
        if array.is_empty() {
            return None;
        }
        let index = array.len() / 2;
        mid += index;
        match (key.cmp(&array[index]), index) {
            (Ordering::Less, _) => {
                mid -= index;
                array = &array[..index];
            }
            (Ordering::Equal, _) => break Some(mid),
            (_, 0) => break None,
            (Ordering::Greater, _) => array = &array[index..],
        }
    }
}
