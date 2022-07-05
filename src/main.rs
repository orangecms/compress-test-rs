use compression::prelude::*;
use lzss::Lzss;
use std::env;
use std::fs;
use std::io::Write;

const COMPRESSED_SIZE: usize = 0x800000;
const EI: usize = 12; // offset bits
const ORIGINAL_FILE: &str = "Image2";
const COMPRESSED_FILE: &str = "Image.lzss";
const UNCOMPRESSED_FILE: &str = "Image.unlzss";
const GZCOMPRESSED_FILE: &str = "Image.gz";
const GZUNCOMPRESSED_FILE: &str = "Image.ungz";

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
    println!("file size: {size}");

    /*
    println!("compress.....");
    let compressed = original
        .into_iter()
        .encode(&mut GZipEncoder::new(), Action::Finish)
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    println!("success: {:?}\n", compressed.len());
    println!("write compressed file.....");
    let mut file = fs::File::create(GZCOMPRESSED_FILE).unwrap();
    file.write_all(&compressed).unwrap();
    */

    println!("read back compressed file.....");
    let compressed = fs::read(GZCOMPRESSED_FILE).expect("no no file");
    println!("file size: {}", compressed.len());

    println!("decompress.....");
    let decompressed = compressed
        .into_iter()
        .decode(&mut GZipDecoder::new())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();
    println!("success: {:?}\n", decompressed.len());
    let mut file = fs::File::create(GZUNCOMPRESSED_FILE).unwrap();
    file.write_all(&decompressed).unwrap();

    /*
    // from test run: 8207254
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
    // assert_eq!(result, Ok(14)); // there was no overflow and the output is 14 bytes long
    match result {
        Ok(r) => {
            println!("success: {r}\n");
            let mut file = fs::File::create(UNCOMPRESSED_FILE).unwrap();
            file.write_all(&output).unwrap();
        }
        Err(r) => println!("error: {r}\n"),
    }
    */
}
