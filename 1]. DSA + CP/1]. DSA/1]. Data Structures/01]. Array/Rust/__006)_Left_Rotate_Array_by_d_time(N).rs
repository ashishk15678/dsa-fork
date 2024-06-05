// This algo takes extra space O(d) and time O(N)

pub fn lRotate(d: usize, mut v: Vec<u8>) -> Vec<u8> {
    let mut temp = vec![];

    for i in 0..d {
        temp.push(v[i]);
    }
    for j in 0..(v.len() - d) {
        v[j] = v[d + j]
    }
    for k in 0..d {
        let t = k + (v.len() - d);
        v[t] = temp[k];
    }
    v
}

fn main() {
    let v: Vec<u8> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let d: usize = 5;
    println!("Before rotation : {:?}", &v);
    println!("After rotation by d  : {:?} = {:?}", &d, lRotate(d, v));
}
