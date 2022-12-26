
mod sort;

#[cfg(test)]
mod tests {
    use super::sort::*;

    #[test]
    fn test_bubble_sort() {
        let mut arr = vec![5, 3, 6, 7, 9, 1, 8, 4, 2];
        bubble_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_select_sort() {
        let mut arr = vec![5, 3, 6, 7, 9, 1, 8, 4, 2];
        select_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_insert_sort() {
        let mut arr = vec![5, 3, 6, 7, 9, 1, 8, 4, 2];
        assert_eq!(insert_sort(&mut arr), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_merge_sort() {
        let arr = vec![5, 3, 6, 7, 9, 1, 8, 4, 2];
        assert_eq!(merge_sort(arr), vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }

    #[test]
    fn test_quick_sort() {
        let mut arr = vec![5, 3, 6, 7, 9, 1, 8, 4, 2];
        quick_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]);
    }
}
