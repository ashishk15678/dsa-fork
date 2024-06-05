pub fn rotateLeft1(mut arr: Vec<usize>) -> Vec<usize> {
    let mut temp: usize = arr[0];
    for i in 0..arr.len() - 1 {
        arr[i] = arr[i + 1];
    }

    //have done this because mutable borrow and immutable borrow
    //of array in one line is an issue to compiler
    let t = arr.len() - 1;
    arr[t] = temp;
    arr
}

fn main() {
    let mut arr: Vec<usize> = [1, 2, 3, 4, 5, 6].to_vec();
    println!("Before left rotate : {:?}", &arr);
    println!("After left rotate : {:?}", rotateLeft1(arr));
}
