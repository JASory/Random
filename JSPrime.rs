/*
   A simple Fast deterministic primality test, mostly intended to be a balance between extremely optimized tests like RCPrime and 
naive trial division tests. Generally faster than most deterministic tests and roughly the same speed on average as the naive 
hashtables as produced by Forisek, Jancina and Bradley Berg. Currently approximately 4 times slower than RCPrime/ENT implementation. 
   
   Example benchmark on i3-4005U; all primes counted in 0;2^32
   
   203280221 primes counted in  2185.546162445s  

*/


  // first primes and factors of bases
 const PRIMES : [u64;46] = [
    2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71,
   73, 79, 83, 89, 97, 101, 103, 107, 109, 113,127, 131, 137, 139, 149, 151, 157, 163, 167, 173,
   179, 181, 191,193,407521,299210837
 ];

fn modpow(x : u64,mut  pow: u64, modulus: u64)-> u64{// Modular Exponentiation

  let mut z = 1u128;
  let mut base = x.clone() as u128;
  let n = modulus as u128;

 while pow > 1 {
   if pow&1 == 0 {
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

fn strong_fermat(p: u64, base: u64)->bool{// checks if base^p = 1 mod p  or base^(d*2^n)= -1 for some n
     let p_minus = p-1;
     let zeroes = p_minus.trailing_zeros() as u64; // Breaks number down to p= d*2^n -1
     let d = p_minus>>zeroes;
     let mut x = modpow(base,d, p); // base^d mod p
     if x == 1u64 || x==p_minus{   // checks if base^p = 1 mod p  or base^(d*2^n)= -1
       return true
       }
    for _ in 0..zeroes-1{// checks for all d*2^zeroes. One is subtracted since d*2^n was already checked above
     x = ((x as u128 * x as u128)%p as u128) as u64;// modular squaring
     if x == p_minus {       // if any d*2^zeroes = p-1  then it passes
       return true
     }
    }
    return false        // otherwise it fails
 }
 
 fn jsprime(x: u64) -> bool{
  if x == 1{
    return false
  }
  
 for i in PRIMES[..].iter(){// Returns true if prime, returns false if divisible by prime
    if x==*i{
      return true
    }
    if x%i==0{
     return false
    }
  }
 
  for i in [2,325,9375,28178,450775,9780504,1795265022]{// Checks using the preselected bases
    if strong_fermat(x,i)==false{
     return false
    }
  }
  return true
 }
