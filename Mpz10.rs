
const RADIX : u128 = 0x8AC7230489E80000 ;
    
fn carry_add(carry: u64, x: u64, y : u64,  output: &mut u64)->u64{
    let interim =  x as u128 + y as u128 + carry as u128;
    *output = (interim % RADIX ) as u64;
    return (interim / RADIX ) as u64
}

/*
fn subborrow(carry: u64, x: u64, y: u64, output: &mut u64)->u64{

}
*/

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
   digits: Vec<u64>,
}


fn string_format(x: u64)->String{
    let k = x.to_string();
    let leading = (0..(19-k.len())).map(|_| "0").collect::<String>();
    leading + &k
}

impl Mpz{

 fn new(digits: Vec<u64>)->Self{// New 
         Mpz{digits}
     }
     
 fn len(&self)->usize{
     self.digits.len()
 }     
 
 fn format(&self)->String{
     let interim = self.digits[..self.len()-2].iter().rev().map(|x| string_format(*x)).collect::<Vec<String>>();
     self.digits[self.len()-1].to_string() + &interim.join("")
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
  
 let mut z= Mpz{digits: vec![1]};
 
     for i in 1..x+1{
         if i%y == x%y{
            z.self_scalar(i)
         }
     }
       z
    }
   
 
   fn multiply(&mut self,other: &Mpz)->Mpz{
         let mut y= Mpz::new(vec![]);
         let mut offset = 0usize;
        
          for i in other.digits.iter(){
              let mut k = self.scalar(i);
                  y.shift_add(&k,offset);
                  offset+=1;
         }
          y
     }

}
