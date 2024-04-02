fn kth_smallest(arr: &mut [i32], k: usize) -> i32 {
    arr.sort();
    arr[k - 1]
}

fn main() {
    let mut arr = [7, 10, 4, 3, 20, 15];
    let k = 3;
    println!("The {}th smallest element is {}", k, kth_smallest(&mut arr, k));
}
