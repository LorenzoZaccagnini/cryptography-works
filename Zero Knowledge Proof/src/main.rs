use rand::Rng;
use std::io::Error;
use std::io::ErrorKind;

fn main() {
    let mut rng = rand::thread_rng();
    let n = u64::pow(2, 19) - 1;
    let mut a = 5;
    let mut b = 22;

/*     let mut line = String::new();
    println!("Enter secret between 5 and 22 :");
    let x = get_input().trim().parse::<u64>().unwrap(); */

    let x = 9;


    let g = 3;
    let h = 4;

    let r: i128 = (rng.gen_range(0..n)) as i128;

    
    println!("n= {}", n);
    println!("b= {}", b);
    println!("r= {}", r);
    println!("x= {}", x);

    let (c1_0, c1_1, c1_2, c1_3) = positive_proof(g, h, b - x, -r, n);
    println!("\nb-x= {}", b - x);
    let commit1 = (c1_0 * c1_1 * c1_2 * c1_3) % n;

    let (c2_0, c2_1, c2_2, c2_3) = positive_proof(g, h, x - a, r, n);
    println!("\nx-a= {}", x - a);
    let commit2 = (c2_0 * c2_1 * c2_2 * c2_3) % n;

    println!("\nCommit 1: {0}{1}{2}{3}", c1_0, c1_1, c1_2, c1_3);
    println!("Commit 2: {0}{1}{2}{3}", c2_0, c2_1, c2_2, c2_3);

    let prover1 = (mod_pow(g, b, n) * mod_pow_sig(commit1, -1, n)) % n;

    let prover2 = (commit2 * mod_pow(g, a, n)) % n;

    println!("Prover 1: {}", prover1);
    println!("Prover 2: {}", prover2);

    let prover3 = (mod_pow(g, x, n) * mod_pow_sig(h, r, n)) % n;

    println!("Prover 3: {}", prover3);

    if prover1 == prover2 {
        println!(
            "Peggy has proven that her value is between: {0} and {1}",
            a, b
        );
    } else {
        println!(
            "Peggy has NOT proven that her value between: {0} and {1}",
            a, b
        );
    }

    
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}


fn lipmaa(val: u64) -> Result<(u64, u64, u64, u64), std::io::Error> {
    for i in 0..100000 {
        for j in 0..1000 {
            for k in 0..100 {
                for l in 0..10 {
                    if i ^ 2 + j ^ 2 + k ^ 2 + l ^ 2== val {
                        return Ok((i, j, k, l))
                    }
                }
            }
        }
     }
     Err(Error::new(ErrorKind::NotFound, "Couldn't find sequence"))
}



// https://rob.co.bb/posts/2019-02-10-modular-exponentiation-in-rust/
fn mod_pow(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn mod_pow_sig(mut base: u64, mut exp: i128, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }
    let mut result = 1;
    base = base % modulus;
    while exp > 0 {
        if exp % 2 == 1 {
            result = result * base % modulus;
        }
        exp = exp >> 1;
        base = base * base % modulus
    }
    result
}

fn positive_proof(g: u64, h: u64, x: u64, r: i128, n: u64) -> (u64, u64, u64, u64) {
    println!("r pos= {}", r);
    let mut rng = rand::thread_rng();
    let (x0, x1, x2, x3) = lipmaa(x).unwrap();
    let r1 = rng.gen_range(0..n);
    let r2 = rng.gen_range(0..n);
    let r3 = rng.gen_range(0..n);
    println!("r1 = {}", r1);
    println!("r1 cast = {}", r1 as i128);

    let r0: u64 = (r - r1 as i128 - r2 as i128 - r3 as i128) as u64;

    let c0 = (mod_pow(g, x0 ^ 2, n) * mod_pow(h, r0, n)) % n;
    let c1 = (mod_pow(g, x1 ^ 2, n) * mod_pow(h, r1, n)) % n;
    let c2 = (mod_pow(g, x2 ^ 2, n) * mod_pow(h, r2, n)) % n;
    let c3 = (mod_pow(g, x3 ^ 2, n) * mod_pow(h, r3, n)) % n;

    return (c0, c1, c2, c3);
}
