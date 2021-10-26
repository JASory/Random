/*
  This algorithm is due to William Kahan
*/

fn kahan(array: Vec<f64>)->f64{
    let mut sum = 0f64;
    let mut c = 0f64;
    
    for i in array{
        let y = i -c;
        let t = sum + y;
        
        c = (t -sum)-y;
        sum = t;
    }
    sum
}
/*
 this algorithm is due to Klein, A.   doi: 10.1007/s00607-005-0139-x
*/

fn kahan_babushka(array: Vec<f64>)->f64{
      let s = 0f64;
      let cs = 0f64;
      let ccs = 0f64;
      
      for i in array{
        let t = s + i;
        let c = 0f64;
        let cc = 0f64;
        if s.abs() >= i.abs(){
          c = (s-t) + i;
        }
        else{
          c = (i-t) + s;
        }
        s = t;
        t = cs + c;
        if cs.abs() >= c.abs() {
           cc = (cs -t) + c;
        }
        else{
         cc = (c-t) + cs
        }
        cs = t;
        ccs = ccs + cc;
      }
      s+cs+ccs
}
