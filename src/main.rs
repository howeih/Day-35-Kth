extern crate rand;
use rand::{thread_rng, Rng};

fn kth(items: Vec<i32>, k: usize, depth: usize) -> (i32, usize) {
    if items.len() == 1 {
        return (items[0], depth);
    }
    let pivot_idx = rand::thread_rng().gen_range(0, items.len());
    let pivot = items[pivot_idx];
    let split = items.iter().filter(|x| **x <= pivot).count();
    if k < split {
        let items = items.into_iter().filter(|x| *x <= pivot).collect();
        let depth = depth + 1;
        return kth(items, k, depth);
    } else {
        let items = items.into_iter().filter(|x| *x > pivot).collect();
        let depth = depth + 1;
        return kth(items, k - split, depth);
    }
}

fn main() {
    let mut rng = thread_rng();
    const SIZE: usize = 1000000;
    let mut items = Vec::<i32>::with_capacity(SIZE);
    for i in 0..SIZE {
        items.push(i as i32);
    }
    rng.shuffle(&mut items);
    println!("{:?}", kth(items, SIZE / 2, 1));
}
