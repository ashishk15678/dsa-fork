pub fn rotateleft(mut arr: Vec<usize>) -> Vec<usize> {
    let temp: usize = arr[0];
    for i in 0..arr.len() - 1 {
        arr[i] = arr[i + 1];
    }
    let len = arr.len() - 1;
    arr[len] = temp;
    arr
}

pub fn rotate(mut arr: Vec<usize>, d: usize) -> Vec<usize> {
    for _i in 0..d {
        let v = rotateleft(arr);
        arr = v;
    }
    arr
}

pub fn main() {
    let arr: Vec<usize> = [1, 2, 3, 4, 5, 6, 7, 8, 9].to_vec();
    println!("before rotation by d : {:?}", &arr);
    let d = 4;
    println!("After rotation by d : {} = {:?}", &d, rotate(arr, d));
}
