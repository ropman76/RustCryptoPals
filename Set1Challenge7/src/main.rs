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
        let mut  InvShiftRowsblock =  Block {
            one:self.one.clone(),
            two:self.two.clone(),
            three:self.three.clone(),
            four:self.four.clone()
        };
        
        let mut row_two = InvShiftRowsblock.two.clone();

        let threehold = row_two[3];
        row_two[3] =row_two[2];
        row_two[2] = row_two[1];
        row_two[1] =row_two[0];
        row_two[0] =threehold ;
        
        InvShiftRowsblock.two = row_two;


        let mut row_three = InvShiftRowsblock.three.clone();

        row_three[2] =  row_three[0]; 
        row_three[3] =  row_three[1];
        row_three[0] =  row_three[2];
        row_three[1] =  row_three[3];

        InvShiftRowsblock.three = row_three;
        
        let mut row_four = InvShiftRowsblock.four.clone();
        
        row_four[3] = row_four[0];
        row_four[0] = row_four[1];
        row_four[1] = row_four[2];
        row_four[2] = row_four[3];

        InvShiftRowsblock.four = row_four;

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