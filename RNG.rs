
//Calls the RDRAND instruction, very high quality RNG
 fn hardware_rng()-> u64{
    let mut x: u64 = 0; 
    let k = unsafe { core::arch::x86_64::_rdrand64_step(&mut x) } ;
   x
 }
 
 //Affine RNG, typically called a linear congruential generator this is a non-cryptographic pseudo-random generator
 fn affine_RNG()->u64{
     let mut seed = std::time::Instant::now().elapsed().as_nanos() as u64;
 
  for i in 0..10{
      seed = seed.overflowing_mul(18029154779448018981u64).0.overflowing_add(3935559000370003845u64).0
  }
  seed
 }


// Xorshift 

fn xor32_rng() -> u32{
let mut x = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos() as u32;
  x^=(x<<13);
  x^=(x>>17);
  x^=(x<<5);
  x
}

// Xorshift for 64-bit, due to the low entropy provided by calling system time, the standard 64-bit Xorshift algorithm can't be used
fn xor64_rng() -> u64{
        let (x,y) = (xor_32(),xor_32());
        return unsafe {std::mem::transmute::<(u32,u32),u64>((x,y))};
}

//Full PRNG function for all architectures, useful for non-cryptographic applications
fn rng_64() -> u64 {
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    {   // call RDRAND if possible
        if is_x86_feature_detected!("rdrand"){
            let mut x: u64 = 0;
            unsafe { core::arch::x86_64::_rdrand64_step(&mut x) };
            return x;
        }
        // use xor_64 if x86 but rdrand cannot be called
       return xor_64()
    }

    {// use xor for all other architectures
        xor_64()
    }
}




