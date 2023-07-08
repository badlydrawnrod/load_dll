use arviss::Address;

pub fn read_instruction(slice: &[u8], addr: Address) -> Result<u32, Address> {
    let index = addr as usize;
    if (0..slice.len() - 3).contains(&index) {
        if let Ok(slice) = &slice[index..index + 4].try_into() {
            let result = u32::from_le_bytes(*slice);
            return Ok(result);
        }
    } else if (0..slice.len() - 1).contains(&index) {
        // Cater for a 16-bit instruction in the last two bytes of the image.
        if let Ok(slice) = &slice[index..index + 2].try_into() {
            let result = (u16::from_le_bytes(*slice)) as u32;
            return Ok(result);
        }
    }
    Err(addr)
}
