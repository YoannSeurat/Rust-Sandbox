// begin is for left index and end is right index
// of the sub-array of arr to be sorted
pub fn mergesort(array: &mut Vec<i32>, begin: usize, end: usize) {

    if end > begin {
        let mid = begin + (end - begin) / 2;
        mergesort(array, begin, mid);
        mergesort(array, mid+1, end);
        merge(array, begin, mid, end);
    }

}

// Merges two sorted subarrays of array.
// First subarray is array[left..mid]
// Second subarray is array[mid+1..right]
pub fn merge(array: &mut Vec<i32>, left: usize, mid:usize, right: usize) {

    //print!("Merging : ");
    let mut left_arr: Vec<i32> = Vec::new();
    for i in 0..(mid-left+1) {
        left_arr.push(array[left + i]);
        //print!("{} ", left_arr[i]);
    }
    //print!(" and ");
    let mut right_arr: Vec<i32> = Vec::new();
    for i in 0..(right-mid) {
        right_arr.push(array[mid + 1 + i]);
        //print!("{} ", right_arr[i]);
    }
    //println!();
    
    let mut curr_array_index = left; // where we should place current number in the array
    let mut curr_left_arr_index = 0; // what current number we are looking at in the left array
    let mut curr_right_arr_index = 0; // what current number we are looking at in the right array

    while curr_left_arr_index < mid-left+1 && curr_right_arr_index < right-mid {
        if left_arr[curr_left_arr_index] <= right_arr[curr_right_arr_index] {
            array[curr_array_index] = left_arr[curr_left_arr_index];
            curr_left_arr_index += 1;
        } else {
            array[curr_array_index] = right_arr[curr_right_arr_index];
            curr_right_arr_index += 1;
        }
        curr_array_index += 1;
    }

    while curr_left_arr_index < mid-left+1 {
        array[curr_array_index] = left_arr[curr_left_arr_index];
        curr_left_arr_index += 1;
        curr_array_index += 1;
    }

    while curr_right_arr_index < right-mid {
        array[curr_array_index] = right_arr[curr_right_arr_index];
        curr_right_arr_index += 1;
        curr_array_index += 1;
    }

    // print!("Result : ");
    // for i in left..=right {
    //     print!("{} ", array[i]);
    // }
    // println!();

}