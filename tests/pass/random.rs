use rand::{rngs::SmallRng, Rng, SeedableRng};
// mac-os `rand` does some pointer shenanigans
//@compile-flags: -Zmiri-permissive-provenance

fn main() {
    // Test `getrandom` directly (in multiple different versions).
    let mut data = vec![0; 16];
    getrandom_1::getrandom(&mut data).unwrap();
    getrandom_2::getrandom(&mut data).unwrap();

    // Try seeding with "real" entropy.
    let mut rng = SmallRng::from_entropy();
    let _val = rng.gen::<i32>();
    let _val = rng.gen::<isize>();
    let _val = rng.gen::<i128>();

    // Also try per-thread RNG.
    let mut rng = rand::thread_rng();
    let _val = rng.gen::<i32>();
    let _val = rng.gen::<isize>();
    let _val = rng.gen::<i128>();
}
