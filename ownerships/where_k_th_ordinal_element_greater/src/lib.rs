#![forbid(unsafe_code)]

pub fn where_k_th_ordinal_element_greater<'a>(
    lhs: &'a Vec<i32>,
    rhs: &'a Vec<i32>,
    k: usize,
) -> &'a Vec<i32> {
    if k >= lhs.len() || k >= rhs.len() {
        return rhs;
    }

    let mut lhs_copy = lhs.clone();
    let mut rhs_copy = rhs.clone();
    lhs_copy.sort();
    rhs_copy.sort();

    if lhs_copy[k] > rhs_copy[k] {
        return lhs;
    } else {
        return rhs;
    }
}
