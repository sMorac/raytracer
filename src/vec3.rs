use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Vec3 {
        Vec3 { x, y, z }
    }
    pub fn new_zero() -> Vec3 {
        Vec3 { x:0.0, y:0.0, z:0.0 }
    }
    pub fn new_unit() -> Vec3 {
        Vec3 { x: 1.0, y: 1.0, z: 1.0 }
    }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn square_length(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn make_unit_vector(self) -> Vec3 {
        self / self.length()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
    pub fn sqrt(&self) -> Self {
        Self { x: self.x.sqrt(),
               y: self.y.sqrt(),
               z: self.z.sqrt() }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Vec3({}, {}, {})", self.x, self.y, self.z)
    }
}

impl ops::Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: - self.x,
            y: - self.y,
            z: - self.z,
        }
    }
}

impl ops::AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        };
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl ops::Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        }
    }
}

impl ops::Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, vect: Vec3) -> Vec3 {
        Vec3 {
            x: vect.x * self,
            y: vect.y * self,
            z: vect.z * self,
        }
    }
}


impl ops::MulAssign<Vec3> for Vec3 {
    fn mul_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        };
    }
}
impl ops::MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, k: f32) {
        *self = Self {
            x: self.x * k,
            y: self.y * k,
            z: self.z * k,
        };
    }
}

impl ops::Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, k: f32) -> Self {
        Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        }
    }
}

impl ops::DivAssign<Vec3> for Vec3 {
    fn div_assign(&mut self, other: Vec3) {
        *self = Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        };
    }
}
impl ops::DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, k: f32) {
        *self = Self {
            x: self.x / k,
            y: self.y / k,
            z: self.z / k,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec3() {
            // raytracer::print_test_image();
        let mut s = Vec3::new(1.0, 1.0, 1.0);
        println!("{}", s);
        let ms = -s;
        println!("{}", ms);
        s += ms;
        println!("{}", s);
        s -= ms;
        println!("{}", s);
        println!("{}", s.length());
        println!("{}", s.square_length());
        s *= ms;
        println!("{}", s);
        s *= 3.0;
        println!("{}", s);
        s /= 3.0;
        println!("{}", s);
        s /= ms;
        println!("{}", s);
        let uv = s.make_unit_vector();
        println!("{}", s);
        println!("{}", uv);
        let a = Vec3::new(1.0, 0.0, 0.0);
        let b = Vec3::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        println!("{}", c);
        println!("{}", c.dot(uv));
    }
}