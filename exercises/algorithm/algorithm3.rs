/*
    sort
    This problem requires you to implement a sorting algorithm
    you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn partition<T: PartialOrd>(array: &mut [T], i: usize, j: usize) -> usize {
    let mut n = i;
    for k in i..j {
        if array[k] < array[j] {
            array.swap(n, k);
            n += 1;
        }
    }
    array.swap(n, j);
    n
}

fn sort0<T: PartialOrd>(array: &mut [T], i: usize, j: usize) {
    if i >= j {
        return;
    };
    let n = partition(array, i, j);
    if n > 0 {
        sort0(array, i, n - 1);
    }
    sort0(array, n + 1, j);
}

fn sort<T: PartialOrd>(array: &mut [T]) {
    //TODO
    if array.is_empty() {
        return;
    }
    sort0(array, 0, array.len() - 1);
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
