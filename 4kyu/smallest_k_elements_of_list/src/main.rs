fn main() {
    println!("Hello, world!");
}
use std::collections::BinaryHeap;

fn get_k_smallest<T: Copy + Ord + PartialOrd>(arr: &mut [T], k: usize) -> Vec<T> {
    let mut heap = BinaryHeap::with_capacity(k);

    for val in arr {
        if val <= k {
            heap.push(Reverse(val));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::get_k_smallest;

    #[test]
    fn sample_tests() {
        test_correctness_helper(&mut [1, 2, 3], 0); // []
        test_correctness_helper(&mut [1, 2, 3], 1); // [1]
        test_correctness_helper(&mut [1, 2, 3], 2); // [1,2]
        test_correctness_helper(&mut [1, 2, 3], 3); // [1,2,3]

        test_correctness_helper(&mut [3, 2, 1], 1); // [1]
        test_correctness_helper(&mut [3, 2, 1], 2); // [1,2]
        test_correctness_helper(&mut [3, 2, 1], 3); // [1,2,3]

        test_correctness_helper(&mut [1, 1, 1], 3); // [1,1,1]
        test_correctness_helper(&mut [1, 1, 1], 2); // [1,1]

        test_correctness_helper(&mut [1, -5, 1], 2); //[-5,1]
        test_correctness_helper(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 2); // [1,2]
        test_correctness_helper(&mut [1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 9); // [1,2,3,4,5,6,7,8,9]
    }

    fn test_correctness_helper(arr: &mut [i64], k: usize) {
        let mut arr_sorted: Vec<i64> = arr.to_vec();
        arr_sorted.sort_unstable();
        // this is the sorted arr, so the first K elements here are also the smallest
        let expected: Vec<i64> = arr_sorted.iter().take(k).copied().collect();
        let ans = get_k_smallest(arr, k);
        let mut ans_sorted = ans.clone();
        ans_sorted.sort();
        // I also sort your answer, because you can give the answers in whatever order you want
        println!("sorted arr: {arr_sorted:?}, your answer: {ans:?}\nyour answer after sorting: {ans_sorted:?}, expected answer: {expected:?}\n");
        assert_eq!(ans_sorted, expected);
    }
}
