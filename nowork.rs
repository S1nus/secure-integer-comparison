use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    const share_count : usize = 2;

    let tss = ShamirSecretSharing {
        threshold: 1,
        share_count: share_count,
        prime: 11,
    };

    let num_1 = 5;
    let num_2 = 2;

    let num1_shares = tss.share(num_1);
    let num2_shares = tss.share(num_2);

    println!("num1 shares: {:?}", num1_shares);
    println!("num2 shares: {:?}", num2_shares);

    //println!("reconst {}", tss.reconstruct(&[0,1,2], &[num1_shares[0], num1_shares[1], num1_shares[2]]));
    
    let person_0_product = (num1_shares[0] * num2_shares[0])%tss.prime;
    println!("person_0_product {}", person_0_product);
    let person_1_product = (num1_shares[1] * num2_shares[1])%tss.prime;
    println!("person_1_product {}", person_1_product);

    let person_0_product_shares = tss.share(person_0_product);
    println!("person_0_product_shares {:?}", person_0_product_shares);
    let person_1_product_shares = tss.share(person_1_product);
    println!("person_1_product_shares {:?}", person_1_product_shares);

    let p0ps0_s = tss.share(person_0_product_shares[0]);
    println!("p0ps0: {:?}", p0ps0_s);
    let p0ps1_s = tss.share(person_0_product_shares[1]);
    println!("p0ps1: {:?}", p0ps1_s);

    let p1ps0_s = tss.share(person_1_product_shares[0]);
    println!("p1ps0: {:?}", p1ps0_s);
    let p1ps1_s = tss.share(person_1_product_shares[1]);
    println!("p1ps1: {:?}", p1ps1_s);

    let f0 = tss.reconstruct(&[0,1], &[p0ps0_s[0], p1ps0_s[0]]);
    let f1 = tss.reconstruct(&[0,1], &[p0ps0_s[1], p1ps0_s[1]]);

    let f2 = tss.reconstruct(&[0,1], &[p0ps1_s[0], p1ps1_s[0]]);
    let f3 = tss.reconstruct(&[0,1], &[p0ps1_s[1], p1ps1_s[1]]);

    let g0 = tss.reconstruct(&[0,1], &[f0, f1]);
    let g1 = tss.reconstruct(&[0,1], &[f2, f3]);

    let res = tss.reconstruct(&[0,1], &[g0, g1]);
    println!("res {}", res);
}
