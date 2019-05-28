pub mod binarytree;
pub mod sidewinder;
pub mod solutions;
use rand::Rng;
use rand::distributions::{Distribution, Standard};

#[derive(Debug)]
enum CoinFlip {
  Heads,
  Tails
}

impl Distribution<CoinFlip> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> CoinFlip {
        if rng.gen::<bool>() {
          CoinFlip::Heads
        } else {
          CoinFlip::Tails
        }
    }
}
