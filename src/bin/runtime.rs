use randomized_skip_list::{SkipList, SkipMap};

use rand::thread_rng;
use rand::seq::SliceRandom;
use std::time::Instant;

fn main() {
    let n = 10000000;
    // let mut sl: SkipList<usize> = SkipList::new();
    let mut sm: SkipMap<usize,usize> = SkipMap::new();
    
    let mut vec: Vec<usize> = (0..n).collect();
    let mut rng = thread_rng();
    vec.shuffle(&mut rng);

    let mut i = 0;
    let mut inserted: Vec<usize> = Vec::new();

    let mut insert_time: Vec<u128> = Vec::new();
    let mut find_time: Vec<u128> = Vec::new();

    for num in vec.iter() {
        let t1 = Instant::now();
        sm.put(num, *num);
        let t2 = Instant::now();
        
        inserted.push(*num);

        let t3 = Instant::now();
        let a = sm.get(&num);
        let t4 = Instant::now();
        assert!(a.is_some());

        let t1 = t2.duration_since(t1).as_nanos();
        let t3 = t4.duration_since(t3).as_nanos();

        insert_time.push(t1);
        find_time.push(t3);

        i += 1;

        eprintln!("{i}");
    }

    let mut deletion_time: Vec<u128> = vec![0;n];

    for j in (0..n).rev() {
        let to_del = vec[j];
        
        let t1 = Instant::now();
        let a = sm.del(&to_del);
        let t2 = Instant::now();
        assert!(a.is_some());

        let t1 = t2.duration_since(t1).as_nanos();

        deletion_time[j] = t1;

        eprintln!("{j}");
    }

    println!("n,put,get,del");
    for i in 0..n {
        println!("{i},{},{},{}", insert_time[i], find_time[i], deletion_time[i]);
    }

}