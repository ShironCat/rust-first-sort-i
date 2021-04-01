use clap::clap_app;
use crossbeam::{channel::unbounded, sync::WaitGroup};
use itertools::Itertools;
use num::{BigInt, BigRational, ToPrimitive};
use parking_lot::Mutex;
use std::{error::Error, sync::Arc, thread::spawn};

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(first_sort_i =>
        (version: "1.0")
        (author: "Victor Azadinho Miranda <victorazadinho@pm.me>")
        (about: "Solve E(n) as described in the 'First Sort I' coding challenge")
        (@arg threads: -t +takes_value "Sets the number of threads to use (default: 1)")
        (@arg VALUE: +required "Sets the value of n to solve")
    )
    .get_matches();
    match matches.value_of("VALUE").unwrap_or("").parse::<u64>() {
        Ok(n) => {
            let result: f64;
            if matches.is_present("threads") {
                result = par_e(
                    n,
                    matches
                        .value_of("threads")
                        .unwrap_or_default()
                        .parse::<usize>()
                        .unwrap_or(1),
                );
            } else {
                result = e(n);
            }
            println!("{}", result);
            Ok(())
        }
        Err(e) => Err(Box::new(e)),
    }
}

fn f(mut l: Vec<u64>) -> u64 {
    let mut i = 0;
    let mut s = 0;
    while i < l.len() - 1 {
        if l[i] > l[i + 1] {
            l.insert(0, l[i + 1]);
            l.remove(i + 2);
            s += 1;
            i = 0;
        } else {
            i += 1;
        }
    }
    s
}

fn par_e(n: u64, t: usize) -> f64 {
    let acc = Arc::new(Mutex::new(BigInt::from(0)));
    let wg = WaitGroup::new();
    let (tx, rx) = unbounded::<bool>();
    for (i, p) in (1..(n + 1))
        .into_iter()
        .permutations(n as usize)
        .unique()
        .enumerate()
    {
        if i > (t - 1) {
            rx.recv().unwrap_or_default();
        }
        let (acc, wg, tx) = (acc.clone(), wg.clone(), tx.clone());
        spawn(move || {
            let r = f(p);
            let mut acc = acc.lock();
            *acc += r;
            drop(wg);
            tx.send(true).unwrap_or_default();
        });
    }
    wg.wait();
    let r = acc.lock();
    BigRational::new(
        r.clone(),
        (1..(n + 1))
            .into_iter()
            .fold(BigInt::from(1), |acc, x| acc * x),
    )
    .to_f64()
    .unwrap_or_default()
}

fn e(n: u64) -> f64 {
    BigRational::new(
        (1..(n + 1))
            .into_iter()
            .permutations(n as usize)
            .unique()
            .map(|x| f((*x).iter().copied().collect()))
            .sum(),
        (1..(n + 1))
            .into_iter()
            .fold(BigInt::from(1), |acc, x| acc * x),
    )
    .to_f64()
    .unwrap_or_default()
}
