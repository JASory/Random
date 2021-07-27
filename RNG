
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
