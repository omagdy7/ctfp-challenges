#![allow(dead_code)]
#![allow(unused_variables)]

use std::{collections::HashMap, fmt::Debug, hash::Hash, usize};

struct MemoFunc<K, V> {
    cache: HashMap<K, V>,
    func: fn(&mut MemoFunc<K, V>, K) -> V,
}

trait Call<K, V> {
    fn call(&mut self, arg: K) -> V;
}

impl<K, V> Call<K, V> for MemoFunc<K, V>
where
    K: Clone + Hash + Eq + Debug,
    V: Clone,
{
    fn call(&mut self, arg: K) -> V {
        let func = self.func.clone();
        if let Some(ret) = self.cache.get(&arg) {
            println!("{:?} was cached", arg);
            ret.clone()
        } else {
            let ret = func(self, arg.clone());
            self.cache.insert(arg, ret.clone());
            ret
        }
    }
}

impl<K, V> MemoFunc<K, V>
where
    K: Eq + Hash + Clone,
    V: Clone,
{
    pub fn from_func(func: fn(&mut Self, K) -> V) -> Self {
        Self {
            cache: HashMap::new(),
            func,
        }
    }
}

struct NoMemoize<K, V>(fn(&mut NoMemoize<K, V>, K) -> V);

impl<K, V> Call<K, V> for NoMemoize<K, V> {
    fn call(&mut self, arg: K) -> V {
        (self.0)(self, arg)
    }
}

fn fib<C: Call<usize, usize>>(cached_func: &mut C, n: usize) -> usize {
    match n {
        0 => 0,
        1 => 1,
        _ => cached_func.call(n - 1) + cached_func.call(n - 2),
    }
}

fn always_true(a: bool) -> bool {
    true
}
fn always_false(a: bool) -> bool {
    false
}
fn same(a: bool) -> bool {
    a
}
fn negate(a: bool) -> bool {
    !a
}

#[cfg(test)]
mod tests {

    use super::*;
    #[test]
    fn test_same_behaviour() {
        assert_eq!(NoMemoize(fib).call(20), MemoFunc::from_func(fib).call(20));
    }

    #[test]
    fn test_fib_no_memoize() {
        use std::time::Duration;
        use std::time::Instant;
        let mut normal_fib = NoMemoize(fib);
        let start = Instant::now();
        normal_fib.call(35);
        let elapsed = start.elapsed();
        assert!(elapsed > Duration::from_millis(100));
    }

    #[test]
    fn rand_fails() {
        use rand::Rng;
        fn random<C>(_: &mut C, _: ()) -> f64 {
            let mut rng = rand::rng();
            rng.random::<f64>()
        }

        assert_ne!(
            NoMemoize(random).call(()),
            MemoFunc::from_func(random).call(())
        );
    }

    #[test]
    fn rand_works() {
        use rand::{rng, Rng};
        use rand::{rngs::StdRng, SeedableRng}; // 0.8.2
        fn seeded<M>(_: &mut M, seed: [u8; 32]) -> f64 {
            let mut rng = StdRng::from_seed(seed);
            rng.random::<f64>()
        }
        let mut seed = [0u8; 32];
        rng().fill(&mut seed[..]);
        assert_eq!(
            NoMemoize(seeded).call(seed),
            MemoFunc::from_func(seeded).call(seed)
        );
    }

    #[test]
    fn test_fib_memoize() {
        use std::time::Duration;
        use std::time::Instant;
        let mut memoized_fib = MemoFunc::from_func(fib);
        let start = Instant::now();
        memoized_fib.call(40);
        let elapsed = start.elapsed();
        assert!(elapsed < Duration::from_millis(1));
    }
}
