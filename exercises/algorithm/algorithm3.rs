/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/
// I AM NOT DONE

use std::slice::SliceIndex;

fn quick_sort<T: PartialEq + PartialOrd + Copy>(array: &mut [T], left: usize, right: usize) {
    if right-left<=1{
        return;
    }
    let first = left;
    let last=right;
    let mut left = left+1;
    let mut right = right-1;
    let privo = *array.get(first).unwrap();
    while left < right {
        while *array.get(left).unwrap() <= privo && left < right {
            left += 1;
        }
        while *array.get(right).unwrap() >= privo && left < right {
            right -= 1;
        }
        if left < right {
            let tmp = *array.get(left).unwrap();
            *array.get_mut(left).unwrap() = *array.get(right).unwrap();
            *array.get_mut(right).unwrap() = tmp;
        }else{
            break;
        }
    }
    let tmp =*array.get(first).unwrap();
    *array.get_mut(first).unwrap() = *array.get(right).unwrap();
    *array.get_mut(right).unwrap() = tmp;
    quick_sort(array,first,right);
    quick_sort(array,right+1,last);
}

fn sort<T: PartialOrd + Copy>(array: &mut [T]) {
    quick_sort(array,0,array.len());
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