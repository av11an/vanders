use std::ops::{Add, Div, Mul, Sub};

#[derive(Copy, Clone, Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32
}

impl Vec2 {
    pub fn new(x: f32, y: f32) -> Self { Self { x, y } }

    pub fn yx(&self) -> Self { Self { x: self.y, y: self.x } }

    pub fn xyyx(&self) -> Vec4 { 
        Vec4 {
            x: self.x,
            y: self.y,
            z: self.y,
            w: self.x
        }
    }

    pub fn abs(&self) -> Self { Self::new(self.x.abs(), self.y.abs()) }

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

pub struct Vec4 {
   pub x: f32,
   pub y: f32,
   pub z: f32,
   pub w: f32
}

impl Vec4 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self { Self { x, y, z, w } }

    pub fn xyyx(&self) -> Self { Vec4::new(self.x, self.y, self.y, self.x) }

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
