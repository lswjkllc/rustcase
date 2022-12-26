use std::fmt;

// 冒泡排序
pub fn bubble_sort<T: Ord + fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 0..arr.len()-1 {
        for j in 0..arr.len()-1-i {
            if arr[j] > arr[j+1] {
                // println!("{:?}", arr);
                arr.swap(j, j+1);
            }
        }
    }
}

// 选择排序
pub fn select_sort<T: Ord + fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    for i in 0..arr.len()-1 {
        let mut min_idx = i;
        for j in i+1..arr.len() {
            if arr[min_idx] > arr[j] {
                // println!("{:?}", arr);
                min_idx = j;
            }
        }
        arr.swap(min_idx, i);
    }
}

// 插入排序
pub fn insert_sort<T: Ord + Clone + fmt::Debug>(arr: &mut [T]) -> Vec<T> {
    let mut sorted_arr = Vec::with_capacity(arr.len());

    for item in arr.iter().cloned() {
        let n_len = sorted_arr.len();
        for i in 0..=n_len {
            if i == n_len || item < sorted_arr[i] {
                // println!("{:?}", sorted_arr);
                sorted_arr.insert(i, item);
                break;
            }
        }
    }

    sorted_arr
}

// 归并排序
pub fn merge_sort<T: Ord + fmt::Debug>(mut arr: Vec<T>) -> Vec<T> {
    if arr.len() <= 1 {
        return arr;
    }

    // spilt vector
    let oarr = arr.split_off(arr.len() / 2);

    // sort left
    let left_arr = merge_sort(arr);
    // sort right
    let right_arr = merge_sort(oarr);
    // bring the sorted halfs together
    
    merge(left_arr, right_arr)
}

fn merge<T: Ord + fmt::Debug>(left_arr: Vec<T>, right_arr: Vec<T>) -> Vec<T> {
    let mut sorted_arr = Vec::with_capacity(left_arr.len() + right_arr.len());

    let mut left_iter = left_arr.into_iter();
    let mut right_iter = right_arr.into_iter();
    let mut left_peek = left_iter.next();
    let mut right_peek = right_iter.next();
    loop {
        match left_peek {
            Some(ref left_val) => match right_peek {
                Some(ref right_val) => {
                    if left_val > right_val {
                        sorted_arr.push(right_peek.unwrap());
                        right_peek = right_iter.next();
                    } else {
                        sorted_arr.push(left_peek.unwrap());
                        left_peek = left_iter.next();
                    }
                },
                None => {
                    sorted_arr.push(left_peek.unwrap());
                    sorted_arr.extend(left_iter);
                    break 
                },
            },
            None => {
                if let Some(right_val) = right_peek {
                    sorted_arr.push(right_val);
                }
                sorted_arr.extend(right_iter);
                break 
            },
        }
    }
    // println!("{:?}", sorted_arr);

    sorted_arr
}

// 快速排序
pub fn quick_sort<T: Ord + fmt::Debug>(arr: &mut [T]) {
    if arr.len() <= 1 {
        return;
    }

    let q = pivot(arr);
    let (left, right) = arr.split_at_mut(q);

    quick_sort(left);
    quick_sort(&mut right[1..]);
}

fn pivot<T: Ord + fmt::Debug>(arr: &mut [T]) -> usize {
    // println!("{:?}", arr);
    let mut p = 0;
    for i in 1..arr.len() {
        if arr[i] < arr[p] {
            // p+1 <-> i
            arr.swap(p+1, i);
            // p   <-> p+1
            arr.swap(p, p+1);
            // p++
            p += 1;
        }
    }

    p
}