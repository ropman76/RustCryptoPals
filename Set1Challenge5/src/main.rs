extern crate hex;

fn main() {
   
    let IceIceBaby = String::from("Burning 'em, if you ain't quick and nimble I go crazy when I hear a cymbal");
    let Ice = IceIceBaby.into_bytes();
    let new_Vec = vec![b'I',b'C',b'E'];

    let mut combined_bytes = Vec::new();    
    let mut counter = 0;
    for (v,main) in  Ice.iter().enumerate()
    {
           
                let new_bytes = new_Vec[counter] ^ main;
                combined_bytes.push(new_bytes);
                counter +=1;
         
                if counter == new_Vec.len() {counter = 0;}
    
    
    }
    
     let rm  =hex::encode(combined_bytes);
   println!("line {}",rm);
}
