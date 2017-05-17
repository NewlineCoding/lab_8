extern crate rand;
extern crate num_cpus;
extern crate threadpool;


use rand::distributions::{IndependentSample, Range};
use std::collections::HashSet;
use threadpool::ThreadPool;
use std::sync::mpsc::channel;

// elsewhere: let num = num_cpus::get();


fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

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

fn is_prime_mr(mut n: u64, mut tests_per_core: u64) -> bool {
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
    let mut rng = rand::thread_rng();
    let range = Range::new(3, n - 1);
    let mut random_number = 3;
    let mut random_numbers: HashSet<u64> = HashSet::new();
    for _ in 0..(tests_per_core * (num_cpu as u64)) {
        while random_numbers.contains(&random_number) {
            //håll koll på ampersandet
            random_number = range.ind_sample(&mut rng);
        }
        random_numbers.insert(random_number);
    }
    for i in random_numbers {
        let tx = tx.clone();
        workers.execute(move || {
            tx.send(is_composite_witness(i, n, t, u)).unwrap();
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

fn main() {
    for i in 1000..1000000 {
        if is_prime_mr(i, 3) {
        print ! ("\n{} is prime MR\n", i)
        }
    }
}