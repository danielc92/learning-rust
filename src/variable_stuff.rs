pub fn try_it_out() {
    // signed types allow for negative numbers (including 0)
    let _signed_num_1: i8 = 20;
    let _signed_num_2: i16 = -32500;
    let _signed_num_3: i32 = 32500;
    let _signed_num_4: i64 = 9223372036854775807;
    let _signed_num_5: i128 = -170141183460469231731687303715884105728;
    // unsigned types allow for only positive numbers and 0
    let _unsigned_num_0: u8 = 0;
    let _unsigned_num_1: u8 = 255;
    let _unsigned_num_2: u16 = 65535;
    let _unsigned_num_3: u32 = 4294967295;
    let _unsigned_num_4: u64 = 18_446_744_073_709_551_615;
    let _unsigned_num_5: u128 = 23;
    // constants cannot be reassigned
    const SOME_SETTING: u8 = 5;

    // varibales can be initialized after binding phase
    let _binding;
    _binding = 12345;
}
