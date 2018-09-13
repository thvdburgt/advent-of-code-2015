extern crate crypto;

use crypto::digest::Digest;
use crypto::md5::Md5;

fn mine_advent_coin<F>(key: &[u8], predicate: F) -> u64
where
    F: Fn([u8; 16]) -> bool,
{
    let mut hasher = Md5::new();

    for i in 0..std::u64::MAX {
        hasher.input(key);
        hasher.input(i.to_string().as_bytes());

        let mut output = [0; 16];
        hasher.result(&mut output);

        if predicate(output) {
            return i;
        }
        hasher.reset();
    }

    0
}

pub fn solve_puzzle_part_1(input: &str) -> u64 {
    mine_advent_coin(&(input.as_bytes()), |a| {
        a[0] as u32 + a[1] as u32 + (a[2] >> 4) as u32 == 0
    })
}

pub fn solve_puzzle_part_2(input: &str) -> u64 {
    mine_advent_coin(&(input.as_bytes()), |a| {
        a[0] as u32 + a[1] as u32 + a[2] as u32 == 0
    })
}
