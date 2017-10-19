use byteorder::{ByteOrder, LittleEndian};
use nom::le_u32;

lazy_static! {
  /// The default pallete used by MagicaVoxel - this is supplied if no pallete
  /// is included in the .vox file.
  pub static ref DEFAULT_PALLETE: Vec<u32> =
    include_bytes!("resources/default_pallete.bytes")
        .chunks(4)
        .map(LittleEndian::read_u32)
        .collect();
}

named!(pub extract_pallete <&[u8], Vec<u32> >, complete!(do_parse!(
    take!(12) >>
    colors: many_m_n!(255, 255, le_u32) >>
    (colors)
)));

#[cfg(test)]
mod tests {
    use super::*;
    use avow::vec;

    #[test]
    fn can_parse_pallete_chunk() {
        let bytes = include_bytes!("resources/valid_pallete.bytes").to_vec();
        let result = super::extract_pallete(&bytes);
        assert!(result.is_done());
        let (_, pallete) = result.unwrap();
        vec::are_eq(pallete, DEFAULT_PALLETE.to_vec());
    }
}