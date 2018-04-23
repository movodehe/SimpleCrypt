use crypter::{bytes_to_u64, u64_to_bytes};

#[cfg(test)]
mod tests {
    use super::*;
    
    # [test]
    fn u64_to_bytes_1() {
        assert_eq!([0; 8], u64_to_bytes(0));
    }
    # [test]
    fn u64_to_bytes_2() {
        assert_eq!([0, 0, 0, 0, 0, 0, 0, 123], u64_to_bytes(123));
    }
    # [test]
    fn u64_to_bytes_3() {
        assert_eq!([0, 0, 0, 0, 0, 0, 1, 0], u64_to_bytes(256));
    }
    # [test]
    fn u64_to_bytes_4() {
        let base: u64 = 256;
        assert_eq!([100, 0, 0, 0, 0, 0, 0, 0], u64_to_bytes(base.pow(7) * 100));
    }
    # [test]
    fn u64_to_bytes_5() {
        let base: u64 = 256;
        assert_eq!([89, 233, 92, 97, 1, 33, 255, 123], u64_to_bytes(base.pow(0) * 123 + base.pow(1) * 255 + base.pow(2) * 33 + base.pow(3) * 1
        + base.pow(4) * 97 + base.pow(5) * 92 + base.pow(6) * 233 + base.pow(7) * 89));
    }
    # [test]
    fn u64_to_bytes_6() {
        let base: u64 = 256;
        assert_eq!([255, 255, 255, 255, 255, 255, 255, 255], u64_to_bytes(base.pow(0) * 255 + base.pow(1) * 255 + base.pow(2) * 255 + base.pow(3) * 255
        + base.pow(4) * 255 + base.pow(5) * 255 + base.pow(6) * 255 + base.pow(7) * 255));
    }

    # [test]
    fn bytes_to_u64_1() {
        assert_eq!(0, bytes_to_u64([0; 8]));
    }
    # [test]
    fn bytes_to_u64_2() {
        assert_eq!(123, bytes_to_u64([0, 0, 0, 0, 0, 0, 0, 123]));
    }
    # [test]
    fn bytes_to_u64_3() {
        assert_eq!(256, bytes_to_u64([0, 0, 0, 0, 0, 0, 1, 0]));
    }
    # [test]
    fn bytes_to_u64_4() {
        let base: u64 = 256;
        assert_eq!(base.pow(7) * 100, bytes_to_u64([100, 0, 0, 0, 0, 0, 0, 0]));
    }
    # [test]
    fn bytes_to_u64_5() {
        let base: u64 = 256;
        assert_eq!(base.pow(0) * 123 + base.pow(1) * 255 + base.pow(2) * 33 + base.pow(3) * 1
        + base.pow(4) * 97 + base.pow(5) * 92 + base.pow(6) * 233 + base.pow(7) * 89, bytes_to_u64([89, 233, 92, 97, 1, 33, 255, 123]));
    }
    # [test]
    fn bytes_to_u64_6() {
        let base: u64 = 256;
        assert_eq!(base.pow(0) * 255 + base.pow(1) * 255 + base.pow(2) * 255 + base.pow(3) * 255
        + base.pow(4) * 255 + base.pow(5) * 255 + base.pow(6) * 255 + base.pow(7) * 255, bytes_to_u64([255, 255, 255, 255, 255, 255, 255, 255]));
    }
}