extern crate hex;
extern crate base64;

use std::str;
use std::fs;
use base64::{ decode,decode_config};

fn main() {
  //  let tTexts = File::open("6.txt").expect("Unable to open txt file");
    let file = fs::read_to_string("6.txt").expect("txt error");
    
   let new_file = file.replace("\n","");

    
   let _b64file = decode(&new_file.trim()).expect("test");
  
    let mut current_key_size = 2;

    while current_key_size != 42 
    {
     
     // let _first_slice = &_b64file[1..=current_key_size];c

    
        let iter:Vec<Vec<u8>>  = _b64file.chunks(current_key_size).map(|x| x.to_vec()).collect();
      //skip the first one, since that is the starting slice we are comparing
     
        let mut first = false;
        let mut _first_slice:Vec<u8> =  Vec::new();
        let mut compare_vec:Vec<u8> = Vec::new();
        let mut counter_edit_size:u32 = 0;
        let mut count_loop:u32 = 1;
        for it1 in iter
        {
       
          if(!first)
          {
       
         
              _first_slice=it1;
         
              first = true;
         
          

        } else {
 
          
            compare_vec = it1;

            let first_slice_clone = _first_slice.clone();
            let compare_clone = compare_vec.clone();
            let hamming_distnace = haming_time( first_slice_clone,compare_clone);
            
            
            counter_edit_size += hamming_distnace;
            _first_slice = compare_vec;
            count_loop +=1;
            
        }
          
    
      
     
      
   }


     
        println!("KeySize: {}",current_key_size);
        let final_size = (current_key_size as u32) * count_loop;
        println!("distance: {}",counter_edit_size/final_size);
      

     
      current_key_size +=1;
    
  }
 
}



fn haming_time (start:Vec<u8>,compare:Vec<u8>) -> u32
{
 
  let mut counter = 0;
  for (v,main) in  start.iter().enumerate()
  {
      if(compare.len() > v)
      {
        let new_bytes = main ^ compare[v];
        let all_ones =  new_bytes.count_ones();
        counter = counter + all_ones;
      }
     
     
  }
  counter
}