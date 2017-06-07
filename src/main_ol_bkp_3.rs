extern crate rand;
extern crate num_cpus;
extern crate threadpool;
extern crate mersenne_twister;

use rand::distributions::{IndependentSample, Range};
use std::collections::HashSet;
use threadpool::ThreadPool;
use mersenne_twister::MersenneTwister;
use rand::{Rng, SeedableRng};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};
use std::u64::MAX;

// elsewhere: let num = num_cpus::get();

fn is_prime_naive(mut n: u64) -> bool {
    assert!(n != 0);
    let n2 = n as f64;
    let sqrt_n2 = n2.sqrt().ceil();
    let sqrt_n2_u64 = sqrt_n2 as u64;
    for i in 2..sqrt_n2_u64{
        if n % i == 0 {
           return false
        }

    }
    true
}

fn modular_exp(mut a: u64, mut b: u64, mut c: u64) -> u64 {
    let b_in_bin = format!("{:b}", b);
    //print!("\n{}\n", b_in_bin);
    //let b_in_bin_c = &b_in_bin[2..];
    let mut a_pow_b_mod_c = 1;
    a = a % c;
    let len = b_in_bin.chars().count();
    for i in 0..len {
        if i > 0 {
            a = a.pow(2);
            a = a % c;
        }
        if b_in_bin.chars().nth(len - i - 1).unwrap() == '1' {
            a_pow_b_mod_c = a_pow_b_mod_c * a;
            a_pow_b_mod_c = a_pow_b_mod_c % c;
        }

    }
    a_pow_b_mod_c
}

fn tu_finder(mut n: u64) -> (u64, u64) {
    assert!(n % 2 == 1);
    let mut u: u64 = n - 1;
    let mut t: u64 = 0;
    while u % 2 == 0 {
        t = t + 1;
        u = u / 2;
    }
    //print!("tufinder returning {}, {}", t, u);
    (t, u)
}

fn is_composite_witness(a: u64, n: u64, t: u64, u: u64) -> bool {
    for i in 0..t{
        //print!("doing loop {} in icw", i);
        let two: u64 = 2;
        if modular_exp(a, (two.pow(i as u32))*u, n) == (n-1){
            return false
        }
        if i == 0 && (modular_exp(a, u, n) == 1){
            return false
        }

    }
    true
}

fn is_prime_mr_multi_core(mut n: u64, mut tests_per_core: u64) -> bool {
    let num_cpu = num_cpus::get();
    let (tx, rx) = channel();
    let mut workers = ThreadPool::new_with_name("worker".into(), num_cpu);
    assert! (tests_per_core != 0 && n != 2 && n != 3);
    if tests_per_core > (n - 2) {
        tests_per_core = n - 2
    }
    if n % 2 == 0 {
        return false
    }
    let tu_tuple = tu_finder(n);
    let mut t: u64 = tu_tuple.0;
    let mut u: u64 = tu_tuple.1;
    let seed: u64 = 12345679;
    let mut rng: MersenneTwister = SeedableRng::from_seed(seed);
    let range = Range::new(3, n - 1);
    let mut random_number: u64 = 3;
    for _ in 0..(tests_per_core * (num_cpu as u64)) {
        random_number = range.ind_sample(&mut rng);
        let tx = tx.clone();
        workers.execute(move || {
            tx.send(is_composite_witness(random_number, n, t, u)).unwrap();
        });
    }
    let mut bools: Vec<bool> = Vec::with_capacity((tests_per_core * (num_cpu as u64)) as usize);
    for _ in 0..(tests_per_core * (num_cpu as u64)) {
        bools.push(rx.recv().unwrap_or(false));
    }
    if bools.contains(&true){
        return false
    }
    true
}


fn is_prime_mr_single_core(mut n: u64, mut tests_per_core: u64) -> bool {
    let num_cpu = num_cpus::get();
    assert! (tests_per_core != 0 && n != 2 && n != 3);
    if tests_per_core > (n - 2) {
        tests_per_core = n - 2
    }
    if n % 2 == 0 {
        return false
    }
    let tu_tuple = tu_finder(n);
    let mut t: u64 = tu_tuple.0;
    let mut u: u64 = tu_tuple.1;
    let seed: u64 = 12345679;
    let mut rng: MersenneTwister = SeedableRng::from_seed(seed);
    let range = Range::new(3, n - 1);
    let mut random_number = 3;
    let mut random_numbers: HashSet<u64> = HashSet::new();
    for i in 1..tests_per_core{
        random_number = range.ind_sample(&mut rng);
        if is_composite_witness(random_number, n, t, u) {
            return false
        }
    }
    true
}


fn main() {
    print!("{}", MAX);
    let not_quite_max: u64 = MAX;
    let not_quite_not_quite_max: u64 = MAX - 200000000;
    let beg1 = Instant::now();
    for i in not_quite_not_quite_max..not_quite_max  {
        if is_prime_mr_multi_core(i, 1) {
            print!("\n{} is prime MR mc\n", i)
        }
   }
    let mid1 = Instant::now();
    for i in not_quite_not_quite_max..not_quite_max {
        if is_prime_mr_single_core(i, 4) {
           print!("\n{} is prime MR sc\n", i)
           }
    }
    let end1 = Instant::now();

    print!("\nit took {:?} time units for the multicore MR and {:?} time units for the single core", mid1.duration_since(beg1), end1.duration_since(mid1))
}