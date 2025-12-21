
pub(crate) fn u16_to_vec(num: &u16) -> Vec<u8> {
    vec![(num >> 8) as u8, (num & 0x00FF) as u8]
}

#[cfg(test)]
mod test {
    use crate::util::big_endian::u16_to_vec;

    #[test]
    fn test_u16_to_vec() {
        let num: u16 = 0x02FF;
        let expected: Vec<u8> = vec![0x02, 0xFF];
        assert_eq!(expected, u16_to_vec(&num));
    }
}