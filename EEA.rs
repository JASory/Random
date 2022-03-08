/*
   Extended Euclidean Algorithm and Applications
*/

fn eea(p: i64 , q: i64)->(i64,i64,i64){
         let mut gcd : i64 =p; 
         let mut new_r : i64 =q;
         let mut bezout_1 : i64 =1;
         let mut new_s : i64 =0;
         let mut bezout_2 : i64 = 0;
         let mut new_t: i64 = 1;
    
    while new_r !=0 {
    let quotient =gcd/new_r;
    let mut temp : i64 =new_r;
    new_r=gcd-quotient*temp;
    gcd=temp;
    
    temp=new_s;
    new_s=bezout_1-quotient*temp;
    bezout_1=temp;
    
    temp=new_t;
    new_t=bezout_2-quotient*temp;
    bezout_2=temp;
    
    }
    (gcd,bezout_1,bezout_2)
}


fn mod_pow(x: u64,p: u64 n: u64) -> u64{
   let modpro |x: u64, y: u64, n: u64, n_prime: u64| {
      let t = ((x as u128 *y as u128) as u64;
      let m = (t as u128* n_prime as u128) as u64;
      let u = ((((x as u128*y as u128)>>64) + ((m as u128 + n as u128)>>64))>>64) as u64;
         if u >= n {return u-n}
         return u
   }
   let mut pow = p; 
   let (_ ,r_inv, n_prime) = eea(x,n);
   let mut x_mod = ((x as u128<<64)%(n as u128)) as u64;
   let mut p_mod = ((1u128<<64)%(n as u128)) as u64;
      
   while pow > 1 {
  
   if pow%2 == 0 {
      base =  modpro(x_mod,x_mod,n, n_prime);
      pow>>=1;
   }
  
  else{
  
   p_mod = modpro(x_mod,p_mod,n,n_prime);
   base =  modpro(x_mod,x_mod,n, n_prime);  
   pow=(pow-1)>>1;  
   
 }
      
}
 let fin = modpro(p_mod,x_mod,n,n_prime);
     modpro(fin,1,n,n_prime)
   }      
