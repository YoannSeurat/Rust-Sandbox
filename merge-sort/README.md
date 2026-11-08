# Merge sort

[Merge sort](https://www.geeksforgeeks.org/dsa/merge-sort/) is a popular sorting algorithm known for its efficiency and stability. 

It follows the Divide and Conquer approach. 

It works by recursively dividing the input array into two halves, recursively sorting the two halves and finally merging them back together to obtain the sorted array.

## How it works

Here's a step-by-step explanation of how merge sort works:

- Divide: Divide the list or array recursively into two halves until it can no more be divided.
- Conquer: Each subarray is sorted individually using the merge sort algorithm.
- Merge: The sorted subarrays are merged back together in sorted order. The process continues until all elements from both subarrays have been merged.

![Merge sort example](https://media.geeksforgeeks.org/wp-content/uploads/20250923102849709166/arr_.webp)

## Run

```bash
cargo run
```

The user has nothing to input, the program just tests some basic arrays to see if the merge sort code works.

The arrays tested are :

```rust
[]; // empty list
[1]; // one element
[-50, -1, 1, 4, 6, 8, 10, 100, 401]; // sorted list
[401, 100, 10, 8, 6, 4, 1, -1, -50]; // reverse sorted list
[2, 2, 2, 2, 2]; // same element
[100, 10, 10, 8, 10000, 4, 1, -1, -50, 100, 4, -1, -1]; // duplicates
```

## Example

```text


1 
-50 -1 1 4 6 8 10 100 401 
-50 -1 1 4 6 8 10 100 401 
2 2 2 2 2 
-50 -1 -1 -1 1 4 4 8 10 10 100 100 10000
```