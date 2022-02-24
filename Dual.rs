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

 }

 fn sqrt(&self) -> Self{
    Self::new(self.a.sqrt(), self.b/2f64*self.a.sqrt())
 }

 fn nth_rt(&self, n: u32) -> Self{
    
 }

 fn exp(&self) -> Self{

 }

 fn cos(&self) -> Self{

 }

 fn sin(&self) -> Self{
 
 }
 
 fn tan(&self) -> Self{

 }

 fn cosh(&self) -> Self{

 }
 
 fn sinh(&self) -> Self{
  
 }

 fn tanh(&self) -> Self{

 }


 }
