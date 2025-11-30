use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }

    pub fn yx(&self) -> Self { Self { x: self.y, y: self.x } }

    pub fn xyy(&self) -> Vec3 {
        Vec3 {
            x: self.x,
            y: self.y,
            z: self.y
        }
    }

    pub fn xyyx(&self) -> Vec4 { 
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }

    pub fn abs(&self) -> Self { Self::new(self.x.abs(), self.y.abs()) }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y
    }
}

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl Add<f32> for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x + rhs, self.y + rhs)
    }
}

impl Sub for Vec2 {
    type Output = Vec2;
    fn sub(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x - rhs.x, self.y - rhs.y)
    }
}

impl Mul for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: Vec2) -> Vec2 {
        Vec2::new(self.x * rhs.x, self.y * rhs.y)
    }
}

impl Mul<f32> for Vec2 {
    type Output = Vec2;
    fn mul(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Vec2 {
    type Output = Vec2;
    fn div(self, rhs: f32) -> Vec2 {
        Vec2::new(self.x / rhs, self.y / rhs)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec3 {
   pub x: f32,
   pub y: f32,
   pub z: f32
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }

    pub fn zxy(&self) -> Self {
        Self {
            x: self.z,
            y: self.x,
            z: self.y
        }
    }

    pub fn xz(&self) -> Vec2 {
        Vec2 {
            x: self.x,
            y: self.z
        }
    }

    pub fn dot(a: Self, b: Self) -> f32 {
        a.x * b.x + a.y * b.y + a.z * b.z
    }

    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(self) -> Self {
        let len = self.length();
        if len == 0.0 {
            return self;
        }

        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len
        }
    }

    pub fn mix(a: Self, b: Self, t: f32) -> Self {
        Self {
            x: a.x * (1.0 - t) + b.x * t,
            y: a.y * (1.0 - t) + b.y * t,
            z: a.z * (1.0 - t) + b.z * t,
        }
    }

    pub fn cross(a: Self, b: Self) -> Self {
        Self {
            x: a.y * b.z - a.z * b.y,
            y: a.z * b.x - a.x * b.z,
            z: a.x * b.y - a.y * b.x,
        }
    }

    pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Vec3::new(f(self.x), f(self.y), f(self.z))
    }
}

impl Add for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}

impl Add<f32> for Vec3 {
    type Output = Vec3;
    fn add(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x + rhs, self.y + rhs, self.z + rhs)
    }
}

impl Sub for Vec3 {
    type Output = Vec3;
    fn sub(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Mul<f32> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f32) -> Vec3 {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}

impl Div for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Vec4 {
   pub x: f32,
   pub y: f32,
   pub z: f32,
   pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }

    pub fn xyyx(&self) -> Self { Vec4::new(self.x, self.y, self.y, self.x) }

    pub fn rgb(&self) -> Vec3 { Vec3::new(self.x, self.y, self.z) }

    pub fn map<F: Fn(f32) -> f32>(self, f: F) -> Self {
        Vec4::new(f(self.x), f(self.y), f(self.z), f(self.w))
    }

    pub fn exp(&self) -> Self {
        Vec4::new(self.x.exp(), self.y.exp(), self.z.exp(), self.w.exp())
    }

    pub fn tanh(&self) -> Self {
        Vec4::new(self.x.tanh(), self.y.tanh(), self.z.tanh(), self.w.tanh())
    }
}

impl Add for Vec4 {
    type Output = Vec4;
    fn add(self, rhs: Vec4) -> Vec4 {
        Vec4::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z, self.w + rhs.w)
    }
}

impl Mul<f32> for Vec4 {
    type Output = Vec4;
    fn mul(self, rhs: f32) -> Vec4 {
        Vec4::new(self.x * rhs, self.y * rhs, self.z * rhs, self.w * rhs)
    }
}

impl Div for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: Vec4) -> Vec4 {
        Vec4::new(self.x / rhs.x, self.y / rhs.y, self.z / rhs.z, self.w / rhs.w)
    }
}

impl Div<f32> for Vec4 {
    type Output = Vec4;
    fn div(self, rhs: f32) -> Vec4 {
        Vec4::new(self.x / rhs, self.y / rhs, self.z / rhs, self.w / rhs)
    }
}

pub struct Vec4Iter<'a> {
    vec: &'a Vec4,
    index: usize
}

impl<'a> Iterator for Vec4Iter<'a> {
    type Item = &'a f32;
    
    fn next(&mut self) -> Option<Self::Item> {
        let result = match self.index {
            0 => Some(&self.vec.x),
            1 => Some(&self.vec.y),
            2 => Some(&self.vec.z),
            3 => Some(&self.vec.w),
            _ => None,
        };
        self.index += 1;
        result
    }
}

impl<'a> IntoIterator for &'a Vec4 {
    type Item = &'a f32;
    type IntoIter = Vec4Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Vec4Iter {
            vec: self,
            index: 0
        }
    }
}

