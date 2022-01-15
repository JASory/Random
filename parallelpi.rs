use number_theory::traits::NumberTheory;
//203280221 primes counted in t : 264.202911824s

fn pi_interval(lower: u64, upper: u64) -> u64{

  let mut count = 0u64;
    
    //let start = std::time::Instant::now();
    
    for i in lower..upper{
     if i.is_prime(){
      count+=1;
     }
    }
    //let stop = start.elapsed();
    //println!("{} primes counted in t : {:?}",count,stop);
    count
}

fn pi_parallel(lower: u64, upper: u64) -> u64{
  let delta = (upper -lower)/4u64;
  let low = std::thread::spawn(move || {pi_interval(lower,lower + (delta))});
  let mid_low = std::thread::spawn(move|| {pi_interval(lower + delta, lower + (delta*2))} );
  let mid_hi  = std::thread::spawn(move||{pi_interval(lower + (delta*2), lower + delta*3)});
  let hi      = std::thread::spawn(move||{pi_interval(lower + delta*3, upper)});
  
  let total = low.join().unwrap() + mid_low.join().unwrap() + mid_hi.join().unwrap() + hi.join().unwrap();
  return  total

}
// 2712103833 computed in 7997.472779281s



fn main() {
    for i in 40..41{
       let start = std::time::Instant::now();
       let count = pi_parallel(1<<i,1<<(i+1)); //2712103833 computed in 7997.472779281s

       let stop = start.elapsed();
    println!("{} computed in {:?}", count,stop)
}
}
