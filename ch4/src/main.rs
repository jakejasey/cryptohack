/*
Another common encoding scheme is Base64, which allows us to represent binary data as an ASCII string using 64 characters. One character of a Base64 string encodes 6 bits, and so 4 characters of Base64 encode three 8-bit bytes.

Base64 is most commonly used online, so binary data such as images can be easily included into HTML or CSS files.

Take the below hex string, decode it into bytes and then encode it into Base64.

72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf

extern crate hex;

use 

fn main() {

    let input = hex::decode(72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf).expect("Decoding Failed);

    println!("{:?}", input);
}

*/

extern crate hex;
extern crate base64;

fn main() {
    let input = "72bca9b68fc16ac7beeb8f849dca1d8a783e8acf9679bf9269f7bf";

    let decoded_fromhex = hex::decode(input).expect("Decoding failed");

    let encoded = base64::decode(decoded_fromhex);



    println!("{:?}", encoded);
}
