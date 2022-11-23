use crate::memory::Word;

pub fn get_bit(w: Word, bit: Word) -> Word {
    if bit > 31 {
        panic!("get_bit: invalid bit range");
    }

    (w >> bit) & 1
}

pub fn test_bit(w: Word, bit: Word) -> bool {
    get_bit(w, bit) != 0
}

pub fn word_lsb_to_bool(w: Word) -> bool {
    w & 1 != 0
}