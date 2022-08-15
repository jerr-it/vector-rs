use std::ops::{Add, AddAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Vector4;

use super::Vector3;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}

impl<T> Vector2<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    pub fn new(x: T, y: T) -> Vector2<T> {
        Vector2 { x, y }
    }
}

impl<T> From<[T; 2]> for Vector2<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn from(v: [T; 2]) -> Self {
        Vector2::new(v[0], v[1])
    }
}

impl<T> From<Vector3<T>> for Vector2<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn from(v: Vector3<T>) -> Self {
        Vector2::<T>::new(v.x, v.y)
    }
}

impl<T> From<Vector4<T>> for Vector2<T>
where
    T: Default + Copy + Add<Output = T> + Sub<Output = T> + Mul<Output = T>,
{
    fn from(v: Vector4<T>) -> Self {
        Vector2::<T>::new(v.x, v.y)
    }
}

impl Vector2<f32> {
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector2<f32> {
        let inv_sqrt = self.magnitude().recip();
        Vector2 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt,
        }
    }

    pub fn clamp_mag(&mut self, limit: f32) {
        let mag = self.magnitude();
        if mag > limit {
            let inv_mag = mag.recip();
            self.x *= inv_mag * limit;
            self.y *= inv_mag * limit;
        }
    }

    pub fn distance(&self, other: &Vector2<f32>) -> f32 {
        (self.x - other.x).hypot(self.y - other.y)
    }

    pub fn set_rotation(&mut self, angle: f32) {
        let sin = angle.sin();
        let cos = angle.cos();
        self.x = self.x * cos - self.y * sin;
        self.y = self.x * sin + self.y * cos;
    }
}

impl Vector2<f64> {
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Vector2<f64> {
        let inv_sqrt = self.magnitude().recip();
        Vector2 {
            x: self.x * inv_sqrt,
            y: self.y * inv_sqrt,
        }
    }
}

impl<T: Add<Output = T>> Add<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn add(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: AddAssign> AddAssign<Vector2<T>> for Vector2<T> {
    fn add_assign(&mut self, rhs: Vector2<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Sub<Output = T>> Sub<Vector2<T>> for Vector2<T> {
    type Output = Vector2<T>;

    fn sub(self, rhs: Vector2<T>) -> Self::Output {
        Vector2 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: SubAssign> SubAssign<Vector2<T>> for Vector2<T> {
    fn sub_assign(&mut self, rhs: Vector2<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Vector2<T> {
    type Output = Vector2<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Vector2 {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: MulAssign + Copy> MulAssign<T> for Vector2<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector2_new() {
        let vector = Vector2::new(1.0, 2.0);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
    }

    #[test]
    fn test_from_array() {
        let vector = Vector2::from([1.0, 2.0]);
        assert_eq!(vector.x, 1.0);
        assert_eq!(vector.y, 2.0);
    }

    #[test]
    fn test_vector2_as_vector3() {
        let vector = Vector2::new(1.0, 2.0);
        let vector3: Vector3<f64> = vector.into();
        assert_eq!(vector3.x, 1.0);
        assert_eq!(vector3.y, 2.0);
        assert_eq!(vector3.z, 0.0);
    }

    #[test]
    fn test_vector2_as_vector4() {
        let vector = Vector2::new(1.0, 2.0);
        let vector4: Vector4<f64> = vector.into();
        assert_eq!(vector4.x, 1.0);
        assert_eq!(vector4.y, 2.0);
        assert_eq!(vector4.z, 0.0);
        assert_eq!(vector4.w, 0.0);
    }

    #[test]
    fn test_vector2_add() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        let vector3 = vector + vector2;
        assert_eq!(vector3.x, 4.0);
        assert_eq!(vector3.y, 6.0);
    }

    #[test]
    fn test_vector2_add_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        vector += vector2;
        assert_eq!(vector.x, 4.0);
        assert_eq!(vector.y, 6.0);
    }

    #[test]
    fn test_vector2_sub() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        let vector3 = vector - vector2;
        assert_eq!(vector3.x, -2.0);
        assert_eq!(vector3.y, -2.0);
    }

    #[test]
    fn test_vector2_sub_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        let vector2 = Vector2::new(3.0, 4.0);
        vector -= vector2;
        assert_eq!(vector.x, -2.0);
        assert_eq!(vector.y, -2.0);
    }

    #[test]
    fn test_vector2_mul() {
        let vector = Vector2::new(1.0, 2.0);
        let vector2 = vector * 2.0;
        assert_eq!(vector2.x, 2.0);
        assert_eq!(vector2.y, 4.0);
    }

    #[test]
    fn test_vector2_mul_assign() {
        let mut vector = Vector2::new(1.0, 2.0);
        vector *= 2.0;
        assert_eq!(vector.x, 2.0);
        assert_eq!(vector.y, 4.0);
    }
}
