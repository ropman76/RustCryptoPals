extern crate hex;

fn main() {
    let Main_string = "1c0111001f010100061a024b53535009181c";
    let Xor_String ="686974207468652062756c6c277320657965";

   let main_bytes = hex::decode(Main_string).expect("main failed");
  let Xor_Bytes = hex::decode(Xor_String).expect("xor failed");
  let mut new_bytes : Vec<u8> = Vec::new();
  for (index, Main_value) in  main_bytes.iter().enumerate() {
        let bytes =Xor_Bytes[index];
        let Main_bty: &u8 = Main_value;
         let Xored_Bytes = Main_bty ^ bytes;
         new_bytes.push(Xored_Bytes)


  }

let finalstring = hex::encode(new_bytes);
println!("final {}",finalstring);
}