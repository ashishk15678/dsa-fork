#![crate_name = "_1)_Reverse_Array"]

fn reverse(mut arr: Vec<usize>) -> Vec<usize> {
    // on purpose taken all values as usize , else getting indexing error
    let mut high: usize = arr.len() - 1;
    let mut low: usize = 0;

    // run the loop untill high is less than low
    while high > low {
        // swap using XOR
        arr[high] = arr[high] ^ arr[low];
        arr[low] = arr[high] ^ arr[low];
        arr[high] = arr[high] ^ arr[low];
        high -= 1;
        low += 1;
    }
    return arr;
}

fn main() {
    println!("{:?}", reverse([5, 6, 7, 8, 9].to_vec()));
}
