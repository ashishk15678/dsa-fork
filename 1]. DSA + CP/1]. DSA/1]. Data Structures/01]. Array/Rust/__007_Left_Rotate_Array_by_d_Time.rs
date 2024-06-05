// This algorithm takes no extra space
// but time of O(2N)

fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let d: usize = 4;
    println!("Before rotation : {:?}", &v);
    println!("After rotation by d : {} = {:?}", &d, lRotate(v, d));
}

pub fn lRotate(mut v: Vec<u8>, d: usize) -> Vec<u8> {
    v = reverse(d - 1, 0, v);
    v = reverse(v.len() - 1, d, v);
    reverse(v.len() - 1, 0, v)
}

pub fn reverse(mut high: usize, mut low: usize, mut v: Vec<u8>) -> Vec<u8> {
    while high > low {
        //        println!("{:?} {:?}", v[high], v[low]);

        v[high] = v[low] ^ v[high];
        v[low] = v[high] ^ v[low];
        v[high] = v[high] ^ v[low];
        //      println!("{:?} {:?}", v[high], v[low]);
        low += 1;
        high -= 1;
        //println!("iterated");
    }
    v
}
