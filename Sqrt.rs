fn symbolic_sqrt(mut radicand: u64)->(u64,u64){

    let mut factor = 1u64;                    //factor starts at 1 i.e  1sqrt(18)
    let twofactor = radicand.trailing_zeros(); //strips out the two factor 1sqrt(9)
    radicand>>=twofactor;                      // reduction ^
    
    let mut upperbound = (radicand as f64).sqrt();
    if radicand > 1_000_000u64 {
       upperbound = upperbound.sqrt();
    }
    
    'outer: for i in 1..upperbound as u64 {

       'inner: while radicand%(2*i+1).pow(2) == 0 {
            
         factor*=(2*i+1);    // multiply the exterior factor by 2Z+1
         radicand/=(2*i+1).pow(2);   // divides by (2Z+1)^2
        }
    }
    
    factor<<=(twofactor>>1); // multiply the zeros into the factor 3sqrt(1)
    radicand<<=(twofactor&1); // multiply the remainder back in 3sqrt(2)
    (factor,radicand)
    
}
