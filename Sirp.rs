fn carry_mul(carry: u64, x: u64, y: &u64, output: &mut u64)->u64{

    let product = (x as u128 * *y as u128) + carry as u128 ;
    let (jtmp,car) = unsafe { std::mem::transmute::<u128,(u64,u64)>(product) };
    
        *output=jtmp;
          car
   }
   
pub fn  scalar(x: &mut [u64], scale : u64)->u64{
      
   let mut carry = 0u64;
   
   for i in x.iter_mut() {
     carry = carry_mul(carry,*i,&scale, i);
   }   
   carry
  }


fn carry_div(carry: u64, x: u64, y: u64, output: &mut u64)->u64{
    let num = unsafe { std::mem::transmute::<(u64,u64), u128>((x,carry)) };
    let factor = num/y as u128;
     *output = factor as u64; 
    (num %y as u128) as u64
}  

   
     // divides inplace and returns remainder
 fn div_slice(x : &mut[u64], divisor: u64 )->u64{
 
 let mut carry = 0u64;
   
   for i in x.iter_mut().rev() {
     carry = carry_div(carry,*i,divisor, i);
   }   
   carry
 }
 
 fn radix(x: &mut[u64])->Vec<u64>{
      let mut k = vec![];
      for i in 0..x.len()+(x.len()/64) {
         k.push(div_slice(x,0x8AC7230489E80000u64))
      }
      let mut count=0usize;
      for i in k.iter().rev(){
       if *i > 0u64{
        break;
       }
       count+=1;
      }
      k.truncate(k.len()-count);
      k
 }


fn string_format(x: u64)->String{
    let k = x.to_string();
    let leading = (0..(19-k.len())).map(|_| "0").collect::<String>();
    leading + &k
}

fn sirp_prime(start: u64, stop: u64, n: u64, residue: u64, factor: u64)->u64{
    let mut count = 0u64;
    for i in start..stop+1{
      if i%n == residue{
        let mut k = i;
         while k % factor == 0{
           k/=factor;
           count+=1;
         }
      }
    }
    count
}


fn sieve(n:usize) -> Vec<usize>{
    let mut primes = vec![true;(n+1) as usize];
    let mut p= 2;
    while p*p <=n {
        if primes[p] {
            let mut index = p*p;
            while index <= n{
                primes[index] =false;
                index += p;
            }
        }
        
        p +=1;
    }
    let mut result =vec![];
    for (i, p) in primes.iter().enumerate() {
       if *p {
           result.push(i);
       }
    }
    result.remove(0);
    result.remove(0);
    result
}





fn count_factor(n: u64, factor: u64)->u64{
    let r = (factor as f64).recip();
    let p = ((n as f64).log10() /(factor as f64).log10()).floor();
    ((n/factor) as f64 * ((1f64 - r.powf(p+1f64))/(1f64-r)) ).floor() as u64
}

fn logfact(n: f64)->f64{
     (n * n.ln() - n + ((n*(1f64 + 4f64*n*(1f64 + 2f64*n))).ln())/6f64 + 3.14159265359f64.ln()/2.0)
 }


#[derive(Clone)]
pub struct Sirp{
  
  infimum : u64, 
  supremum: u64,
  modulo  : u64,
  residue : u64,
  digits  : Vec<u64>,
  factors : Vec<u64>,

}


impl Sirp{

 pub fn unchecked_new(infimum: u64, supremum: u64, modulo: u64, residue: u64)->Self{
      Sirp{infimum, supremum, modulo, residue, digits: vec![], factors: vec![]}
  }
  
  /**
     Ensures that supremum is greater than infimum and residue is valid
  */
 pub  fn new(lower: u64, upper: u64, modulo: u64, residue: u64)->Option<Self>{
         if residue >= modulo {
             return None
         }
         if lower >= upper{
             return None
         }
        Some(Self::unchecked_new(lower, upper, modulo,residue))
        }
        /**
        Generates the digits by successive multiplication. Due to an optimization shortcut this is only accurate for values less than 2^32
        
        */
        
  
pub  fn gen_digits(&mut self){
  
     let mut acc = 1u64;  // accumulator for factors 

     for i in self.infimum..self.supremum+1{ // inclusive range 
     
       if i % self.modulo == self.residue { // if i is of the residue class modulo n then multiply   If you set residue as stop mod n then you get the k-factorials
       
        if self.digits.len() == 0{
            self.digits.push(1)
        }
        if i >= 4294967296 {
         acc = i
        }
       if acc < 4294967296{
          acc*=i
       }
       if acc >= 4294967296{
        let mut carry = 0u64;
           carry = scalar(&mut self.digits[..],acc);

          if carry > 0 {
       
            self.digits.push(carry)
      
          }
         acc= 1u64;
       } // end if 
      } // else
     }
     
     let carry = scalar(&mut self.digits[..],acc);
     if carry > 0{
        self.digits.push(carry)
     }
     } 
  /**
   
     Converts the stored digits into a radix-10 string
  
  */
  
pub  fn digits_string(&self)->String{
  
      let mut z = self.digits.clone();
      if z.len() ==0{
         return "".to_string()
      }
      let start = std::time::Instant::now();
      let k = radix(&mut z[..]);
      let stop = start.elapsed();
        let len = k.len()-1;
        let interim = k[..len].iter().rev().map(|x| string_format(*x)).collect::<Vec<String>>();
        k[len].to_string() + &interim.join("")
  } 
  /**
     Exact factorization stored into the struct
  */
  
pub  fn factor(&mut self){
        let primes =  sieve(self.supremum as usize);   
        
     for i in primes{
       let n = sirp_prime(self.infimum, self.supremum, self.modulo, self.residue, i as u64);
        if n !=0{
         self.factors.push(i as u64);
         self.factors.push(n as u64);
      }
    }
  }
  
