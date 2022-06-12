
// 生成的 bindings 代码根据 C/C++ 代码生成，里面有一些不符合 Rust 约定，我们不让编译期报警
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deref_nullptr)]

use anyhow::{anyhow, Result};
use std::mem;

mod bindings;

pub use bindings::*;

// 高层的 API，处理压缩，一般应该出现在另一个 crate
pub fn compress(input: &[u8]) -> Result<Vec<u8>> {
    let output = vec![0u8; input.len()];
    unsafe {
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzCompressInit(&mut stream as *mut _, 1, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        // 传入 input / output 进行压缩
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;  // 输入时表示我们的输出 buffer 空间, 返回时该值会被修改, 告诉我们 buffer 里还有多少可用
        let result = BZ2_bzCompress(&mut stream as *mut _, BZ_FINISH as _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to compress"));
        }

        // 结束压缩
        let result = BZ2_bzCompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end compression"));
        }
        let out_len = output.len() - stream.avail_out as usize;
        // output.truncate(len);
        println!("\nbefore: {} bytes", input.len());
        println!("after:  {} bytes", out_len);
        // 压缩比
        println!("Data compression ratio: {}", input.len() as f64 / out_len as f64);
    }

    Ok(output)
}

// 高层的 API，处理解压缩，一般应该出现在另一个 crate
pub fn decompress(input: &[u8]) -> Result<Vec<u8>> {
    // FIXME: better strategy!
    let output = vec![0u8; input.len()];
    unsafe {
        let mut stream: bz_stream = mem::zeroed();
        let result = BZ2_bzDecompressInit(&mut stream as *mut _, 0, 0);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to initialize"));
        }

        // 传入 input / output 进行解压缩
        stream.next_in = input.as_ptr() as *mut _;
        stream.avail_in = input.len() as _;
        stream.next_out = output.as_ptr() as *mut _;
        stream.avail_out = output.len() as _;
        let result = BZ2_bzDecompress(&mut stream as *mut _);
        if result != BZ_STREAM_END as _ {
            return Err(anyhow!("Failed to decompress"));
        }

        // 结束解压缩
        let result = BZ2_bzDecompressEnd(&mut stream as *mut _);
        if result != BZ_OK as _ {
            return Err(anyhow!("Failed to end decompression"));
        }
        // let len = output.len() - stream.avail_out as usize;
        // output.truncate(len);
    }

    Ok(output)
}


#[cfg(test)]
mod tests {
    // use std::process::Output;

    use super::*;

    #[test]
    fn test_compression_decompression() {
        let input = include_str!("1MB.bin").as_bytes();
        let compressed = compress(input).unwrap();
        let decompressed = decompress(&compressed).unwrap();

        assert_eq!(input, &decompressed);
    }
}