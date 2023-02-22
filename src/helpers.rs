pub mod conversions {

    const CONV_MILLI: f32 = 1000.;
    const CONV_MICRO: f32 = 1000000.;
    const CONV_NANO:  f32 = 1000000000.;

    pub fn msec_to_sec(period: &f32) -> f32 {
        period / CONV_MILLI
    }

    pub fn usec_to_sec(period: &f32) -> f32 {
        period / CONV_MICRO
    }

    pub fn farad_to_uf(farads: &f32) -> f32 {
        farads * CONV_MICRO
    }

    pub fn farad_to_nf(farads: &f32) -> f32 {
        farads * CONV_NANO
    }

}
