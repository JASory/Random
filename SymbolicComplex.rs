/*
   Symbolic Complex vs Numerical
   
   In this example we will show that while numerical representation of 
   Complex numbers requires some forethought in the algorithm of complex multiplication, 
   one can use a more elaborate symbolic representation to model multiplication by definition. 
   
   let P = a+bi and Q = c+di and Z = P*Q
   
   First note the direct polynomial multiplication of (a+b) with (c+d)
   
   ac+ad+bc+bd  
   
   We however cannot use this in the numerical model because we need to account for the
   properties of i. Instead we have to cluster and rearrange
   
   ac-bd, ad+bc
   
   We can however do a similar thing in the symbolic model
   
   (a+bi)(c+di)
   
   ac+adi+bci+bdi^2
   
   
   Note: 1. I will be refering to floats as Reals even though they strictly are not
         2. Symbolic evaluation of Imaginary numbers is unheard of since there is basically no
         advantage. Symbolic evaluation in general however is quite useful. 
         3. This implementation is only partially symbolic, a fully symbolic evaluation is more work than I felt like
   
   
*/

// Abstract model of imaginary number
// A simpler way to model this would be embed the coefficient b in the struct
// But the intent here is to model a complex number as literally as possible
#[derive(Clone,Copy,Debug)]
struct Imaginary;


// Numerical Complex 
// C = a+bi 
// Only the coefficients a, and b are stored here
#[derive(Clone,Copy,Debug)]
struct NumericalComplex{
  a: f64, 
  b: f64,
}

// A structure with a literal Imaginary symbol
#[derive(Clone,Copy,Debug)]
struct SymbolicComplex{
   a: f64,
   b: f64,
   i: Imaginary,
}

// Very simple implementation as described above
// Note that the Self keyword means the same type as whatever is implemented
// Self = NumericalComplex in this case
// Lowercase self refers to the actual structure initialised
impl NumericalComplex{

fn new(a: f64, b: f64) -> Self{
    Self{a,b}
}

  // In: Complex self and other 
  // Out: Complex C = self*other
  fn complex_product(&self, other: Self) -> Self{
      Self::new(
      self.a*other.a-self.b*other.b, // ac-bd
      self.a*other.b+self.b*other.a // ad+bc
      )
  }
}

/*
   A symbolic evaluation is going to be a bit more work
*/

impl Imaginary{

// i*a where a is a Real
// Maps i*a to 0+ai Complex 
  fn prod_real(&self, real: f64) -> SymbolicComplex{
      SymbolicComplex{a: 0f64, b: real, i: Imaginary}
  }
  
  // i^2 = -1
  fn prod_imaginary(&self, other: Self) -> f64{
     return -1f64
  }
}


impl SymbolicComplex{

// Initialise a SymbolicComplex number, not strictly necessary but cleaner for other codes
fn new(a: f64, b: f64) -> Self{
    Self{a,b,i: Imaginary}
}

// P + Q 
fn add(&self, other: Self) -> Self{
   Self::new(self.a+other.a,self.b+other.b)
}

// a*b = a*b + 0i
// Maps product of two Reals to Complex
fn real_product(a: f64, b: f64) -> Self{
    Self::new(a*b,0f64)
}


// ac+adi+bci+bdi^2
fn complex_product(&self, other: Self) -> Self{

// Create our polynomial terms

   // Real, mapped to Complex
   let ac = Self::real_product(self.a,other.a);
   
   // 0+adi
   let adi = self.i.prod_real(self.a*other.b);
   
   // 0+bci
   let bci = self.i.prod_real(self.b*other.a);
   
   // i^2 = -1, so this is -1 * (self.b*other.b) + 0i
   let bdi = Self::real_product(self.i.prod_imaginary(other.i),self.b*other.b);
   
   // Since all of the results have been computed in the Complex field we can simply add them
   // Like in the standard polynomial multiplication algorithm
   
   ac.add(adi).add(bci).add(bdi)
  
}

}

fn main(){
    let p = SymbolicComplex{a:2f64, b: 2f64, i: Imaginary};// or SymbolicComplex::new(2f64,2f64);
    let q = SymbolicComplex{a:2f64, b: 3f64, i: Imaginary};
    let z = p.complex_product(q);
    println!("{:?}", z); 
    
    let s = NumericalComplex::new(2f64,2f64);
    let t = NumericalComplex::new(2f64,3f64);
    let z2 = s.complex_product(t);
    println!("{:?}",z2)

    
}
