#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum LightingMode {
    Disable = 0,
    Static = 1,
    Breathing = 2,
    Strobe = 3,
    Spring = 4,
    Cycling = 5,
    Random = 6,
    Wave = 7,
    Water = 8,
    Rainbow = 9,
}
