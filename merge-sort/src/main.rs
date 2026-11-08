/*
Merge Sort

Merge sort is a popular sorting algorithm known for its efficiency and stability. 
It follows the Divide and Conquer approach. 
It works by recursively dividing the input array into two halves, recursively sorting the two halves and finally merging them back together to obtain the sorted array.

Here's a step-by-step explanation of how merge sort works:
- Divide: Divide the list or array recursively into two halves until it can no more be divided.
- Conquer: Each subarray is sorted individually using the merge sort algorithm.
- Merge: The sorted subarrays are merged back together in sorted order. The process continues until all elements from both subarrays have been merged.
*/

mod mergesort;

fn main() {
    let mut array1: Vec<i32> = vec![]; // empty list
    let mut array2: Vec<i32> = vec![1]; // one element
    let mut array3: Vec<i32> = vec![-50, -1, 1, 4, 6, 8, 10, 100, 401]; // sorted list
    let mut array4: Vec<i32> = vec![401, 100, 10, 8, 6, 4, 1, -1, -50]; // reverse sorted list
    let mut array5: Vec<i32> = vec![2, 2, 2, 2, 2]; // same element
    let mut array6: Vec<i32> = vec![100, 10, 10, 8, 10000, 4, 1, -1, -50, 100, 4, -1, -1]; // duplicates

    let size = array1.len();
    mergesort::mergesort(&mut array1, 0, 0);
    for i in 0..size {
        print!("{} ", array1[i]);
    }
    println!();
    
    let size = array2.len();
    mergesort::mergesort(&mut array2, 0, size-1);
    for i in 0..size {
        print!("{} ", array2[i]);
    }
    println!();
    
    let size = array3.len();
    mergesort::mergesort(&mut array3, 0, size-1);
    for i in 0..size {
        print!("{} ", array3[i]);
    }
    println!();
    
    let size = array4.len();
    mergesort::mergesort(&mut array4, 0, size-1);
    for i in 0..size {
        print!("{} ", array4[i]);
    }
    println!();
    
    let size = array5.len();
    mergesort::mergesort(&mut array5, 0, size-1);
    for i in 0..size {
        print!("{} ", array5[i]);
    }
    println!();
    
    let size = array6.len();
    mergesort::mergesort(&mut array6, 0, size-1);
    for i in 0..size {
        print!("{} ", array6[i]);
    }
    println!();
}
