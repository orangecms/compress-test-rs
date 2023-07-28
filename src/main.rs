use lzss::Lzss;
use std::fs;
use std::io::Write;
use std::path::Path;

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
    let path = Path::new(filename.as_str());
    let file = fs::read(path).expect("no such file");
    let size = file.len();

    println!("compress.....");
    let mut output = vec![0; COMPRESSED_SIZE];
    match MyLzss::compress(
        lzss::SliceReader::new(&file),
        lzss::SliceWriter::new(&mut output),
    ) {
        Ok(r) => {
            println!("success: {r}\n");
            println!("write compressed file.....");
            let mut file = fs::File::create(path.with_extension("ore")).unwrap();
            file.write_all(&(r as u32).to_le_bytes()).unwrap();
            file.write_all(&output[..r]).unwrap();
        }
        Err(r) => println!("{r}\n"),
    }
}
