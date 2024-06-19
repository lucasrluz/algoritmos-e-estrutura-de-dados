fn binary_search(array: &[i32], item: i32) -> i32 {
    let mut low: i32 = 0;
    let mut high: i32 = array.len() as i32 - 1;

    while low <= high {
        let mid: i32 = (low + high) / 2;

        if item == array[mid as usize] {
            return mid
        }

        if item < array[mid as usize] {
            high = mid - 1;
        }

        if item > array[mid as usize] {
            low = mid + 1
        }
    }

    return -1
}

#[cfg(test)]
mod binary_search_tests {
    use super::*;

    #[test]
    fn should_return_0() {
        let array = [1];

        assert_eq!(binary_search(&array, 1), 0);
    }

    #[test]
    fn should_return_negative_1() {
        let array = [1];

        assert_eq!(binary_search(&array, 3), -1);
    }

    #[test]
    fn should_return_0_and_1() {
        let array = [1, 2];

        assert_eq!(binary_search(&array, 1), 0);
        assert_eq!(binary_search(&array, 2), 1);
    }

    #[test]
    fn should_return_0_1_and_2() {
        let array = [1, 2, 3];

        assert_eq!(binary_search(&array, 1), 0);
        assert_eq!(binary_search(&array, 2), 1);
        assert_eq!(binary_search(&array, 3), 2);
    }
}
