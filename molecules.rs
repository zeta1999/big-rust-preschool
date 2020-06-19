mod sumth_element;

use std::cmp::Ordering;
use std::iter::Sum;
use sumth_element::sumth_element;

#[derive(Eq)]
struct WeightIndex {
    pub w: u32,
    pub i: u32,
}

impl Ord for WeightIndex {
    fn cmp(&self, other: &Self) -> Ordering {
        self.w.cmp(&other.w)
    }
}

impl PartialOrd for WeightIndex {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for WeightIndex {
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w
    }
}

impl<'a> Sum<&'a WeightIndex> for u64 {
    fn sum<I>(iter: I) -> u64
    where
        I: Iterator<Item = &'a WeightIndex>,
    {
        iter.fold(0u64, |sum, wi| sum + wi.w as u64)
    }
}

pub fn find_subset(l: u32, u: u32, w: &[u32]) -> Vec<u32> {
    let l = l as u64;
    let u = u as u64;
    let mut wi: Vec<_> = w
        .iter()
        .enumerate()
        .map(|(i, &w)| WeightIndex { w, i: i as u32 })
        .collect();

    let (i, slack) = sumth_element(&mut wi, l - 1);
    if i == wi.len() {
        return vec![];
    }
    order_stat::kth(&mut wi[i..], 0);

    let sum = l - 1 - slack;
    if sum + wi[i].w as u64 <= u {
        return wi[..=i].iter().map(|wi| wi.i).collect();
    }

    if i + i + 1 < w.len() && i > 0 {
        order_stat::kth(&mut wi[i + 1..], w.len() - (i + i + 1));
    }

    let mut sum = sum;
    let mut j = 0;
    let mut k = wi.len() - 1;
    while sum < l && j < i {
        sum += (wi[k].w - wi[j].w) as u64;
        wi.swap(j, k);
        j += 1;
        k -= 1;
    }
    if sum >= l {
        wi[..i].iter().map(|wi| wi.i).collect()
    } else {
        vec![]
    }
}

pub fn test(l: u32, u: u32, w: &[u32], solvable: bool) {
    let s = find_subset(l, u, w);
    if solvable {
        assert!((l..=u).contains(&s.iter().map(|&i| w[i as usize]).sum()));
    } else {
        assert!(s.is_empty());
    }
}

#[test]
fn t_1() {
    test(15, 17, &[6, 8, 8, 7], true);
    test(15, 17, &[8, 7, 8, 6], true);
    test(15, 17, &[8, 8, 6, 7], true);
}

#[test]
fn t_2() {
    test(14, 15, &[5, 5, 6, 6], false);
    test(14, 15, &[5, 6, 5, 6], false);
    test(14, 15, &[6, 6, 5, 5], false);
}

#[test]
fn t_3() {
    test(10, 20, &[15, 17, 16, 18], true);
    test(10, 20, &[17, 16, 15, 18], true);
}

#[test]
fn t_4() {
    test(13, 13, &[2, 2, 2, 2, 2, 2, 2], false);
}

#[test]
fn t_5() {
    test(13, 14, &[2, 2, 2, 2, 2, 2, 2], true);
}

#[test]
fn t_6() {
    test(10, 10, &[5, 5, 5], true);
}

#[test]
fn t_7() {
    test(6, 9, &[5, 5, 5], false);
}

#[test]
fn t_8() {
    test(2, 3, &[5, 5, 5], false);
}

#[test]
fn t_9() {
    test(100, 110, &[12, 18, 18, 17, 19, 13, 16, 11, 11, 20], true);
}
