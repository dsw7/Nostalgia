// See <link-to-circuit-diagram>
pub const R1: f32 = 99.5 * 1000.;

// See <link-to-circuit-diagram>
pub const R2: f32 = 21.5 * 1000.;

pub struct Results {
    pub period: f32,
    pub frequency: f32,
    pub cap_f: f32,
    pub cap_uf: f32,
    pub cap_nf: f32,
}
