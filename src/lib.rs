use std::io::{Read, Seek, Write};

#[derive(Debug)]
pub struct TurnipFaceChances {
    /// Face ID == 0, default 35/58 chance.
    normal: u32,
    /// Face ID == 1, default 6/58 chance.
    unamused: u32,
    /// Face ID == 2, default 5/58 chance.
    line_eyes: u32,
    /// Face ID == 3, default 3/58 chance.
    circle_eyes: u32,
    /// Face ID == 4, default 3/58 chance.
    super_happy: u32,
    /// Face ID == 5, default 4/58 chance.
    winky: u32,
    /// Face ID == 6, default 1/58 chance.
    dot_eyes: u32,
    /// Face ID == 7, default 1/58 chance.
    stitch: u32,
}

impl Default for TurnipFaceChances {
    fn default() -> Self {
        Self {
            normal: 35,
            unamused: 6,
            line_eyes: 5,
            circle_eyes: 3,
            super_happy: 3,
            winky: 4,
            dot_eyes: 1,
            stitch: 1,
        }
    }
}

#[derive(Debug)]
pub struct ItemChances {
    /// Default 2/6 chance.
    bobomb: u32,
    /// Default 3/6 chance.
    mr_saturn: u32,
    /// Default 1/6 chance.
    beamsword: u32,
}

impl Default for ItemChances {
    fn default() -> Self {
        Self {
            bobomb: 2,
            mr_saturn: 3,
            beamsword: 1,
        }
    }
}

#[derive(Debug)]
pub struct VegetableChances {
    /// Default 1/128 chance.
    chance_of_item: u32,
    /// Default 127/128 chance.
    chance_of_turnip: u32,
    turnip_face_chances: TurnipFaceChances,
    item_chances: ItemChances,
}

impl Default for VegetableChances {
    fn default() -> Self {
        Self {
            chance_of_item: 1,
            chance_of_turnip: 127,
            turnip_face_chances: Default::default(),
            item_chances: Default::default(),
        }
    }
}

impl VegetableChances {
    /// Read chances from GALE01 NTSC v1.02 disk image.
    fn get<R: Read>(melee: R) -> Self {
        todo!()
    }

    /// Modify GALE01 NTSC v1.02 disk image Peach Vegetable probabilities.
    fn set<W: Write>(&self, melee: W) -> Result<(), String> {
        todo!()
    }
}

/// https://www.gc-forever.com/yagcd/chap14.html#sec14.8.1
pub struct GCMHeader {
    /// Offset to FST inside GCM
    pub fst_offset: u32,
    /// Size of FST
    pub fst_size: u32,
}

impl GCMHeader {
    pub const FST_OFFSET: u32 = 0x424; // length = 0x4
    pub const FST_SIZE_OFFSET: u32 = 0x428; // length = 0x4

    fn new<R: Read + Seek>(mut melee: R) -> std::io::Result<Self> {
        let mut fst_offset_bytes = [0; 4];
        let mut fst_size_bytes = [0; 4];
        melee.seek(std::io::SeekFrom::Start(Self::FST_OFFSET as u64))?;
        melee.read_exact(&mut fst_offset_bytes)?;
        melee.seek(std::io::SeekFrom::Start(Self::FST_SIZE_OFFSET as u64))?;
        melee.read_exact(&mut fst_size_bytes)?;
        Ok(Self {
            fst_offset: u32::from_be_bytes(fst_offset_bytes),
            fst_size: u32::from_be_bytes(fst_size_bytes),
        })
    }
}

impl std::fmt::Debug for GCMHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("GCMHeader")
            .field("fst_offset", &format_args!("0x{:x}", &self.fst_offset))
            .field("fst_size", &format_args!("0x{:x}", &self.fst_size))
            .finish()
    }
}

#[derive(Debug)]
pub struct FstEntry {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    const TURNIP_CHANCE_BYTES: [u8; 16] = [
        0x23, 0x02, // normal
        0x06, 0x02, // unamused
        0x05, 0x02, // line eyes
        0x03, 0x02, // circle eyes
        0x03, 0x02, // super happy
        0x04, 0x06, // winky
        0x01, 0x0c, // dot eyes
        0x01, 0x1e, // stitch
    ];

    const EXPECTED_FST_OFFSET: u32 = 0x456e00;
    const EXPECTED_FST_SIZE: u32 = 0x7529;

    #[test]
    fn gcm_header() -> std::io::Result<()> {
        let path = std::env::var("SSBM_ISO_PATH").unwrap();
        let mut file = File::open("modified.iso")?;
        let header = dbg!(GCMHeader::new(&mut file)?);
        assert_eq!(header.fst_offset, EXPECTED_FST_OFFSET);
        assert_eq!(header.fst_size, EXPECTED_FST_SIZE);
        Ok(())
    }

    #[test]
    fn modify_chances() -> std::io::Result<()> {
        let path = std::env::var("SSBM_ISO_PATH").unwrap();
        // fs::copy(&path, "modified.iso")?;
        let mut file = File::open("modified.iso")?;
        let chances: VegetableChances = Default::default();
        Ok(chances.set(&mut file).unwrap())
    }
}
