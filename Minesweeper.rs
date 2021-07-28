
  // Creates a grid of dimensions x,y. Technically it is infinite, however the coordinates of the places of interest are limited by the datatype size
  //Maximum dimensions of grid are  x & y= 2^32. Works by only allocating the positions of existing numbers, instead imagining that the numbers are on a grid
  struct Grid{                               //like this    {1 ,2 ,3 ,4 ,5 ,
                                             //              6 ,7 ,8 ,9 ,10, 
                                             //              11,12,13,14,15 } 
                                             // If we have a objects of interest at the positions of 4,7, and 13, 
   x : usize,   //x-dimension                // then we simply push 4,7,13 to a vector and ignore the rest as they have one property, non-existence
   y : usize,   //y-dimension                // Likewise the objects of interest have a property of existence and a position. making integers sufficient 
                                             //to represent them 
   plane: Vec<usize>, 
   
 
 }
 
 impl Grid{
    //sets dimensions of grid
    fn new(x: usize, y: usize)->Grid{
      Grid{x ,y , plane: vec![]}
    }
 }
 
 
trait Minesweeper{
    fn creategrid(x: usize,y: usize,minecount: usize)->Grid;
    fn checkposition(&self, pos: usize)->Option<usize>;
}

impl Minesweeper for Grid{

   fn creategrid(x: usize, y: usize, minecount: usize)->Grid{
      
      let mut newgrid = Grid::new(x,y);
      
     for i in 0..minecount {                     //allocates the minecount number of 64-bit integers representing the positions of the mines
      let mut randnum = rand() as usize%(x*y +1);
      while newgrid.plane.contains(&randnum){  //ensures uniqueness of numbers collected
            randnum = rand() as usize%(x*y +1)
      }
      newgrid.plane.push(randnum);
      }
      newgrid.plane.sort();   //sorts the vector to permit optimizations
      newgrid
   }
      //Returns the mine count as a Some() if the position is a mine returns None. 
      //Works by searching the vector for numbers  within the chebyshev distance of 1 from the value selected. 
   fn checkposition(&self, pos: usize)->Option<usize>{
   //simple check to see if the selection is a mine
   if self.plane.contains(&pos){
       return None
   }
   
      let adjacent : Vec<usize>;
    if pos == 0{//top-left                                   
        adjacent = vec![pos+1, pos+self.x, pos+self.x+1];
    }
    else if pos < self.x{//top row
        adjacent = vec![pos-1,pos+1,pos+self.x,pos+self.x-1, pos+self.x+1];
    }
    else if pos == self.x{//top left
        adjacent = vec![pos-1,pos+self.x-1, pos+self.x];
    }
    else if pos%self.x==1{//left column
        adjacent = vec![pos-self.x,pos-self.x+1,pos+1,pos+self.x,pos+self.x+1];
    }
    else if pos%self.x==0{//right column
        adjacent = vec![pos-self.x-1,pos-self.x,pos-1,pos+self.x-1,pos+self.x];
    }
    else if pos ==self.x*(self.y-1){//bottom-left 
        adjacent = vec![pos-self.x,pos-self.x+1,pos+1];
    }
    else if pos > self.x*(self.y-1) && pos < self.x*self.y{//bottom row
        adjacent = vec![pos-self.x-1,pos-self.x,pos-self.x+1,pos-1,pos+1];
    }
    else if pos == self.x*self.y{//bottom-right
        adjacent = vec![pos-self.x-1,pos-self.x,pos-1];
    }
    else {//all other values
        adjacent = vec![pos-self.x-1, pos-self.x, pos-self.x+1, pos-1, pos+1, pos+self.x-1, pos+self.x, pos+self.x+1];
    }
    // interval of values to check
    let interval =  (adjacent[0], adjacent[adjacent.len()-1]);
    let (mut start, mut stop) = (0usize, 0usize);
    //find the index of when the interval starts
    for i in 0..self.plane.len(){
        if self.plane[i] >= interval.0{
            start = i;
            break;
        }
    }
    // find when the interval stops, defaults to self.len() if no end is found
    for j in start..self.plane.len(){
        if self.plane[j] >= interval.1{
            stop = j;
            break;
        }
     if stop ==0{
         stop = self.plane.len()
     }
    }
     //interval to search through, this is the optimization permitted by sorting, instead of having to search through the whole vector we only need to search 
     //an interval of length 2(x+1), resulting in approximately y/2 reduction in the number of elements to check, assuming that the positions are evenly
     //distributed.
    let plane_interval = &self.plane[start..stop]; 
      let mut count = 0usize;
      
      for i in &adjacent{
          for j in plane_interval{
              if i==j{
                  count+=1;
              }
          }
      }
      Some(count)
   }
   
   }//end impl
  
 
 
