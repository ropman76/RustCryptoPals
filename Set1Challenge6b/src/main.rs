extern crate base64;



//use std::str;
use std::fs;
use base64::{ decode};
use std::io::{BufRead, BufReader};
use std::fs::File;

fn main() {
    let tTexts = File::open("ScrabbleWords.txt").expect("Unable to open txt file");
    let tytexts = BufReader::new(tTexts);
    
    let TestLines:Vec<String> = tytexts.lines().map(|s| s.unwrap()).collect();

    let file = fs::read_to_string("6.txt").expect("txt error");
    
    let new_file = file.replace("\n","");
 
     
    let _b64file = decode(&new_file.trim()).expect("test");

    let key_size = 29;

    let mut Blocks:Vec<Vec<u8>> = Vec::new();
    let mut block_counter = 0;

    //this seperates into blocks with the first of each block and the 2 in each block
  
    while block_counter < key_size
    {
        let mut block = Vec::new();
        let mut counter = block_counter;
        while counter < _b64file.len()
        {
            block.push(_b64file[counter]);
            counter += key_size;
        }

        Blocks.push(block);
     
        block_counter +=1;
    }
    
    //loop through each and find the key for each block
    let mut block_counter_set =0;
    let mut posssible_keys:Vec<u8>  =  Vec::new();
    for block in Blocks
    {
        println!("starting block {}",block_counter_set );
       let keys = find_blocks(block);
        posssible_keys.push(keys);
        block_counter_set +=1;
    }
  // block_loop(posssible_keys,_b64file,TestLines);
    println!("did you find what you were looking for?");


}

fn find_blocks(block:Vec<u8>) -> u8
{
    let new_vec = vec![b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'A',b'b',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z',b'.',b'?',b':',b';',b' '];
    let mut found_keys:Vec<u8> = Vec::new();
    let mut found: u8 = 0;
    let mut key:char = 'a';
    let mut vowl_test = 0.0;
    for x_bytes in new_vec
    {
       
        let mut combined_bytes = Vec::new();
        //xored against the inner hex string
        for (v,main) in  block.iter().enumerate()
        {
            let new_bytes = x_bytes ^ main;
            combined_bytes.push(new_bytes);


        }
       
      
        let rm = String::from_utf8(combined_bytes);

        let ret = match rm {
            Ok(v) => v,
            Err(e) => break
           
        };
        let letters = ret.to_lowercase();

        let mut vowel_counter = 0.0;
        let mut avowel_counter = 0.0;
        let mut evowel_counter = 0.0;
        let mut ivowel_counter = 0.0;
        let mut ovowel_counter = 0.0;
        let mut uvowel_counter = 0.0;
        let mut other_counter = 0.0;
        for i in letters.chars()
        {
          match i {
              'a' => avowel_counter +=1.0,
              'e' => evowel_counter +=1.0,
              'i' => ivowel_counter +=1.0,
              'o' => ovowel_counter +=1.0,
              'u' => uvowel_counter  +=1.0,
              _  => other_counter +=1.0
          }


        }
        vowel_counter = avowel_counter + evowel_counter + ivowel_counter + ovowel_counter + uvowel_counter;
        let ecounter = evowel_counter/ (vowel_counter + other_counter);
        let vow_freq = vowel_counter / (vowel_counter + other_counter);
        
        
        if vow_freq > 0.20
        {
          if vowl_test <  vow_freq
          {
              found = x_bytes;
              key = x_bytes as char;
              vowl_test = vow_freq;
          }
           println!("poasible key: {} vowel frequency {}",x_bytes as char,vow_freq);
          // println!("poasible e key: {} ",ecounter);
         // return x_bytes
          // found_keys.push(x_bytes);
        } 
       
       
    }
   //have it return the highest value;
   println!("poasible key: {}",key);
   found

}