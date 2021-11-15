#[derive(Clone, Copy,Debug, PartialEq)]
pub enum Sign{
   Positive,
   Negative,
}

 impl Sign {
   
  pub fn neg(&self)->Self{
      match self {
      Sign::Positive =>  Sign::Negative,
      Sign::Negative =>  Sign::Positive, 
      }
   }
   
 pub fn mul(&mut self, other: &Self)->Self{
       match (&self, other) {
         (&Sign::Positive, &Sign::Negative)=>  Sign::Negative,
         (&Sign::Negative, &Sign::Positive)=>  Sign::Negative,
                                         _=>   Sign::Positive,
       }
   }
 pub fn pow(&self, pow: &u64)-> Sign{
        if self == &Sign::Negative && pow%2 ==1{
           return Sign::Negative
        }
        else{
            Sign::Positive
        }
      }  
 }
 
 

const RADIX : u128 = 0x8AC7230489E80000 ;
    
fn carry_add(carry: u64, x: u64, y : u64,  output: &mut u64)->u64{
    let interim =  x as u128 + y as u128 + carry as u128;
    *output = (interim % RADIX ) as u64;
    return (interim / RADIX ) as u64
}

  // x-y 
fn subborrow(carry: u64, x: u64, y: u64, output: &mut u64)->u64{
        let subtrahend = y + carry;
     if  subtrahend > x {
          *output = RADIX as u64 -(subtrahend - x);
          1u64
     }
     else{
       *output = x -y;
       0u64
     }
}


fn carry_mul(carry: u64, x: u64, y: u64, output: &mut u64)->u64{
     let interim =  x as u128 * y as u128 + carry as u128;
      *output = (interim % RADIX ) as u64;
    return (interim / RADIX ) as u64
}


fn carry_div(carry: u64, x: u64, y: u64, output: &mut u64)->u64{
     *output = x / RADIX as u64;
     x % RADIX as u64
}



fn unequal_add(x: &mut [u64],y: &[u64])->u64{  //first number must be larger

    let mut carry = 0u64;
 
    let (lo,hi) = x.split_at_mut(y.len()); //split to make equal
    
    for (i,j) in lo.iter_mut().zip(y.iter()){                               //add equal 
            carry = carry_add(carry,*i,*j,i)
        }
      
      if carry > 0u64{
       
       for k in hi.iter_mut(){   //add the carry until there is no carry
       carry = carry_add(carry,*k,0u64,k);
       if carry == 0u64{
       break;
       }
       }
      
      }
   carry
 }
 
 fn offset_add(x: &mut[u64],y: &mut[u64], offset: usize)->u64{  //adds the second value to the first, shifted by offset

    let (lo,hi) = x.split_at_mut(offset);
 //uint/shift.rs
     unequal_add(hi,y)
 }
 
 
 fn unequal_sub(x: &mut [u64],y: &[u64])->u64{  //first number must be larger

 let mut carry = 0u64;
 
 let (lo,hi) = x.split_at_mut(y.len()); //split to make equal
    
 for (i,j) in lo.iter_mut().zip(y.iter()){                               //add equal 
        carry = subborrow(carry,*i,*j,i)
       }
      
      if carry > 0u64{
       
       for k in hi.iter_mut(){   //add the carry until there is no carry
       carry = subborrow(carry,*k,0u64,k);
       if carry == 0u64{
       break;
       }
       }
      
      }
   carry

}
 
 fn  scalar_slice(x: &mut [u64], scale : u64)->u64{
      
   let mut carry = 0u64;
   
   for i in x.iter_mut() {
     carry = carry_mul(carry,*i,scale, i);
   }   
   carry
  }
  
   // divides inplace and returns remainder
 fn div_slice(x : &mut[u64], divisor: u64 )->u64{
 
 let mut carry = 0u64;
   
   for i in x.iter_mut().rev() {
     carry = carry_div(carry,*i,divisor, i);
   }   
   carry
 }

#[derive(Clone)]
struct Mpz{
   sign  : Sign,
   digits: Vec<u64>,
}


