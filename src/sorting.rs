pub fn merge<T>(left: &Vec<T>, right: &Vec<T>, sz: usize) -> Vec<T>
where T: PartialOrd + Copy
{
    let mut res: Vec<T> = Vec::with_capacity(sz);
    let mut i: usize = 0;
    let mut j: usize = 0;
    while i < left.len() && j < right.len() {
        if left[i] <= right[j] {
            res.push(left[i]);
            i += 1;
        } else {
            res.push(right[j]);
            j += 1;
        }
    }

    while i < left.len() {
        res.push(left[i]);
        i += 1;
    }

    while j < right.len() {
        res.push(right[j]);
        j += 1;
    }

    res
}

pub fn merge_sort<T>(arr: &Vec<T>) -> Vec<T> 
where T: PartialOrd + Copy
{
    let sz = arr.len();
    if sz < 2 {
        return arr.to_vec();
    }

    let mid = sz / 2;
    let left = merge_sort(&arr[0..mid].to_vec());
    let right = merge_sort(&arr[mid..].to_vec());

    merge(&left, &right, sz)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{distributions::Uniform, Rng};

    #[test]
    fn sort_test() {
        let rng = rand::thread_rng();
        let distr = Uniform::new(-100_000, 100_000);
        let mut samples: Vec<i32> = rng.sample_iter(&distr).take(100_000).collect();
        let sorted = merge_sort(&samples);
        samples.sort();
        assert!(sorted == samples)
    }

    
}