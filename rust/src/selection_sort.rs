fn find_smallest(vector: &Vec<i16>) -> usize {
    let mut smallest = vector[0];
    let mut smallest_index = 0;

    for index in 1..vector.len() {
        if vector[index] < smallest {
            smallest = vector[index];
            smallest_index = index;
        } 
    }

    return smallest_index
}

fn selection_sort(mut vector: Vec<i16>) -> Vec<i16> {
    let mut new_vector = Vec::new();

    for _i in 0..vector.len() {
        let smallest_index = find_smallest(&vector);

        new_vector.push(vector.remove(smallest_index));
    }

    return new_vector;
}

#[cfg(test)]
mod selection_sort_tests {
    use super::*;

    #[test]
    fn should_return_vec_with_1() {
        let vector = vec![1];

        assert_eq!(selection_sort(vector.clone()), [1]);
    }

    #[test]
    fn should_return_vec_with_1_2() {
        let vector = vec![1, 2];
        assert_eq!(selection_sort(vector.clone()), [1, 2]);
        
        let vector = vec![2, 1];
        assert_eq!(selection_sort(vector.clone()), [1, 2]);
    }

    #[test]
    fn should_return_vec_with_1_2_3() {
        let vector = vec![1, 2, 3];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
        
        let vector = vec![2, 1, 3];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
 
        let vector = vec![3, 2, 1];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
 
        let vector = vec![1, 3, 2];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
 
        let vector = vec![3, 1, 2];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
 
        let vector = vec![2, 3, 1];
        assert_eq!(selection_sort(vector.clone()), [1, 2, 3]);
    }
}

#[cfg(test)]
mod find_smallest_tests {
    use super::*;

    #[test]
    fn should_return_0() {
        let vector = vec![1];

        assert_eq!(find_smallest(&vector), 0);
    }

    #[test]
    fn should_return_0_and_1() {
        let vector = vec![1, 2];
        assert_eq!(find_smallest(&vector), 0);

        let vector = vec![2, 1];
        assert_eq!(find_smallest(&vector), 1);
    }

    #[test]
    fn should_return_0_1_and_2() {
        let vector = vec![1, 2, 3];
        assert_eq!(find_smallest(&vector), 0);

        let vector = vec![2, 1, 3];
        assert_eq!(find_smallest(&vector), 1);

        let vector = vec![3, 2, 1];
        assert_eq!(find_smallest(&vector), 2);
    }
}
