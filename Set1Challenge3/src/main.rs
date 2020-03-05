extern crate hex;
use std::collections::HashMap;
use std::str;

fn main() {
   let hex_string = "1b37373331363f78151b7f2b783431333d78397828372d363c78373e783a393b3736";
   let letter_stats :HashMap<String,u16> = HashMap::new();
   let new_Vec = vec![b'a',b'b',b'c',b'd',b'e',b'f',b'g',b'h',b'i',b'j',b'k',b'l',b'm',b'n',b'o',b'p',b'q',b'r',b's',b't',b'u',b'v',b'w',b'x',b'y',b'z',b'1',b'2',b'3',b'4',b'5',b'6',b'7',b'8',b'9',b'A',b'b',b'C',b'D',b'E',b'F',b'G',b'H',b'I',b'J',b'K',b'L',b'M',b'N',b'O',b'P',b'Q',b'R',b'S',b'T',b'U',b'V',b'W',b'X',b'Y',b'Z'];
  
   let main_bytes = hex::decode(hex_string).expect("main failed");
 

    for x_bytes in new_Vec
    {
       
        let mut combined_bytes = Vec::new();
        //xored against the inner hex string
        for (v,main) in  main_bytes.iter().enumerate()
        {
            let new_bytes = x_bytes ^ main;
            combined_bytes.push(new_bytes);


        }
        //todo have the combined xored bytes. do a to string and score
       let ret = String::from_utf8(combined_bytes).expect("A panic message to be displayed");
       let newRet = ret.to_lowercase();
       Scorring(ret,x_bytes);
      
    }
}

fn Scorring(_incoming_string:String,_incoming:u8) {
    
 let mut t = 0.0;
 let mut e = 0.0;
 let mut a = 0.0;
 let mut other = 0.0;
 let string_count = _incoming_string.len() as f32;
  for letter in _incoming_string.chars()
  {
    match letter {
        't' => t +=1.0,
        'e' => e +=1.0,
        'a' => a +=1.0,
        _ => other +=1.0
    };
  
   }

   let tprect =  t/string_count;
   let eprect = e/string_count;
   let aprect = a/string_count;
   let otherprect = other/string_count;

   if(eprect >= 0.01  )
   {
    
    println!("Translation: {}",_incoming_string);
   }

   
   
}