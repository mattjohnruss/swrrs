use std::ops::{Add, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Add for Vec2<T>
where T: Add<Output=T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl<T> Sub for Vec2<T>
where T: Sub<Output=T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl From<Vec2<u32>> for Vec2<i32> {
    fn from(v: Vec2<u32>) -> Self {
        Vec2 {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

impl From<Vec2<f32>> for Vec2<i32> {
    fn from(v: Vec2<f32>) -> Self {
        Vec2 {
            x: v.x as i32,
            y: v.y as i32,
        }
    }
}

// Conversion from Vec3 to Vec2
// Discards the z coord and uses only the x and y coords
impl<T> From<Vec3<T>> for Vec2<T> {
    fn from(v: Vec3<T>) -> Self {
        Vec2{ x: v.x, y: v.y }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Add for Vec3<T>
where T: Add<Output=T> {
    type Output = Self;
    
    fn add(self, other: Self) -> Self {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl<T> Sub for Vec3<T>
where T: Sub<Output=T> {
    type Output = Self;
    
    fn sub(self, other: Self) -> Self {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}
