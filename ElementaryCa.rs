/*
Precursor functions including the rules for some Elementary Cellular automata

*/

// p = x.0 , r= x.2  q = x.1
fn rule30(x:(bool,bool,bool))->bool{
   x.0^(x.1|x.2)
}

fn rule110(x: (bool,bool,bool))->bool{
   !x.0 & x.1 & x.2 ^x.1 ^x.2
}

fn rule90(x:(bool,bool,bool))->bool{   
x.0^x.2
}

fn rule137(x: (bool, bool, bool))->bool{
   (!x.0|x.1|x.2)^x.1^x.2
}
fn rule105(x:(bool,bool,bool))->bool{
    x.0 ^ x.1 ^ !x.2
}

// formatting for printing to terminal
fn symbol(x: bool)->String{
    if x {
       return "#".to_string()
    }
   
      return  " ".to_string()
}


#[derive(Clone)]
struct DenseVector{// Bitvector/Bitmatrix  for dense boolean matrices
 elements: Vec<u64>,
        x: usize,
        y: usize,
}
   
 impl DenseVector{

  fn new(elements: Vec<u64>, x: usize, y:usize)->Self{
     DenseVector{elements, x, y}
 }
    
 
 fn set_random(&mut self){
     for i in self.elements.iter_mut(){
        unsafe { core::arch::x86_64::_rdrand64_step(i);}
     }
 }
 
 fn idx_2d(&self, x: usize, y: usize)->Option<usize>{//2d to 1d index, combine with unchecked
    if x > self.x || y > self.y{
     return None;
    }
    Some(self.x*x + y)
 }
 
 fn unchecked_idx(&self, idx: usize)->bool{
    (self.elements[idx>>6]>>(idx&63))&1 != 0
 }
 
 fn idx(&self, x: usize, y:usize)->Option<bool>{
    match self.idx_2d(x,y){
    Some(k) => return Some(self.unchecked_idx(k)),
    _ => return None,
    }
 }
 
 fn unchecked_flip(&mut self, idx: usize){
    self.elements[idx>>6]^=(1<<(idx&63));
 }
 
 fn flip(&mut self, x: usize, y: usize){
    match self.idx_2d(x,y) {
     Some(k) => self.unchecked_flip(k),
     None    => (),
    }
 }
 
 fn unchecked_set(&mut self, idx:usize, value: bool){
    if self.unchecked_idx(idx)!=value{
         self.unchecked_flip(idx)
     }
 }
 
  fn set(&mut self, x: usize, y: usize, value: bool){
      match self.idx_2d(x,y) {
      Some(k) => if self.unchecked_idx(k) != value { self.unchecked_flip(k)},
      None => (),
      }
 }
 
 fn unchecked_adjacent(&self, idx: usize)->(bool, bool, bool){  //1d neighborhood
    if idx == 0{
    (false,self.unchecked_idx(0), self.unchecked_idx(1) )
    }
    else if idx == self.x-1{
     (self.unchecked_idx(idx-1),self.unchecked_idx(idx), false )  
    }
    else{
    (self.unchecked_idx(idx-1),self.unchecked_idx(idx), self.unchecked_idx(idx+1) ) 
    }
 }
 
 fn adjacent(&self, x: usize, y: usize)->Option<(bool, bool, bool)>{
    match self.idx_2d(x,y){
    Some(k) => return Some(self.unchecked_adjacent(k)),
    None    => return None,
    }
 }
 
 fn unchecked_chebyshev_m1(&self, idx: usize)->u8{// 2d neighborhood
    if idx == 0 {// top-left
       ((self.unchecked_idx(1) as u8)<<4) |  
       ((self.unchecked_idx(self.x) as u8)<<6) |
       ((self.unchecked_idx(self.x+1) as u8)<<7)
    }
   else if  idx < self.x-1{// top row
       ((self.unchecked_idx(idx-1) as u8)<<3) |
       ((self.unchecked_idx(idx+1) as u8)<<4) |
       ((self.unchecked_idx(idx+self.x-1) as u8)<<5) |
       ((self.unchecked_idx(idx+self.x) as u8)<<6) |
       ((self.unchecked_idx(idx+self.x+1) as u8)<<7) 
    }
  else if idx == self.x-1{// top-right
         ((self.unchecked_idx(idx-1) as u8)<<4) |
         ((self.unchecked_idx(idx+self.x-1) as u8)<<5) |
         ((self.unchecked_idx(idx+self.x) as u8)<<6) 
    }
    
      
   else if idx == self.x*(self.y-1){// lower-left
         ((self.unchecked_idx(idx-self.x) as u8)<<1) |
         ((self.unchecked_idx(idx-self.x+1) as u8)<<2) |
         (self.unchecked_idx(idx+1) as u8)<<4 
     }
  else if idx == self.x*self.y-1{// lower-right
         ((self.unchecked_idx(idx-self.x-1) as u8)) |
         ((self.unchecked_idx(idx-self.x) as u8)<<1) |
         (self.unchecked_idx(idx-1) as u8)<<3 
     }   
     
   else if idx > self.x*(self.y-1){//lower row
         (self.unchecked_idx(idx-self.x-1) as u8) |
         ((self.unchecked_idx(idx-self.x) as u8)<<1) |
         ((self.unchecked_idx(idx-self.x+1) as u8)<<2) |
         ((self.unchecked_idx(idx-1) as u8)<<3) |
         ((self.unchecked_idx(idx+1) as u8)<<4) 
     }
     
     else if idx%self.x == 0 {// left column
         ((self.unchecked_idx(idx-self.x) as u8)<<1) |
         ((self.unchecked_idx(idx-self.x+1) as u8)<<2) |
         ((self.unchecked_idx(idx+1) as u8)<<4) |
         ((self.unchecked_idx(idx+self.x) as u8)<<6) |
         ((self.unchecked_idx(idx+self.x+1) as u8)<<7) 
         
     } 
     else if idx%self.x == self.x-1{//right column
         (self.unchecked_idx(idx-self.x-1) as u8) |
         ((self.unchecked_idx(idx-self.x) as u8)<<1) |
         ((self.unchecked_idx(idx-1) as u8)<<3) |
         ((self.unchecked_idx(idx+self.x-1) as u8)<<5) |
         ((self.unchecked_idx(idx+self.x) as u8)<<6) 
     }
     
     else{// center
         (self.unchecked_idx(idx-self.x-1) as u8) | 
         ((self.unchecked_idx(idx-self.x) as u8)<<1) | 
         ((self.unchecked_idx(idx-self.x+1) as u8)<<2) | 
         ((self.unchecked_idx(idx-1) as u8)<<3) |
         ((self.unchecked_idx(idx+1) as u8)<<4) | 
         ((self.unchecked_idx(idx+self.x-1) as u8)<<5) |
         ((self.unchecked_idx(idx+self.x) as u8)<<6) | 
         ((self.unchecked_idx(idx+self.x+1) as u8)<<7)  
     }  
 }
 
 fn chebyshev_m1(&self, x: usize, y: usize)->u8{
    match self.idx_2d(x,y){
    Some(k) => return self.unchecked_chebyshev_m1(k),
    None    => return 0u8,
 }
 }
 
 fn print(&self){
  for i in 0..self.y{
   for j in 0..self.x{
   print!("{}  ",symbol(self.unchecked_idx(self.x*i+j)))
   }
   println!("");
 }
 }
 
}// end implement  
   
 struct ElementaryCA{
        elements: DenseVector,
               x: usize,
               y: usize,
}
   
   impl ElementaryCA{

 fn new(x:usize, y: usize)->ElementaryCA{// Creates grid of the dimensions given
 let elements = DenseVector::new(vec![0u64;(x*y/64)+1],x,y);
    ElementaryCA{elements,x,y}
 }
 
 fn set_random(&mut self){// sets all the elements randomly
    self.elements.set_random();
 }
 
 fn set(&mut self, idx: usize){
    self.elements.unchecked_set(idx,true);
 }
 
 
 fn rule(&mut self, rule: fn((bool,bool,bool))->bool, evolutions: usize){// Elementary Automata rules
    for _ in 0..evolutions{
    let mut interim = self.elements.clone();
      for i in 0..self.x*self.y{
        interim.unchecked_set( i,rule(self.elements.unchecked_adjacent(i) ) );
      }
    self.elements = interim;
    }
 }
 
 
 fn gol(&mut self, evolutions: usize){// Conway Game of Life with step count
 
 for _ in 0..evolutions{
     let mut interim = self.elements.clone();
     
     for i in 0..self.x*self.y{
       let adjacents = self.elements.unchecked_chebyshev_m1(i).count_ones();
       
       if self.elements.unchecked_idx(i) == false && adjacents == 3{ // reproduces
         interim.unchecked_flip(i);
      }
      else if self.elements.unchecked_idx(i) == true && (adjacents != 2  && adjacents != 3){
           interim.unchecked_flip(i);
      }
       
     }
     self.elements = interim;
   }
  }
  
  fn print(&self){
     self.elements.print();
  }

}
   
