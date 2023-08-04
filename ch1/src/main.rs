use std::char::from_u32;

fn main() {
    
    let input = [99,114,121,112,116,111,123,65,83,67,73,73,95,112,114,49,110,116,52,98,108,51,125];

    for i in input.iter() {
        let c = from_u32(*i).ok_or(*i);
        println!("{:?}", c);
    }


}
