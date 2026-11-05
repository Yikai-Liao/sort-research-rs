use sort_research_rs::other::rust_tessera_radix;
fn main() {
    let n: usize = std::env::args().nth(1).unwrap().parse().unwrap();
    let mut v: Vec<u64> = (0..n as u64).map(|i| i.wrapping_mul(0x9e3779b97f4a7c15)).collect();
    for i in 0..v.len() { v[i] ^= v[i].rotate_left(31); }
    let t = std::time::Instant::now();
    rust_tessera_radix::sort(&mut v);
    eprintln!("sorted {} in {:?} = {:.2} ns/rec", n, t.elapsed(), t.elapsed().as_secs_f64() * 1e9 / n as f64);
    assert!(v.windows(2).all(|w| w[0] <= w[1]));
}
