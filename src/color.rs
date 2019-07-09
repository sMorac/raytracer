use std::fmt;
use std::ops;

#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Color {
    pub fn new(red: f32, green: f32, blue: f32) -> Color {
        Color { red, green, blue }
    }

    pub fn black() -> Color {
        Color { red:0.0, green:0.0, blue:0.0 }
    }

    pub fn white() -> Color {
        Color { red: 1.0, green: 1.0, blue: 1.0 }
    }
    
    pub fn length(&self) -> f32 {
        (self.red * self.red + self.green * self.green + self.blue * self.blue).sqrt()
    }

    pub fn square_length(&self) -> f32 {
        self.red * self.red + self.green * self.green + self.blue * self.blue
    }

    pub fn make_unit_vector(self) -> Color {
        self / self.length()
    }

    pub fn dot(&self, other: Self) -> f32 {
        self.red * other.red + self.green * other.green + self.blue * other.blue
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            red: self.green * other.blue - self.blue * other.green,
            green: self.blue * other.red - self.red * other.blue,
            blue: self.red * other.green - self.green * other.red,
        }
    }
    pub fn sqrt(&self) -> Self {
        Self { red: self.red.sqrt(),
               green: self.green.sqrt(),
               blue: self.blue.sqrt() }
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Color({}, {}, {})", self.red, self.green, self.blue)
    }
}

impl ops::Add for Color {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

impl ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            red: - self.red,
            green: - self.green,
            blue: - self.blue,
        }
    }
}

impl ops::AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        };
    }
}

impl ops::Sub for Color {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

impl ops::SubAssign for Color {
    fn sub_assign(&mut self, other: Self) {
        *self = Self {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        };
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;

    fn mul(self, k: f32) -> Self {
        Self {
            red: self.red * k,
            green: self.green * k,
            blue: self.blue * k,
        }
    }
}

impl ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, vect: Color) -> Color {
        Color {
            red: vect.red * self,
            green: vect.green * self,
            blue: vect.blue * self,
        }
    }
}


impl ops::MulAssign<Color> for Color {
    fn mul_assign(&mut self, other: Color) {
        *self = Self {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        };
    }
}
impl ops::MulAssign<f32> for Color {
    fn mul_assign(&mut self, k: f32) {
        *self = Self {
            red: self.red * k,
            green: self.green * k,
            blue: self.blue * k,
        };
    }
}

impl ops::Div<f32> for Color {
    type Output = Self;

    fn div(self, k: f32) -> Self {
        Self {
            red: self.red / k,
            green: self.green / k,
            blue: self.blue / k,
        }
    }
}

impl ops::DivAssign<Color> for Color {
    fn div_assign(&mut self, other: Color) {
        *self = Self {
            red: self.red / other.red,
            green: self.green / other.green,
            blue: self.blue / other.blue,
        };
    }
}
impl ops::DivAssign<f32> for Color {
    fn div_assign(&mut self, k: f32) {
        *self = Self {
            red: self.red / k,
            green: self.green / k,
            blue: self.blue / k,
        };
    }
}

#[derive(Copy, Clone, Debug)]
pub struct ColorU8 {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl ColorU8 {
    pub fn make_from_color(c: Color) -> ColorU8 {
        ColorU8 {
            red: c.red.round() as u8,
            green: c.green.round() as u8,
            blue: c.blue.round() as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color() {
            // raytracer::print_test_image();
        let mut s = Color::new(1.0, 1.0, 1.0);
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
        let a = Color::new(1.0, 0.0, 0.0);
        let b = Color::new(0.0, 1.0, 0.0);
        let c = a.cross(b);
        println!("{}", c);
        println!("{}", c.dot(uv));
    }
}