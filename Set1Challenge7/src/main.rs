use std::fs;
use base64::{ decode};
struct Block {
    one: Vec<u8>,
    two:  Vec<u8>,
    three: Vec<u8>,
    four: Vec<u8>,
 
}
impl Block {

    fn InvShiftRows(&self)  -> Block {
        let  InvShiftRowsblock =  Block {
            one:self.one.clone(),
            two:self.two.clone(),
            three:self.three.clone(),
            four:self.four.clone()
        };
        InvShiftRowsblock
    }

    fn InvSubBytes(&self) -> Block {
        let  InvSubBytesblock =  Block {
            one:self.one.clone(),
            two:self.two.clone(),
            three:self.three.clone(),
            four:self.four.clone()
        };
        InvSubBytesblock
    }

    fn AddRoundKey(&self) -> Block {
        let  AddRoundKeyblock =   Block {
            one:self.one.clone(),
            two:self.two.clone(),
            three:self.three.clone(),
            four:self.four.clone()
        };
        AddRoundKeyblock
    }

    fn InvMixColumns(&self) -> Block {
        let  AddRoundKeyblock = Block {
            one:self.one.clone(),
            two:self.two.clone(),
            three:self.three.clone(),
            four:self.four.clone()
        };
        AddRoundKeyblock

    }

}

fn main() {
   
    let Key = String::from("YELLOW SUBMARINE");
    let _b_key = Key.as_bytes();
    let file = fs::read_to_string("7.txt").expect("txt error");
    let new_file = file.replace("\n","");
  
   
     
    let _b64file = decode(&new_file.trim()).expect("test");
    let m =_b_key.len();
    println!("length:{}",m);


}

fn Inputbytes(input: Vec<u8>) -> Vec<Block>
{
    let mut InitalState:Vec<Block> = Vec::new();
   
    let  AllBlocks:Vec<Vec<u8>> = input.chunks(16).map(|x| x.to_vec()).collect();
   for unstructured_blocks in AllBlocks
    {
     

        let structblock:Vec<Vec<u8>> = unstructured_blocks.chunks(4).map(|x| x.to_vec()).collect();
        let NewBlock = Newblock(structblock);
      
        InitalState.push(NewBlock);
    }


    InitalState
}

fn Newblock(input_rows:Vec<Vec<u8>>) -> Block {
    Block {
        one:input_rows[0].clone(),
        two:input_rows[1].clone(),
        three:input_rows[2].clone(),
        four:input_rows[3].clone()
    }
}