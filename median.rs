fn find_median(arr: &[i32]) -> f64 {
    let len = arr.len();
    if len % 2 == 0 {
        // Array has even number of elements
        let mid = len / 2;
        (arr[mid - 1] as f64 + arr[mid] as f64) / 2.0
    } else {
        // Array has odd number of elements
        let mid = len / 2;
        arr[mid] as f64
    }
}

fn main() {
    let arr1 = [1, 2, 3, 4, 5];
    let arr2 = [1, 2, 3, 4, 5, 6];

    println!("Median of {:?}: {}", arr1, find_median(&arr1));
    println!("Median of {:?}: {}", arr2, find_median(&arr2));
}
