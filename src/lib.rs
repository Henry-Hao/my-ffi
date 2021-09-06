use libc::{c_int, size_t};

#[link(name = "snappy")]
extern "C" {
    fn snappy_compress(
        input: *const u8,
        input_length: size_t,
        compressed: *mut u8,
        compressed_length: *mut size_t,
    ) -> c_int;
    fn snappy_uncompress(
        compressed: *const u8,
        compressed_length: size_t,
        uncompressed: *mut u8,
        uncompressed_length: *mut size_t,
    ) -> c_int;
    fn snappy_max_compressed_length(source_length: size_t) -> usize;
    fn snappy_uncompressed_length(
        compressed: *const u8,
        compressed_length: size_t,
        result: *mut size_t,
    ) -> c_int;
    fn snappy_validate_compressed_buffer(compressed: *const u8, compressed_length: size_t)
        -> c_int;
}

pub fn validate_compressed_buffer(src: &[u8]) -> bool {
    unsafe { snappy_validate_compressed_buffer(src.as_ptr(), src.len() as size_t) == 0 }
}

pub fn compress(src: &[u8]) -> Vec<u8> {
    unsafe {
        let mut len = snappy_max_compressed_length(src.len() as size_t);
        let mut dst: Vec<u8> = Vec::with_capacity(len as usize);
        snappy_compress(src.as_ptr(), src.len(), dst.as_mut_ptr(), &mut len);
        dst.set_len(len as usize);
        dst
    }
}

pub fn decompress(src: &[u8]) -> Option<Vec<u8>> {
    unsafe {
        let mut length = size_t::default();
        if (snappy_uncompressed_length(src.as_ptr(), src.len(), &mut length)) != 0 {
            None
        } else {
            let mut dst: Vec<u8> = Vec::with_capacity(length);
            if snappy_uncompress(src.as_ptr(), src.len(), dst.as_mut_ptr(), &mut length) == 0 {
                dst.set_len(length);
                Some(dst)
            } else {
                None
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn valid() {
        let a = vec![0xde, 0xae, 0xd0, 0x22];
        let c = &compress(&a);
        assert!(validate_compressed_buffer(c));
        assert!(decompress(c) == Some(a));
    }

    #[test]
    fn invalid() {
        let a = vec![0,0,0,0];
        assert!(!validate_compressed_buffer(&a));
        assert!(decompress(&a).is_none());
    }

    #[test]
    fn empty() {
        let d = vec![];
        assert!(!validate_compressed_buffer(&d));
        assert!(decompress(&d).is_none());

        let c = compress(&d);
        assert!(validate_compressed_buffer(&c));
        assert!(decompress(&c) == Some(d));
    }
}
