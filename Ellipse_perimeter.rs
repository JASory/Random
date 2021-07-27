/*
 Various ellipse perimeter functions, including an linear approximation, Gauss-Kummer Series approximation and full evaluation


*/

// Ensure that a >=  b and a !=0   Fastest evaluation in only 4 arithmetic steps, maximum error of 6.9% around eccentricity of 0.28
 fn linear_ellipse(a : f64, b : f64)-> f64{
    a*((b/a*2.283185307179586)+4.0)
 }
 
 
 //Abridged Gauss-Kummer Series
  fn gk_ellipse(a: f64, b: f64)-> f64{
  
     let h  : f64 = ((a-b)*(a-b))/((a+b)*(a+b))*0.25;
     
     3.141592653589793*(a+b)*(1.0 + h + h*h* (0.25 + h*(0.25 + h*0.390625)))
  }
  
  //Full-evaluation using Gauss-Kummer Series, very accurate but inefficient. Translation of Gerard Pichon's Qbasic implementation 
  
  fn ellipse_perimeter(x: f64, y: f64)->f64{
    let mut a = x.abs();
    let mut b = y.abs();
    let mut z = 0f64   ;
    
    if a < b{
       z = a;
       a = b;
       b = z;
    }
    
    let h = ((a-b)/(a+b))*((a-b)/(a+b));
    
    let mut sum = 0.0;
    let mut j = 1.0; 
    let mut k = 0.0;
    
    while sum+j != sum {
    
     k+=1.0;
     j = h*j*((k-1.5)/k)*((k-1.5)/k);
     sum+=j;
    
    }
   (a+b)*(sum+1.0)*3.141592653589793
 }
