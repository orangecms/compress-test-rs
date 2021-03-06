use lzss::Lzss;
use std::fs;
use std::io::Write;

const COMPRESSED_SIZE: usize = 0xc00000;
const EI: usize = 12; // offset bits
const ORIGINAL_FILE: &str = "Image";
const COMPRESSED_FILE: &str = "Image.lzss";
const UNCOMPRESSED_FILE: &str = "Image.unlzss";

fn main() {
    // offset_bits aka EI, usually 10..13
    // length_bits aka EJ, usually 4..5
    // buf_fill_byte (often 0x20, space) - should we use 0x00, zero byte?
    // decompress_buf_size, always 1 << EI
    // compress_buf_size, always 2 << EI
    type MyLzss = Lzss<EI, 4, 0x00, { 1 << EI }, { 2 << EI }>;

    println!("read original file.....");
    let original = fs::read(ORIGINAL_FILE).expect("no no file");
    let size = original.len();

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
            let mut file = fs::File::create(COMPRESSED_FILE).unwrap();
            file.write_all(&output[..r]).unwrap();
        }
        Err(r) => println!("error: {r}\n"),
    }

    println!("read back compressed file.....");
    let compressed = fs::read(COMPRESSED_FILE).expect("no no file");

    println!("decompress.....");
    let mut output = vec![0; size];
    let result = MyLzss::decompress(
        lzss::SliceReader::new(&compressed),
        lzss::SliceWriter::new(&mut output),
    );

    match result {
        Ok(r) => {
            println!("success: {r}\n");
            let mut file = fs::File::create(UNCOMPRESSED_FILE).unwrap();
            file.write_all(&output).unwrap();
        }
        Err(r) => println!("error: {r}\n"),
    }
}
