use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    const share_count : usize = 3;

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
    let person_2_product = (num1_shares[2] * num2_shares[2])%tss.prime;
    println!("person_2_product {}", person_2_product);

    let person_0_product_shares = tss.share(person_0_product);
    println!("person_0_product_shares {:?}", person_0_product_shares);
    let person_1_product_shares = tss.share(person_1_product);
    println!("person_1_product_shares {:?}", person_1_product_shares);
    let person_2_product_shares = tss.share(person_2_product);
    println!("person_2_product_shares {:?}", person_2_product_shares);

    let fp0 = tss.reconstruct(&[0,1,2], &[person_0_product_shares[0], person_1_product_shares[0], person_2_product_shares[0]]);
    println!("fp0: {}", fp0);
    let fp1 = tss.reconstruct(&[0,1,2], &[person_0_product_shares[1], person_1_product_shares[1], person_2_product_shares[1]]);
    println!("fp1: {}", fp1);
    let fp2 = tss.reconstruct(&[0,1,2], &[person_0_product_shares[2], person_1_product_shares[2], person_2_product_shares[2]]);
    println!("fp2: {}", fp2);

    let result = tss.reconstruct(&[0,1,2], &[fp0, fp1, fp2]);
    println!("result {}", result);

}
