/*

OEIS sequences return vector of sequence 

Ideal ranges of 0;2^64-1, if range is not able to be efficiently met the range is given above the function
Priority given to memory and computational efficiency. 

For some cases LUT (look-up-table) is infact optimal but is omitted due to desire to retain algorithmic implementation details

*/
// bench 0.75s for n = 1E+8
fn kolakaski(limit: usize)->Vec<u8>{
   let mut pushvalue = 0u8;
   let mut sequence = vec![1u8,2u8, 2u8];
   
   for i in  3..limit{
    if i&1 == 0{
    pushvalue = 2
    }
    else {
    pushvalue = 1;
    }
     if sequence[i-1] == 2u8{
        sequence.push(pushvalue);
        sequence.push(pushvalue);
     }
     else{
      sequence.push(pushvalue);
     }
   }
   sequence
}

// Proth numbers iterating k , n is kept static  Proth number  k*2^n +1 while k< 2^n. Limit = element count in this instance
fn proth(n: u64, limit: u64 )-> Vec<u64>{

   let two_factor = 2<<n; 
   let mut sequence = vec![];
   
   for i in 0..limit {
      let k = 2*i+1;
      if k > two_factor{
       break;
      }
      sequence.push(k*two_factor+1)
   }
   sequence  
}
// the factorial
fn factorial(limit: u64)->Vec<u64>{
let start = 1;
let sequence = vec![1];
   for i in 1..limit{
   start*=i;
   sequence.push(start)
   }
   sequence
}

fn fast_collatz(mut x: u64)->bool{ // Fastest possible check for Collatz, if the Collatz condition is not met this function will never return
  x>>=x.trailing_zeros();          // Division by two until the number is odd. This line splits x into  d*2^n and removes the 2^n factor
    while x !=1{              //    
       x=(x<<1)+x+1;               // compiler trick, optimized 3x+1
       x>>=x.trailing_zeros();   // converts the number back to odd if 3x+1 is even
    }
    return true
}

fn count_collatz(mut x: u64)->u32{
   let mut count = x.trailing_zeros();
    x>>=count;
    while x !=1{
       x=(x<<1)+x+1;
       count= count+1+x.trailing_zeros();
       x>>=x.trailing_zeros();
    }
    return count
}
