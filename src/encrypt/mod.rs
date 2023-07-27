use crate::matrix::{Matrix, Row};
fn encrypt<const H: usize, const W: usize> (m: Matrix<H, W>, sols: Vec<usize>, height: usize, md: usize) {
    println!("{height}");
    let c = "a";
    let b = c.as_bytes()[0];
    let i: u8 = 0b10000000;

    let mut rng = lfsr(4);
    let i = rng() % height;
    let mut s = m[i];
    let mut b = sols[i];

    let ans = vec![99, 111, 102, 102, 101, 101, 99, 108, 117, 98];
    let rans = Row::new(&ans);

    println!("{}", s * rans);

    println!("{}\n{:?}", s, b);
        
}

fn lfsr(mut seed: usize) -> impl FnMut() -> usize {
    move || {
        let mut out = 0;
        for _ in 0..64 {
            let newbit = (seed ^ (seed >> 1) ^ (seed >> 3) ^ (seed >> 4)) & 1;
            out <<= 1;
            out |= seed & 1;
            seed = (seed >> 1) | (newbit << 63);
        }
        out
    }
}
