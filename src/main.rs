use lzss::Lzss;
use std::fs;
use std::io::Write;

const COMPRESSED_SIZE: usize = 0xc00000;
const EI: usize = 12; // offset bits

fn main() {
    // offset_bits aka EI, usually 10..13
    // length_bits aka EJ, usually 4..5
    // buf_fill_byte (often 0x20, space) - should we use 0x00, zero byte?
    // decompress_buf_size, always 1 << EI
    // compress_buf_size, always 2 << EI
    type MyLzss = Lzss<EI, 4, 0x00, { 1 << EI }, { 2 << EI }>;

    println!("read original file.....");
    let filename = std::env::args().nth(1).expect("pass a file name");
    let original = fs::read(filename.as_str()).expect("no such file");
    let size = original.len();
    // we need temporary copies here - fine
    let compressed_file = String::from(filename.as_str()) + ".lzss";
    let uncompressed_file = String::from(filename.as_str()) + ".unlzss";

    println!("compress.....");
    let mut output = vec![0; COMPRESSED_SIZE];
    let result = MyLzss::compress(
        lzss::SliceReader::new(&original),
        lzss::SliceWriter::new(&mut output),
    );
    match result {
        Ok(r) => {
            println!("success: {r}\n");
            println!("write compressed file.....");
            // this here needs to be a temporary copy
            let mut file = fs::File::create(compressed_file.as_str()).unwrap();
            file.write_all(&output[..r]).unwrap();
        }
        Err(r) => println!("error: {r}\n"),
    }

    println!("read back compressed file.....");
    // because compressed_file is used twice - though only to read from...?!
    let compressed = fs::read(compressed_file.as_str()).expect("compressed file got lost");

    println!("decompress.....");
    let mut output = vec![0; size];
    let result = MyLzss::decompress(
        lzss::SliceReader::new(&compressed),
        lzss::SliceWriter::new(&mut output),
    );

    match result {
        Ok(r) => {
            println!("success: {r}\n");
            let mut file = fs::File::create(uncompressed_file).unwrap();
            file.write_all(&output).unwrap();
        }
        Err(r) => println!("error: {r}\n"),
    }
}
