pub(crate) struct Quaternion {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32
}

impl Quaternion {

    pub fn new() -> Self {
        Self {a: 1., b: 1., c: 1., d: 1.}
    }

    pub fn new_from_coefficients(a: f32, b: f32, c: f32, d: f32) -> Self{
        Self {a, b, c, d}
    }

    pub fn normalize(&mut self) {
        let scaling_factor: f32 = (self.a*self.a + self.b*self.b + self.c*self.c + self.d*self.d).sqrt();
        print!("Scaling factor: {}", scaling_factor);
        self.a /= scaling_factor;
        self.b /= scaling_factor;
        self.c /= scaling_factor;
        self.d /= scaling_factor;

    }

    pub fn get_norm(self) -> f32{
        (self.a*self.a + self.b*self.b + self.c*self.c + self.d*self.d).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes() {
        let mut quaternion : Quaternion = Quaternion::new_from_coefficients(3., 5., 7., 9.);
        quaternion.normalize();
        assert_eq!(1., quaternion.get_norm());
    }
}
