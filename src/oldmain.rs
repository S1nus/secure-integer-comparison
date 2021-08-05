use threshold_secret_sharing::shamir::ShamirSecretSharing;

fn main() {

    const share_count : usize = 80;

    let tss = ShamirSecretSharing {
        threshold: 1,
        share_count: share_count,
        prime: 7757,
    };

    let mut indices = [0; share_count];
    for i in 0..share_count {
        indices[i] = i;
    }


    /* 
     * Player 1 and Player 2 each have a secret number, X and Y, respectively
     * They wish to determine who's number is bigger without revealing the numbers.
     */

    let x = 250;
    let y = 121;
    let x_shares = tss.share(x);
    let y_shares = tss.share(y);

    /* Assuming X + Y < (P - 1) / 2,
     * X < Y == lsb(2 *( (X - Y) % P ) % P)
     */

    let x_minus_y0 = (x_shares[0] - y_shares[0]) % tss.prime;
    let x_minus_y1 = (x_shares[1] - y_shares[1]) % tss.prime;

    println!("X minus Y = {}", (tss.reconstruct(&[0,1], &[x_minus_y0, x_minus_y1])+tss.prime)%tss.prime);
    println!("should be {}", x-y);

    let times_2_0 = (2 * x_minus_y0)%tss.prime;
    let times_2_1 = (2 * x_minus_y1)%tss.prime;

    println!("times 2: {}", (tss.reconstruct(&[0,1], &[times_2_0, times_2_1])+tss.prime)%tss.prime);
    println!("Should be: {}", 2*((x-y)%tss.prime)%tss.prime);

    /*
     * Now that we have calculated (2 * ((X-Y)%p))%p
     * we can use the bit-decomposition protocol to get the least-signifigant bit
     */

    /*
     * Player 1 and Player 2 must jointly compute a random number, called r
     * This can be accomplished by securely-adding a random number from each player.
     * They also must compute shares of the least-signifigant bit, and securely-compute a share of
     * the least-signifigant bit of r
     */
    let p1_r = 321;
    let p2_r = 27;

    let p1_r_lsb = p1_r % 2;
    let p2_r_lsb = p2_r % 2;

    let p1_r_lsb_shares = tss.share(p1_r_lsb);
    let p2_r_lsb_shares = tss.share(p2_r_lsb);

    let p1_r_shares = tss.share(p1_r);
    let p2_r_shares = tss.share(p2_r);

    let r0 = p1_r_shares[0]+p2_r_shares[0];
    let r1 = p1_r_shares[1]+p2_r_shares[1];

    let r = tss.reconstruct(&[0,1], &[r0, r1]);

    /* 
     * computing the lSB of R
     */
    let sum_of_p1p2_0 = p1_r_lsb_shares[0] + p2_r_lsb_shares[0];
    let sum_of_p1p2_1 = p1_r_lsb_shares[1] + p2_r_lsb_shares[1];
    let sum_of_p1p2_2 = p1_r_lsb_shares[2] + p2_r_lsb_shares[2];
    let sum_of_p1p2_3 = p1_r_lsb_shares[3] + p2_r_lsb_shares[3];

    let prod_p1p2_0 = (p1_r_lsb_shares[0] * p2_r_lsb_shares[0]) % tss.prime;
    let prod_p1p2_1 = (p1_r_lsb_shares[1] * p2_r_lsb_shares[1]) % tss.prime;
    let prod_p1p2_2 = (p1_r_lsb_shares[2] * p2_r_lsb_shares[2]) % tss.prime;
    let prod_p1p2_3 = (p1_r_lsb_shares[3] * p2_r_lsb_shares[3]) % tss.prime;

    let prod2_0 = (2*prod_p1p2_0)%tss.prime;
    let prod2_1 = (2*prod_p1p2_1)%tss.prime;
    let prod2_2 = (2*prod_p1p2_2)%tss.prime;
    let prod2_3 = (2*prod_p1p2_3)%tss.prime;

    let xor_0 = (sum_of_p1p2_0 - prod2_0) % tss.prime;
    let xor_1 = (sum_of_p1p2_1 - prod2_1) % tss.prime;
    let xor_2 = (sum_of_p1p2_2 - prod2_2) % tss.prime;
    let xor_3 = (sum_of_p1p2_3 - prod2_3) % tss.prime;

    println!("LSB: {}", tss.reconstruct(&[0,1,2,3], &[xor_0, xor_1, xor_2, xor_3]));

    println!("R: {}", r);
    println!("R should be: {}", p1_r+p2_r);

    /* C = R + V
     * Where V is the 2*(X-Y) value we previously computed
     */

    let c_shares = vec![
        r0 + times_2_0,
        r1 + times_2_1
    ];

    println!("C: {}", (tss.reconstruct(&[0,1], &[c_shares[0], c_shares[1]])+tss.prime)%tss.prime);
    println!("C should be: {}", r+(tss.reconstruct(&[0,1], &[times_2_0, times_2_1])+tss.prime)%tss.prime);
    
    /* C is revealed to both parties.
     * The least-signifigant-bit of C can be freely observed by both parites
     * we can get the result of the comparison from this like so
     */

    let revealed_c = (tss.reconstruct(&[0,1], &[c_shares[0], c_shares[1]]) + tss.prime) % tss.prime;
    println!("lsb of revealed_c: {}", revealed_c%2);
    if (revealed_c % 2 == 0) {
        println!("its even");
        println!("result is {}", tss.reconstruct(&[0,1,2,3], &[xor_0, xor_1, xor_2, xor_3]));
    }
    else if (revealed_c % 2 == 1) {
        println!("its odd");
        println!("result is {}", 1-tss.reconstruct(&[0,1,2,3], &[xor_0, xor_1, xor_2, xor_3]));
    }
    else {
        println!("revealed C is not a bit");
    }
    println!("Result should be: {}" , if (x > y) { 0 } else { 1 });
}
