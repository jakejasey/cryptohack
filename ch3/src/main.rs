/*
Another common encoding scheme is Base64, which allows us to represent binary data as an ASCII string using 64 characters. One character of a Base64 string encodes 6 bits, and so 4 characters of Base64 encode three 8-bit bytes.

Base64 is most commonly used online, so binary data such as images can be easily included into HTML or CSS files.

Take the below hex string, decode it into bytes and then encode it into Base64.

72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf

*/

use hex::FromHex;


fn main() {
    println!("challenge 4");

    match Vec::from_hex("72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf") {
        Ok(vec) => {
            for b in vec {
                println!("{}", b as char);
            }
        }
        Err(e) => {
            println!("Dealt with!");
        }
    }

}
