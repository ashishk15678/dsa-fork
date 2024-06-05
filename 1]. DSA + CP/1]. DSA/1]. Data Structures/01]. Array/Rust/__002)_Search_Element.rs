fn search(arr: Vec<u8>, n: u8) -> i8 {
    for i in 0..arr.len() {
        if (arr[i] == n) {
            return i as i8;
        }
    }
    return -1;
}

fn main() {
    println!("{}", search([20, 5, 6, 13].to_vec(), 1));
}