  /**
      Fast approximation of factorization. Zero guarantee of accuracy
  */
  
    // correct to push all values 
pub  fn fast_factor(&mut self){
      let primes =  sieve(self.supremum as usize);
     
      for i in primes{
       let factors = ((count_factor(self.supremum, i as u64) - count_factor(self.infimum, i as u64))/self.modulo);
        if factors > 0u64{
          self.factors.push(i as u64);
          self.factors.push(factors);
        }
      }
  }
  
  /**
     Converts stored factor into a string in the form "2^n * 3^n" ... 
  */
  
pub  fn factor_string(&self)->String{
      let mut factorstring = vec![];
      for i in 0..(self.factors.len()/2)-1{
       if self.factors[i*2 +1] == 1{
        factorstring.push(self.factors[i*2].to_string());
      }
      else{
        factorstring.push(self.factors[i*2].to_string() + "^" + &self.factors[i*2+1].to_string());
      }
      
      }
      factorstring.join(" * ")
  }
  
  
  
  /// Linear time near-exact approximation, unless 1-factorial then constant-time evaluation
pub  fn log(&self, base: f64)->f64{
      if  self.modulo == 1 && self.residue == 0{
        return  (logfact(self.supremum as f64)/base.ln())/self.modulo as f64 - (logfact(self.infimum as f64 + 1f64)/base.ln())/self.modulo as f64
      } 
      if self.factors.len() > 0usize { // if factorized then use generated factors
           let mut sum = 0f64;
           
           for i in 0..(self.factors.len()/2)-1{
             sum+=(self.factors[i*2] as f64).log(base)*self.factors[i*2 +1] as f64;
           }
           return sum      
      }
      let mut sum = 0f64;
        for i in self.infimum..self.supremum+1{
          if i%self.modulo == self.residue{
            sum+=(i as f64).log(base)
          }
        }
        return  sum
     }
     
     /// Constant-time approximation. 
pub  fn fast_log(&self, base: f64)->f64{
       if self.residue ==  self.supremum%self.modulo && (self.modulo != 1){
          return (logfact(self.supremum as f64)/base.ln() )/self.modulo as f64  + ((self.supremum as f64).ln()/10f64.ln())/self.modulo as f64
       }
       (logfact(self.supremum as f64)/base.ln())/self.modulo as f64 - (logfact(self.infimum as f64 + 1f64)/base.ln())/self.modulo as f64
     }
  
      
       /// Number of times the exact factor is multiplied, not including composition of factors
  fn count_factor(&self, factor: u64)->u64{
       let mut zeros = 0u64;
       
        for i in self.infimum..self.supremum+1{ // inclusive range 
     
          if i % self.modulo == self.residue { 
            let mut k = i;
           while k % factor == 0{
              k/=factor;
              zeros+=1;
           }
          }
       
       }
       zeros 
  }
    
  
  /// Number of zeros in radix-10 representation 
  fn zeros(&self)->u64{
      if self.infimum == 1 && self.modulo == 1 && self.residue == 0{
         let mut count = 0u64;
         let mut k = self.supremum;
         while k > 0{
         count+=k/5u64;
         k/=5u64;
         }
         return count
      }
  
       std::cmp::min(self.count_factor(2), self.count_factor(5))
  }
  /** Converts the digits to a vector of u32. Solely for conversion of Sirp to Bigint in the num-bigint crate like below. As this crate has no dependencies it this requires including both sirp and num-bigint as dependencies
  ```
  use num_bigint::BigUint;
  
    let factorial100 = Sirp::new(1,100,1,0).unwrap();
    
    factorial100.gen_digits();  
    
    let vector = factorial100.to_u32();
    
    let bigfactorial = BigUint::new(vector)
    
  ```
  */
  pub fn to_u32(&self)->Vec<u32>{
       let mut u32_vec = vec![];
       for i in self.digits.iter(){
         let (lower,upper) = unsafe { std::mem::transmute::<u64,(u32,u32)>(*i) };
         u32_vec.push(lower);
         u32_vec.push(upper);
       }
       u32_vec
  } 
 
} 
