extern crate base64;

//use std::str;
use std::fs;
use base64::{ decode};

fn main() {


    let file = fs::read_to_string("6.txt").expect("txt error");
    
    let new_file = file.replace("\n","");
 
     
    let _b64file = decode(&new_file.trim()).expect("test");

   // Terminator X: Bring the noise
    let new_vec = vec![b'T',b'e',b'r',b'm',b'i',b'n',b'a',b't',b'o',b'r',b' ',b'X',b':',b' ',b'B',b'r',b'i',b'n',b'g',b' ', b't',b'h',b'e',b' ',b'n',b'o',b'i',b's',b'e'];

    let mut combined_bytes = Vec::new();    
    let mut counter = 0;
    for (v,main) in  _b64file.iter().enumerate()
    {
           
                let new_bytes = new_vec[counter] ^ main;
                combined_bytes.push(new_bytes);
                counter +=1;
         
                if counter == new_vec.len() {counter = 0;}
    
    
    }


    let rm = String::from_utf8(combined_bytes);

    
    let ret = match rm {
        Ok(v) => v,
        Err(e) =>String::from("Error"),
       
    };

    println!("guess: {}",ret);
}
