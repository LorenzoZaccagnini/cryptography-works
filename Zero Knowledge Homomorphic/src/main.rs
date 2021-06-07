fn main() {
    let n: u64 = 101;
    let g: u64 = 3;
    let ans: u64= 7;
    let x: u64 = 3;
    let y: u64 = 4;

    let E1: u64 = n_mod_m(u64::pow(g,n_mod_m(x+y, n-1) as u32),n);
    let E2: u64 = n_mod_m(u64::pow(g, x as u32) * u64::pow(g, y as u32), n);
    let E3: u64 = n_mod_m(u64::pow(g, ans as u32), n);

    println!("======Agreed parameters============");
    println!("P= {}", n);
    println!("G= {}", g);
    println!("x= {}", x);
    println!("y= {}", y);
    println!("ans= {0} calc ans = {1}", ans, x + y);

    println!("======Encrypted values============");
    println!("g^y={}",n_mod_m(g^y, n));
    println!("g^x={}", n_mod_m(g.pow(x  as u32), n));

    println!("======zkSnark====================");
    println!("E1={}",E1);
    println!("E2={}",E2);
    println!("E3={}",E3);

    if E2==E3 {
        println!("Alice has proven she knows the sum is {}",ans)
    } else {
        println!("Alice has proven she does not know the sum is {}",ans)
    }

}

fn n_mod_m <T: std::ops::Rem<Output = T> + std::ops::Add<Output = T> + Copy>
  (n: T, m: T) -> T {
    ((n % m) + m) % m
}