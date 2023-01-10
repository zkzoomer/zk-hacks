use ark_bls12_381::Fr;
use ark_ec::AffineCurve;
use prompt::{puzzle, welcome};
use std::str::FromStr;
use trusted_setup::data::puzzle_data;
use trusted_setup::PUZZLE_DESCRIPTION;

fn main() {
    welcome();
    puzzle(PUZZLE_DESCRIPTION);
    let (_ts1, _ts2) = puzzle_data();

    let s_prime = Fr::from_str("5592216610550884993006174526481245").unwrap();
    let n_prime = Fr::from_str("38452154918091875653578148163112927").unwrap();

    for k in 0..70000 {
        let s = n_prime*Fr::from(k) + s_prime;
        if _ts1[0].mul(s) == _ts1[1] && _ts2[0].mul(s) == _ts2[1] {
            println!("{}", s);
        }
    }

    let s = Fr::from_str("114939083266787167213538091034071020048").unwrap();
    assert_eq!(_ts1[0].mul(s), _ts1[1]);
    assert_eq!(_ts2[0].mul(s), _ts2[1]);

    println!("gottem");
}
