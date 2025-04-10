/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/


fn sort<T: std::cmp::PartialOrd + Clone>(array: &mut [T]){
	//TODO
    fn add<T: std::cmp::PartialOrd + Clone>(heap: &mut Vec<T>, val: T, length: &mut usize) {
        *length += 1;
        heap.push(val);
        let mut i = *length;
        while i > 1 && heap[i / 2] > heap[i] {
            heap.swap(i, i / 2);
            i /= 2;
        }
    }

    fn pop<T: std::cmp::PartialOrd + Clone>(heap: &mut Vec<T>, length: &mut usize) -> T {
        let ret = heap[1].clone();
        heap.swap(1, *length);
        *length -= 1;
        let mut i = 1;
        while i * 2 <= *length {
            let mut child = i * 2;
            if child + 1 <= *length && heap[child + 1] < heap[child] {
                child += 1;
            }
            if heap[i] > heap[child] {
                heap.swap(i, child);
                i = child;
            } else {
                break;
            }
        }
        ret
    }

    let mut heap: Vec<T> = array.to_vec();
    heap.push(array[0].clone());
    let mut length: usize = 0;

    for i in 0..array.len() {
        add(&mut heap, array[i].clone(), &mut length);
    }

    for i in 0..array.len() {
        array[i] = pop(&mut heap, &mut length);
    }
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