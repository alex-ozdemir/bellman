use bellman::{
    multicore::Worker,
    multiexp::{multiexp, FullDensity},
};
use bls12_381::{Bls12, Scalar};
use cpu_time::ProcessTime;
use ff::Field;
use group::{Curve, Group};
use pairing::Engine;
use rand_core::SeedableRng;
use rand_xorshift::XorShiftRng;
use std::str::FromStr;
use std::sync::Arc;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let trials: usize = usize::from_str(&args[1]).expect("arg 1: trials");
    let elems_log2: usize = usize::from_str(&args[2]).expect("arg 2: log2(elems)");
    let zero_bytes: usize = usize::from_str(&args[3]).expect("arg 3: zero bytes");
    assert!(zero_bytes <= 32);
    let elems = 1 << elems_log2;
    let mut rng = XorShiftRng::from_seed([7; 16]);
    let mut net_cpu_time = std::time::Duration::ZERO;
    let mut net_wall_time = std::time::Duration::ZERO;
    for _ in 0..trials {
        let v = Arc::new(
            (0..elems)
                .map(|_| {
                    let s = Scalar::random(&mut rng);
                    let mut sbytes = s.to_bytes();
                    for i in (sbytes.len() - zero_bytes)..sbytes.len() {
                        sbytes[i] = 0;
                    }
                    Scalar::from_bytes(&sbytes).unwrap()
                })
                .collect::<Vec<_>>(),
        );
        let v_bits = Arc::new(v.iter().map(|e| e.into()).collect::<Vec<_>>());
        let g = Arc::new(
            (0..elems)
                .map(|_| <Bls12 as Engine>::G1::random(&mut rng).to_affine())
                .collect::<Vec<_>>(),
        );

        let pool = Worker::new();

        let start = std::time::Instant::now();
        let start_p = ProcessTime::now();
        let _: <Bls12 as Engine>::G1 = multiexp(&pool, (g.clone(), 0), FullDensity, v_bits.clone())
            .wait()
            .unwrap();
        let end = std::time::Instant::now();
        let time_cpu = start_p.elapsed();
        net_cpu_time += time_cpu;
        net_wall_time += end - start;
    }
    net_cpu_time /= trials as u32;
    net_wall_time /= trials as u32;
    println!("elems       : {}", elems);
    println!("zero bytes  : {}/32", zero_bytes);
    println!("wall      us: {}", net_wall_time.as_secs_f64() * 1e6);
    println!("cpu       us: {}", net_cpu_time.as_secs_f64() * 1e6);
    println!(
        "wall us/elem: {}",
        net_wall_time.as_secs_f64() / elems as f64 * 1e6
    );
    println!(
        "cpu  us/elem: {}",
        net_cpu_time.as_secs_f64() / elems as f64 * 1e6
    );
}
