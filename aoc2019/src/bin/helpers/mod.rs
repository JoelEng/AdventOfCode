#[allow(dead_code)]
pub mod intcode;

/** Get a specific digit from a number. pos = 0 gives least significant digit. count specifies number of digits to include */
pub fn get_digit(num: u32, pos: u32, count: u32) -> u8 {
    (num / 10u32.pow(pos) % (10u32.pow(count))) as u8
}
