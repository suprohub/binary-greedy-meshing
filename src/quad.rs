use bitfields::bitfield;

#[bitfield(u64, new = false)]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct Quad {
    #[bits(6)]
    x: u8,
    #[bits(6)]
    y: u8,
    #[bits(6)]
    z: u8,
    #[bits(6)]
    w: u8,
    #[bits(6)]
    h: u8,
    #[bits(2)] // ao
    _reserved: u8,
    m: u32,
}

impl Quad {
    pub fn new(x: u8, y: u8, z: u8, w: u8, h: u8, m: u32) -> Self {
        QuadBuilder::new()
            .with_x(x)
            .with_y(y)
            .with_z(z)
            .with_w(w)
            .with_h(h)
            .with_m(m)
            .build()
    }
}
