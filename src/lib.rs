use crossbeam;
use rand::Rng;

pub fn parallel_sort(data: &mut [i64], threads_count: usize) {
    const MIDDLE_STEP: usize = 10_000;
    let length = data.len();
    if length < 2 {
        return;
    }
    if length < MIDDLE_STEP {
        data.sort();
    } else {
        let chunks = std::cmp::min(length, threads_count);
        crossbeam::scope(|scope| {
            for slice in data.chunks_mut(data.len() / chunks) {
                scope.spawn(move |_| slice.sort());
            }
        })
        .unwrap();
        // Merge result.
        data.sort();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_random_vec() -> Vec<i64> {
        let mut vec = vec![0; 1_000_000];
        for i in vec.iter_mut() {
            *i = rand::thread_rng().gen_range(1..=10_000);
        }
        vec
    }

    #[test]
    fn common() {
        let mut vec1 = get_random_vec();
        let mut vec2 = vec1.clone();

        vec1.sort();
        parallel_sort(vec2.as_mut_slice(), 8);

        assert_eq!(vec1, vec2)
    }
}
