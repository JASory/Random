/*
    Conversion functions from non-standard units to SI units
    
    To reverse most of these conversions one can simply have n*conversion_func(1f64).recip() 
   where n is the value to be converted. 
     
*/

   /*
   
       Length to SI units
   
   */
   
                                   // Inches to metre
  pub fn  in_m(a: f64) -> f64{
          a*0.0254
  }
                                   // Foot to metre
  pub fn  ft_m(a: f64) -> f64{
          a*0.3048
  }
                                   // Yard to metre
  pub fn  yd_m(a: f64) -> f64{
          a*0.9144
  }
                                      //Statue miles to km 
  pub fn  mi_km(a: f64) -> f64{
          a*1.609344
  }
                                       //Nautical miles to km
  pub fn  nm_km(a: f64) -> f64{
          a*1.852
  }
  
  /*
  
    Area to SI
  
  */
                                      
  pub fn  in2_m2(a: f64) -> f64{
          a*0.00064516
  }                            
         
  pub fn  ft2_m2(a: f64) -> f64{
          a*0.092903
   }
                                 // yards^2 to m^2
  pub fn  yd2_m2(a: f64) -> f64{
          a*0.8361273
  }
                                  // miles^2 to km^2
  pub fn  mi2_km2(a: f64) -> f64{
          a*2.5899881
  }
                                   // nautical miles^2 to km^2
  pub fn  nm2_km2(a: f64) -> f64{
          a*3.429904
  }       
                                    // acres to km^2
  pub fn  acre_km2(a: f64) -> f64{
          a*0.004047
  }
                                     // hectares to km^2
  pub fn  hct_km2(a: f64) -> f64{
          a*0.01
  }        
   /*
           
        Volume to SI units   
   */
                                      
                                    // Inches^3 to cm^3  
  pub fn  in3_m3(a: f64) -> f64{
          a*0.000016387064
  }
                                    // feet^3 to m^3
  pub fn  ft3_m3(a: f64) -> f64{
          a*0.0283168
  }
                                    // yards^3 to m^3
  pub fn  yd3_m3(a: f64) -> f64{
          a*0.7645548
  }
                                     //fluid oz to liters
  pub fn  oz_l(a: f64) -> f64{
          a*0.0295735
  }        
                                       //gallons to liters
  pub fn  gal_l(a: f64) -> f64{
          a*3.78541
  }
                                        //grains to kilograms
  pub fn  grn_kg(a: f64) -> f64{
          a*0.0006479891
  }
                                       //carat to kilograms
  pub fn  crt_kg(a: f64) -> f64{
          a*0.002
  }
                                    //Avoirdupois pound to kilogram
  pub fn  lb_kg(a: f64) -> f64{
          a*0.4535924
  }        
                                   //short ton to metric ton
  pub fn  tn_mt(a: f64) -> f64{
          a*0.907185
  }
  
/*                
         Energy conversions
*/ 
                                    //Fahrenheit to Kelvin
  pub fn  f_klv(a: f64) -> f64{
          (a-32.0)*0.55555+273.15
  }
                                  // Celsius to Kelvin    Inverse is a + c_klv(0f64)
  pub fn  c_klv(a: f64) -> f64{
           a-27
  }
                                   //Foot-pounds to Joules
  pub fn  fts_jl(a: f64) -> f64{
          a*1.35518
  }
                                  //BTU to kilocalories
  pub fn  btu_kc(a: f64) -> f64{
          a*0.252164
  }
                                     //BTU to joules
  pub fn  btu_jl(a: f64) -> f64{
          a*1055.06
  }
                                     //Curie to Becquerel
  pub fn  cu_bql(a: f64) -> f64{
          a*3.7E+10
  }
                                    //Dyne to newton
  pub fn  dyn_nw(a: f64) -> f64{
          a*1E-5
  }
                                    //Horsepower to watt
  pub fn  hp_wt(a: f64) -> f64{
          a*735.499
  }
                                    //erg to Joule
  pub fn  erg_jl(a: f64) -> f64{
          a*1E-7
  }         
                                     //electronvolt to Joule
  pub fn  ev_jl(a: f64) -> f64{
          a*1.60218E-19
  }
                                    //Joule to electronvolt
  pub fn  jl_ev(a: f64) -> f64{
          a*6.242E+18
  }
          
   /*
   
        Astronomical units
   
   */       
          
                                    // Astronomical unit to km
  pub fn  au_km(a: f64) -> f64{
          a*149597870.7
  }
                                     //lightyear to kilometer
  pub fn  ly_km(a: f64) -> f64{
          a*9460730472580.8
  }
                                     //Parsec to km
  pub fn  pc_km(a: f64) -> f64{
          a*30856775814913.673
  }
                                     //Solar radii to km
  pub fn  sor_km(a: f64) -> f64{
          a*695700.0
  }
                                    //Solar mass to kg
  pub fn  som_kg(a: f64) -> f64{
          a*1.98847E+30
  }
                                     //Earth mass to kilograms
  pub fn  ert_kg(a: f64) -> f64{
          a*5.974E+24
  }
          
   /*
   
       Time 
   
   */       
          
                                       //hour to seconds
  pub fn  hr_s(a: f64) -> f64{
          a*3600.0
  }
                                       //day to seconds   
  pub fn  dy_s(a: f64) -> f64{
          a*86400.0
  }    
                                       // mean tropical year to seconds
  pub fn  yr_s(a: f64) -> f64{
          a*31556736.0
  }
                                       // Julian year to seconds
  pub fn  jlyr_s(a: f64) -> f64{
          a*31557600.0
  }

/*
            Planck unit conversions
*/                                          
                                    
                                    //Meter to Planck-length Gaussian
  pub fn  si_plg(a: f64) -> f64{
          a*6.18792353E+34
  }
                                   //Meter to Planck-length Lorentz-H
  pub fn  si_pll(a: f64) -> f64{
          a*1.745505324E+34
  }
  
/*
    Atomic units 
*/           
                                    //Dalton to kg
  pub fn  dalt_kg(a: f64) -> f64{
          a*1.66054E-27
  }  
  
/*
    Physics conversions
*/                           
   
                                    //Frequency to wavelength metres
  pub fn  hz_wl(a: f64) -> f64{
          a*3.33564095198152E-9
  } 
  


  /*
  
     Prefix conversions
  
  */

  pub fn yota_base(a: f64){
             a*1E+24 
             } 
             
  pub fn zeta_base  (a : f64){
             a*1E+21
             }
             
  pub fn exa_base(a : f64){
             a*1E+18
             }
       
  pub fn peta_base(a : f64){
             a*1E+15
             } 
             
  pub fn tera_base(a : f64){
             a*1E+12
             }
 
  pub fn giga_base(a : f64){
             a*1E+9
             }    
             
  pub fn mega_base(a : f64){
             a*1E+6
             }                  
   
  pub fn kilo_base(a : f64){
             a*1E+3
             }  
   
  pub fn mill_base(a : f64){
             a*1E-3
             }  
             
  pub fn micr_base(a : f64){
             a*1E-6
             }  
             
  pub fn nano_base(a : f64){
             a*1E-9
             }  
             
  pub fn pico_base(a : f64){
             a*1E-12
             }  
             
  pub fn femt_base(a : f64){
             a*1E-15
             }  
             
  pub fn atto_base(a : f64){
             a*1E-18
             }  
             
  pub fn zept_base(a : f64){
             a*1E-21
             }  
             
  pub fn yoct_base(a : f64){
             a*1E-24
             }                                                                                                                 
