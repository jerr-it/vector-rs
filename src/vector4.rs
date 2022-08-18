use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::{Vector2, Vector3};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[repr(C)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T> Vector4<T>
where
    T: Copy
        + Default
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    pub const fn new(x: T, y: T, z: T, w: T) -> Vector4<T> {
        Vector4 { x, y, z, w }
    }

    // Implement the dot product of two vectors
    pub fn dot(&self, other: &Vector4<T>) -> T {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }
}

impl<T> From<[T; 4]> for Vector4<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: [T; 4]) -> Self {
        Vector4::new(v[0], v[1], v[2], v[3])
    }
}

impl<T> From<Vector2<T>> for Vector4<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: Vector2<T>) -> Self {
        Vector4::new(v.x, v.y, Default::default(), Default::default())
    }
}

impl<T> From<Vector3<T>> for Vector4<T>
where
    T: Default
        + Copy
        + std::ops::Add<Output = T>
        + std::ops::Sub<Output = T>
        + std::ops::Mul<Output = T>,
{
    fn from(v: Vector3<T>) -> Self {
        Vector4::new(v.x, v.y, v.z, Default::default())
    }
}

impl<T: Add<Output = T>> Add<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn add(self, rhs: Vector4<T>) -> Self::Output {
        Vector4 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
            w: self.w + rhs.w,
        }
    }
}

impl<T: AddAssign> AddAssign<Vector4<T>> for Vector4<T> {
    fn add_assign(&mut self, rhs: Vector4<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: Sub<Output = T>> Sub<Vector4<T>> for Vector4<T> {
    type Output = Vector4<T>;

    fn sub(self, rhs: Vector4<T>) -> Self::Output {
        Vector4 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
            w: self.w - rhs.w,
        }
    }
}

impl<T: SubAssign> SubAssign<Vector4<T>> for Vector4<T> {
    fn sub_assign(&mut self, rhs: Vector4<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector4<T> {
    type Output = Vector4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector4<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector4_new() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        assert_eq!(vector4.x, 1.0);
        assert_eq!(vector4.y, 2.0);
        assert_eq!(vector4.z, 3.0);
        assert_eq!(vector4.w, 4.0);
    }

    #[test]
    fn test_from_array() {
        let vector4 = Vector4::from([1.0, 2.0, 3.0, 4.0]);
        assert_eq!(vector4.x, 1.0);
        assert_eq!(vector4.y, 2.0);
        assert_eq!(vector4.z, 3.0);
        assert_eq!(vector4.w, 4.0);
    }

    #[test]
    fn test_vector4_dot() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_2 = Vector4::new(5.0, 6.0, 7.0, 8.0);
        assert_eq!(vector4.dot(&vector4_2), 70.0);
    }

    #[test]
    fn test_vector4_add() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_add = Vector4::new(2.0, 4.0, 6.0, 8.0);
        let vector4_result = vector4 + vector4_add;
        assert_eq!(vector4_result.x, 3.0);
        assert_eq!(vector4_result.y, 6.0);
        assert_eq!(vector4_result.z, 9.0);
        assert_eq!(vector4_result.w, 12.0);
    }

    #[test]
    fn test_vector4_add_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_add = Vector4::new(2.0, 4.0, 6.0, 8.0);
        vector4 += vector4_add;
        assert_eq!(vector4.x, 3.0);
        assert_eq!(vector4.y, 6.0);
        assert_eq!(vector4.z, 9.0);
        assert_eq!(vector4.w, 12.0);
    }

    #[test]
    fn test_vector4_sub() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_sub = Vector4::new(2.0, 4.0, 6.0, 8.0);
        let vector4_result = vector4 - vector4_sub;
        assert_eq!(vector4_result.x, -1.0);
        assert_eq!(vector4_result.y, -2.0);
        assert_eq!(vector4_result.z, -3.0);
        assert_eq!(vector4_result.w, -4.0);
    }

    #[test]
    fn test_vector4_sub_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_sub = Vector4::new(2.0, 4.0, 6.0, 8.0);
        vector4 -= vector4_sub;
        assert_eq!(vector4.x, -1.0);
        assert_eq!(vector4.y, -2.0);
        assert_eq!(vector4.z, -3.0);
        assert_eq!(vector4.w, -4.0);
    }

    #[test]
    fn test_vector4_mul() {
        let vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_mul = 2.0;
        let vector4_result = vector4 * vector4_mul;
        assert_eq!(vector4_result.x, 2.0);
        assert_eq!(vector4_result.y, 4.0);
        assert_eq!(vector4_result.z, 6.0);
        assert_eq!(vector4_result.w, 8.0);
    }

    #[test]
    fn test_vector4_mul_assign() {
        let mut vector4 = Vector4::new(1.0, 2.0, 3.0, 4.0);
        let vector4_mul = 2.0;
        vector4 *= vector4_mul;
        assert_eq!(vector4.x, 2.0);
        assert_eq!(vector4.y, 4.0);
        assert_eq!(vector4.z, 6.0);
        assert_eq!(vector4.w, 8.0);
    }
}
