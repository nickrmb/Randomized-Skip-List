use rand::{thread_rng, Rng};
use randomized_skip_list::SkipList;

fn main() {
    let mut sl: SkipList<isize> = SkipList::new();

    let mut rng = thread_rng();

    for _ in 0..10 {
        let rnd: isize = rng.gen_range(-10..=10);
        sl.insert(rnd);
    }

    for i in sl.iter() {
        println!("{i}");
    }

    if sl.delete(&7) != None {
        println!("has 7");
        for i in sl.iter() {
            println!("{i}");
        }
    }
}
