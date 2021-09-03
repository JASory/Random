/*
 Precursor functions 
*/

// modular exponentiation
fn modpow(x : u64,mut  pow: u64, modulus: u64)-> u64{  //upgrades to u128 to allow

  let mut z = 1u128;
  let mut base = x.clone() as u128;
  let n = modulus as u128;
  if pow ==0 {
    return z as u64
  }

 while pow > 1 {
  
   if pow%2 == 0 {
      base = base*base % n ;
      pow>>=1;
   }
  
  else{
  
   z = base*z % n;
   base = base*base % n;
   pow=(pow-1)>>1;  
   
 }
 }

 (base*z % n) as u64

}

  // Multiplication and subtraction modulo n
fn mul_sub_mod(x: u64, y: u64,z: u64,  n: u64)->u64{
    ((x as u128 * y as u128 - z as u128 ) % n as u128) as u64
 }

/*
 Functions demonstrating various primality tests
 
    Type            Complexity    Operations for checking 2^64-59    
            
 Trial Division     O(sqrt(n)/3)          1431655766 divisions 
   
 

*/


 fn trial_division(p: u64)-> bool{  //optimized trial division by the 5-rough numbers, aka 6Z-1 ∪ 6Z+1
     if p&1 == 0 && p !=2 || p==1 || ( p%3==0 && p !=3){  // checks for 1,2,and 3 cases
        return  false
     }
    let limit = ((p as f64).sqrt() as u64 +1u64)/6 +1;  // set upper bound as sqrt(p)/6 +1
    for i in 1..limit{
    if p%(6*i-1) == 0 || p%(6*i+1) == 0{// check if divisible by the 5-rough numbers up to the limit
     return false
    }                        // Higher p-rough sets have greater efficiency but diminishing returns 
    }
    return true
 }
 
 
 fn fermat_test(p: u64, base: u64)->bool{// fermat test
     if modpow(base,p-1, p)==1{  // if 2^p-1 mod p = 1 return true as it is a pseudoprime to base
        true                      
     }  
       false            // else return false
    }
    
   
 fn strong_fermat(p: u64, base: u64)->bool{// checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n  
     let zeroes = (p-1).trailing_zeros() as u64; // Breaks number down to p= d*2^n -1
     let d = (p-1)/ (1<<zeroes);
     let mut x = modpow(base,d, p); // base^d mod p
     if x == 1u64 || x==p-1{   // checks if base^p = 1 mod p  or base^(d*2^n)= -1
       return true
       }
    for _ in 0..zeroes-1{// checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
     x = modpow(x, 2, p);
     if x == p-1 {       // if any d*2^zeroes = p-1  then it passes
       return true
     }
    }
    return false        // otherwise it fails
 }
 
 fn miller_rabin(p: u64)->bool{// probabilistic miller rabin (1/4)^5 , skips 2 and 3
    for _ in 0..5{
     if strong_fermat(p,2 + rand()%(p-4)) == false{    // there is no rand function here, write your own or copy from the RNG.rs file is this repository
      return false
     }
    }
    return true
 }
 
 
 fn lucas_lehmer(p: u8)-> bool{// Mersenne primality test
     let mut base = 4u64;
     let mersenne = (1u64<<p) -1; // the p-th mersenne number current max 64
     
     for _ in 0..p-2{
        base = mul_sub_mod(base, base,2, mersenne);
     }
     
     if base == 0{
     true
     }
     false
 }
 
 