fn string_format(x: u64)->String{
    let k = x.to_string();
    let leading = (0..(19-k.len())).map(|_| "0").collect::<String>();
    leading + &k
}

impl Mpz{

 fn new(sign: Sign , digits: Vec<u64>)->Self{// New 
         Mpz{sign, digits}
     }
     
 fn len(&self)->usize{
     self.digits.len()
 }     
 
  fn less_than(&self,other: &Mpz)-> bool{
    if self.len() > other.len(){
       return false
    }
   for (x,y) in self.digits.iter().rev().zip(other.digits.iter().rev()){
      if x > y{
        return false
      }
   } 
  return true
 }
 
 
 fn format(&self)->String{
     let interim = self.digits[..self.len()-1].iter().rev().map(|x| string_format(*x)).collect::<Vec<String>>();
     println!("{:?}",interim);
     self.digits[self.len()-1].to_string() + &interim.join("")
 }
 
 
 
 fn add_assign(&mut self, mut other: Self){
 
      let mut carry = 0u64;
 
     if self.sign == other.sign {
        if &self.len() < &other.len(){
 
            self.digits.extend_from_slice(&other.digits[self.len()..])
        }
     carry = unequal_add(&mut self.digits[..],&other.digits[..]);
     }
     
    else{
       if self.less_than(&other){
             carry = unequal_sub(&mut other.digits[..],&self.digits[..]);
             *self = other;
        }
       else {
             carry = unequal_sub(&mut self.digits[..],&other.digits[..]);
        }
    }
    
     if carry == 1u64{
          self.digits.push(1u64)
     }
   
 }

 fn shift_add(&mut self, mut other: &Self, shift: usize){
   
   
       self.digits.resize(other.len()+shift,0);
   
       let mut carry : u64 =0;
       
       
       for i in 0..other.digits.len(){
        carry = carry_add(carry,self.digits[i+shift],other.digits[i],&mut self.digits[i+shift])}
       
   
     if carry > 0u64{
                   self.digits.push(1u64)
    }
    
   }
   
  pub fn self_scalar(&mut self, x: u64){
      let  carry = scalar_slice(&mut self.digits[..],x);
     if carry > 0u64{
         self.digits.push(carry);
     }
  }
  
  pub fn scalar(&self, x: &u64)->Mpz{
      let mut k = self.clone();
      k.self_scalar(*x);
      k
  }
  
  pub fn self_div(&mut self, x: u64)->u64{
          div_slice(&mut self.digits[..],x)
  }
  
  pub fn word_div(&self,x: u64)->(Self, u64){
       let mut quotient = self.clone();
       let remainder = quotient.self_div(x);
       (quotient,remainder)
  }
  
  
  pub  fn k_factorial(x: u64, y:u64)-> Mpz{
  
 let mut z= Mpz::new(Sign::Positive, vec![1]);
 
     for i in 1..x+1{
         if i%y == x%y{
            z.self_scalar(i)
         }
     }
       z
    }
   
 
   fn multiply(&mut self,other: &Mpz)->Mpz{
         let mut y= Mpz::new(self.sign.mul(&other.sign), vec![]);
         let mut offset = 0usize;
        
          for i in other.digits.iter(){
              let mut k = self.scalar(i);
                  y.shift_add(&k,offset);
                  offset+=1;
         }
          y
     }
     
     pub  fn pow(&self, mut y:u64)->Mpz{
         let mut z= Mpz::new(self.sign.pow(&y),vec![1]);
         let mut x_mpz = self.clone();
          if y == 0{
              return z
          }
         while y > 1{
    
           if y%2 ==0 {
              x_mpz = x_mpz.multiply(&x_mpz.clone());
              y>>=1;
           }
           else{
              z = x_mpz.multiply(&z);
          x_mpz = x_mpz.multiply(&x_mpz.clone());
              y = (y-1)>>1;
           }
        }
      return x_mpz.multiply(&z)
     }
     
   // fn sqrt 

}
