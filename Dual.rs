/*
  Dual numbers. Complex analogs where i^2 = 0
  Note that dual analogs of Real-valued unary functions R can be constructed by R(a), R'(a)b

  Included is an example of using dual numbers to evaluate polynomials plus the derivative simultaneously
*/
#[derive(Clone)]
 struct Dual{
    a: f64,
    b: f64,
 }
 
 
 impl Dual {
 
 fn new(a: f64, b: f64) -> Self{
   Self{a,b}
 }

 fn coef(&self) -> (f64,f64){
   (self.a,self.b)
 }

 fn addition(&self, other: Self) -> Self{
  Self::new(self.a + other.a, self.b + other.b)
 }
 
 fn mut_addition(&mut self, other: Self){
    self.a+=other.a;
    self.b+=other.b;
 }

 fn subtraction(&self, other: Self) -> Self{
  Self::new(self.a-other.a, self.b-other.b)
 }
 
 fn product(&self, other: Self) -> Self{
    Self::new(self.a*other.a, self.a*other.b + other.a*self.b)
 }
 
 fn mut_product(&mut self, other: Self){
    self.b = self.a*other.b + other.a*self.b;
    self.a*=other.a;
 }

 fn sqr(&self) -> Self{
  self.product(self.clone())
 }

 fn pow(&self, n: f64) -> Self{
    Self::new(self.a.powf(n), self.b*n*self.a.powf(n-1f64))
 }

 fn division(&self, other: Self) -> Self{
     Self::new(self.a/other.a, (self.b*other.a - other.b*self.a)/other.a*other.a)
 }

 fn sqrt(&self) -> Self{
    Self::new(self.a.sqrt(), self.b/2f64*self.a.sqrt())
 }

 fn nth_rt(&self, n: u32) -> Self{
    let pow = (n as f64).recip();
    Self::new(self.a.powf(pow), self.b*pow*self.a.powf(pow-1f64))
 }

 fn exp(&self) -> Self{
    let e = 2.718281828f64;
    Self::new(e.powf(self.a), self.b*e.powf(self.a))
 }
   
 fn ln(&self) -> Self{
     Self::new(self.a.ln(), self.b/self.a)
 }
 /*
 fn log(&self, z: f64) -> Self{
   
 }  
*/
 fn cos(&self) -> Self{
    Self::new(self.a.cos(),self.b*(-self.a.sin()))
 }

 fn sin(&self) -> Self{
     Self::new(self.a.sin(),self.b*self.a.cos())
 }
 
 fn tan(&self) -> Self{
    Self::new(self.a.tan(),self.b/self.a.cos()*self.a.cos())
 }

 fn cosh(&self) -> Self{
    Self::new(self.a.cosh(),self.b*self.a.sinh())
 }
 
 fn sinh(&self) -> Self{
      Self::new(self.a.sinh(),self.b*self.a.cosh())
 }
/*
 fn tanh(&self) -> Self{
    Self::new(,)
 }
*/
 }
// Input : coefficient vector in reverse degree order. i.e 3x^2 + 4x + 5 -> 5,4,3  and evaluation point
// Output: P(x), P'(x)
fn polynomial_eval(coef: Vec<f64>, eval: f64) -> (f64,f64){
     let x_point = Dual::new(eval,1f64);
     let mut var_pow = Dual::new(eval,1f64);
     let mut start = Dual::new(coef[0],0f64);
     
     for i in 1..coef.len(){
         start.mut_addition(Dual::new(coef[i],0f64).product(var_pow.clone()));
         var_pow.mut_product(x_point.clone());
     }
     start.coef()
 }
