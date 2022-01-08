use std::{thread::sleep, time::Duration};

const CLEAR_LINE: &str = "\x1B[2J\x1B[1;1H";

struct Progress<Iter> {
    it: Iter,
    i: usize,
}

impl<Iter> Progress<Iter> {
    pub fn new(it: Iter) -> Self {
        Progress { it, i: 0 }
    }
}

impl<Iter> Iterator for Progress<Iter>
where
    Iter: Iterator,
{
    type Item = Iter::Item;
    fn next(&mut self) -> Option<Self::Item> {
        println!("{}{}", CLEAR_LINE, "∎".repeat(self.i));
        self.i += 1;

        self.it.next()
    }
}

trait ProgressIteratorExt: Sized {
    fn progress(self) -> Progress<Self>;
}

impl<Iter> ProgressIteratorExt for Iter
where
    Iter: Iterator,
{
    fn progress(self) -> Progress<Self> {
        Progress::new(self)
    }
}

fn expensive_calculation(_n: &i32) {
    sleep(Duration::from_secs(1));
}

fn main() {
    let v = vec![1, 2, 3];
    for n in v.iter().progress() {
        expensive_calculation(n);
    }
}
