use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Vector2;

use super::Vector4;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

impl<T> Vector3<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    pub const fn new(x: T, y: T, z: T) -> Vector3<T> {
        Vector3 { x, y, z }
    }

    // Calculate the dot product of two vectors
    pub fn dot(&self, other: &Vector3<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    // Calculate the cross product of two vectors
    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        Vector3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}

impl<T> From<[T; 3]> for Vector3<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: [T; 3]) -> Self {
        Vector3::new(v[0], v[1], v[2])
    }
}

impl<T> From<Vector2<T>> for Vector3<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: Vector2<T>) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: Default::default(),
        }
    }
}

impl<T> From<Vector4<T>> for Vector3<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: Vector4<T>) -> Self {
        Vector3 {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl Vector3<f32> {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3<f32> {
        let inv_sqrt = self.magnitude().recip();
        Vector3 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt,
            z: self.z * inv_sqrt,
        }
    }
}

impl Vector3<f64> {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector3<f64> {
        let inv_sqrt = self.magnitude().recip();
        Vector3 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt,
            z: self.z * inv_sqrt,
        }
    }
}

impl<T: Add<Output = T>> Add<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn add(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl<T: AddAssign> AddAssign<Vector3<T>> for Vector3<T> {
    fn add_assign(&mut self, rhs: Vector3<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl<T: Sub<Output = T>> Sub<Vector3<T>> for Vector3<T> {
    type Output = Vector3<T>;

    fn sub(self, rhs: Vector3<T>) -> Self::Output {
        Vector3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl<T: SubAssign> SubAssign<Vector3<T>> for Vector3<T> {
    fn sub_assign(&mut self, rhs: Vector3<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector3<T> {
    type Output = Vector3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector3<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector3_new() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_from_array() {
        let v = Vector3::from([1.0, 2.0, 3.0]);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_vector3_as_vector4() {
        let v = Vector3::new(1.0, 2.0, 3.0);
        let v4: Vector4<f64> = v.into();
        assert_eq!(v4.x, 1.0);
        assert_eq!(v4.y, 2.0);
        assert_eq!(v4.z, 3.0);
        assert_eq!(v4.w, 0.0);
    }

    #[test]
    fn test_vector3_dot() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_vector3_cross() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(4.0, 5.0, 6.0);
        let v3 = v1.cross(&v2);
        assert_eq!(v3.x, -3.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, -3.0);
    }

    #[test]
    fn test_vector3_add() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 5.0);
        let v3 = v1 + v2;
        assert_eq!(v3.x, 4.0);
        assert_eq!(v3.y, 6.0);
        assert_eq!(v3.z, 8.0);
    }

    #[test]
    fn test_vector3_add_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 3.0);
        v1 += v2;
        assert_eq!(v1.x, 4.0);
        assert_eq!(v1.y, 6.0);
        assert_eq!(v1.z, 6.0);
    }

    #[test]
    fn test_vector3_sub() {
        let v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 3.0);
        let v3 = v1 - v2;
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -2.0);
        assert_eq!(v3.z, 0.0);
    }

    #[test]
    fn test_vector3_sub_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        let v2 = Vector3::new(3.0, 4.0, 1.0);
        v1 -= v2;
        assert_eq!(v1.x, -2.0);
        assert_eq!(v1.y, -2.0);
        assert_eq!(v1.z, 2.0);
    }

    #[test]
    fn test_vector3_mul() {
        let v1 = Vector3::new(1.0, 2.0, 4.0);
        let v2 = v1 * 2.0;
        assert_eq!(v2.x, 2.0);
        assert_eq!(v2.y, 4.0);
        assert_eq!(v2.z, 8.0);
    }

    #[test]
    fn test_vector3_mul_assign() {
        let mut v1 = Vector3::new(1.0, 2.0, 3.0);
        v1 *= 2.0;
        assert_eq!(v1.x, 2.0);
        assert_eq!(v1.y, 4.0);
        assert_eq!(v1.z, 6.0);
    }
}
