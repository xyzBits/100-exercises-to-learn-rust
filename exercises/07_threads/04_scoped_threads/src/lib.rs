// TODO: Given a vector of integers, split it in two halves
//  and compute the sum of each half in a separate thread.
//  Don't perform any heap allocation. Don't leak any memory.
use std::thread;
pub fn sum(v: Vec<i32>) -> i32 {
    let (left, right) = v.split_at(v.len() / 2);

    // scope 是一个用于创建和管理线程的函数，在某个特定的作用域内创建多个线程，
    // 并在该作用域结束 时自动等待所有线程完成，线程会在闭包完成时自动清理，
    // 无需手动调用 join
    thread::scope(|s| {
        let handle_left = s.spawn(|| left.iter().sum::<i32>());

        let handle_right = s.spawn(|| right.iter().sum::<i32>());

        handle_left.join().unwrap() + handle_right.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
