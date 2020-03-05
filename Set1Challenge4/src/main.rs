extern crate hex;

use std::str;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
   
    let tTexts = File::open("ScrabbleWords.txt").expect("Unable to open txt file");
    let tytexts = BufReader::new(tTexts);
    
    let TestLines:Vec<String> = tytexts.lines().map(|s| s.unwrap()).collect();
    
 
    let new_Vec = vec![b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'A',b'b',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'];
    let file = File::open("4.txt").expect("Unable to open C file");
    let reader = BufReader::new(file);

    
    for line in reader.lines()
    {
        let _line = line.expect("Unable to read line");
       
        let main_bytes = hex::decode(_line).expect("main failed");
       // println!("Line {}",Linecounter);
       
        for x_bytes in &new_Vec
        {
           
            let mut combined_bytes = Vec::new();
            //xored against the inner hex string
            for (v,main) in  main_bytes.iter().enumerate()
            {
                let new_bytes = x_bytes ^ main;
                combined_bytes.push(new_bytes);
    
    
            }
            //todo have the combined xored bytes. do a to string and score
          // let ret = String::from_utf8(combined_bytes).expect("A panic message to be displayed");
          
            let rm = String::from_utf8(combined_bytes);

            let ret = match rm {
                Ok(v) => v,
                Err(e) => break
               
            };

           let mut word_score = 0;
           
          for word in &TestLines
          {
                
            if ret.to_uppercase().contains(word) {word_score +=1;}
              
          }
          
          if word_score > 6 {println!("Line: {}",ret)}
        }
    
    
    
    
    }



}
