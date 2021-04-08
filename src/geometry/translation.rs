use approx::{AbsDiffEq, RelativeEq, UlpsEq};
use num::{One, Zero};
use std::fmt;
use std::hash;
#[cfg(feature = "abomonation-serialize")]
use std::io::{Result as IOResult, Write};

#[cfg(feature = "serde-serialize")]
use serde::{Deserialize, Deserializer, Serialize, Serializer};

#[cfg(feature = "abomonation-serialize")]
use abomonation::Abomonation;

use simba::scalar::{ClosedAdd, ClosedNeg, ClosedSub};

use crate::base::allocator::Allocator;
use crate::base::dimension::{DimNameAdd, DimNameSum, U1};
use crate::base::storage::Owned;
use crate::base::{CVectorN, Const, DefaultAllocator, MatrixN, Scalar};

use crate::geometry::Point;

/// A translation.
#[repr(C)]
#[derive(Debug)]
pub struct Translation<N: Scalar, const D: usize>
// where
//     DefaultAllocator: Allocator<N, D>,
{
    /// The translation coordinates, i.e., how much is added to a point's coordinates when it is
    /// translated.
    pub vector: CVectorN<N, D>,
}

impl<N: Scalar + hash::Hash, const D: usize> hash::Hash for Translation<N, D>
where
    Owned<N, Const<D>>: hash::Hash,
{
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.vector.hash(state)
    }
}

impl<N: Scalar + Copy, const D: usize> Copy for Translation<N, D> where Owned<N, Const<D>>: Copy {}

impl<N: Scalar, const D: usize> Clone for Translation<N, D>
where
    Owned<N, Const<D>>: Clone,
{
    #[inline]
    fn clone(&self) -> Self {
        Translation::from(self.vector.clone())
    }
}

#[cfg(feature = "abomonation-serialize")]
impl<N, const D: usize> Abomonation for Translation<N, D>
where
    N: Scalar,
    CVectorN<N, D>: Abomonation,
{
    unsafe fn entomb<W: Write>(&self, writer: &mut W) -> IOResult<()> {
        self.vector.entomb(writer)
    }

    fn extent(&self) -> usize {
        self.vector.extent()
    }

    unsafe fn exhume<'a, 'b>(&'a mut self, bytes: &'b mut [u8]) -> Option<&'b mut [u8]> {
        self.vector.exhume(bytes)
    }
}

#[cfg(feature = "serde-serialize")]
impl<N: Scalar, const D: usize> Serialize for Translation<N, D>
where
    Owned<N, Const<D>>: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        self.vector.serialize(serializer)
    }
}

#[cfg(feature = "serde-serialize")]
impl<'a, N: Scalar, const D: usize> Deserialize<'a> for Translation<N, D>
where
    Owned<N, Const<D>>: Deserialize<'a>,
{
    fn deserialize<Des>(deserializer: Des) -> Result<Self, Des::Error>
    where
        Des: Deserializer<'a>,
    {
        let matrix = CVectorN::<N, D>::deserialize(deserializer)?;

        Ok(Translation::from(matrix))
    }
}

impl<N: Scalar, const D: usize> Translation<N, D>
// where
//     DefaultAllocator: Allocator<N, D>,
{
    /// Creates a new translation from the given vector.
    #[inline]
    #[deprecated(note = "Use `::from` instead.")]
    pub fn from_vector(vector: CVectorN<N, D>) -> Translation<N, D> {
        Translation { vector }
    }

    /// Inverts `self`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// assert_eq!(t * t.inverse(), Translation3::identity());
    /// assert_eq!(t.inverse() * t, Translation3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Translation2::new(1.0, 2.0);
    /// assert_eq!(t * t.inverse(), Translation2::identity());
    /// assert_eq!(t.inverse() * t, Translation2::identity());
    /// ```
    #[inline]
    #[must_use = "Did you mean to use inverse_mut()?"]
    pub fn inverse(&self) -> Translation<N, D>
    where
        N: ClosedNeg,
    {
        Translation::from(-&self.vector)
    }

    /// Converts this translation into its equivalent homogeneous transformation matrix.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3, Matrix3, Matrix4};
    /// let t = Translation3::new(10.0, 20.0, 30.0);
    /// let expected = Matrix4::new(1.0, 0.0, 0.0, 10.0,
    ///                             0.0, 1.0, 0.0, 20.0,
    ///                             0.0, 0.0, 1.0, 30.0,
    ///                             0.0, 0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    ///
    /// let t = Translation2::new(10.0, 20.0);
    /// let expected = Matrix3::new(1.0, 0.0, 10.0,
    ///                             0.0, 1.0, 20.0,
    ///                             0.0, 0.0, 1.0);
    /// assert_eq!(t.to_homogeneous(), expected);
    /// ```
    #[inline]
    pub fn to_homogeneous(&self) -> MatrixN<N, DimNameSum<Const<D>, U1>>
    where
        N: Zero + One,
        Const<D>: DimNameAdd<U1>,
        DefaultAllocator: Allocator<N, DimNameSum<Const<D>, U1>, DimNameSum<Const<D>, U1>>,
    {
        let mut res = MatrixN::<N, DimNameSum<Const<D>, U1>>::identity();
        res.fixed_slice_mut::<Const<D>, U1>(0, D)
            .copy_from(&self.vector);

        res
    }

    /// Inverts `self` in-place.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation2, Translation3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let mut inv_t = Translation3::new(1.0, 2.0, 3.0);
    /// inv_t.inverse_mut();
    /// assert_eq!(t * inv_t, Translation3::identity());
    /// assert_eq!(inv_t * t, Translation3::identity());
    ///
    /// // Work in all dimensions.
    /// let t = Translation2::new(1.0, 2.0);
    /// let mut inv_t = Translation2::new(1.0, 2.0);
    /// inv_t.inverse_mut();
    /// assert_eq!(t * inv_t, Translation2::identity());
    /// assert_eq!(inv_t * t, Translation2::identity());
    /// ```
    #[inline]
    pub fn inverse_mut(&mut self)
    where
        N: ClosedNeg,
    {
        self.vector.neg_mut()
    }
}

impl<N: Scalar + ClosedAdd, const D: usize> Translation<N, D>
// where
//     DefaultAllocator: Allocator<N, D>,
{
    /// Translate the given point.
    ///
    /// This is the same as the multiplication `self * pt`.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation3, Point3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_eq!(transformed_point, Point3::new(5.0, 7.0, 9.0));
    #[inline]
    pub fn transform_point(&self, pt: &Point<N, D>) -> Point<N, D> {
        pt + &self.vector
    }
}

impl<N: Scalar + ClosedSub, const D: usize> Translation<N, D>
// where
//     DefaultAllocator: Allocator<N, D>,
{
    /// Translate the given point by the inverse of this translation.
    ///
    /// # Example
    /// ```
    /// # use nalgebra::{Translation3, Point3};
    /// let t = Translation3::new(1.0, 2.0, 3.0);
    /// let transformed_point = t.inverse_transform_point(&Point3::new(4.0, 5.0, 6.0));
    /// assert_eq!(transformed_point, Point3::new(3.0, 3.0, 3.0));
    #[inline]
    pub fn inverse_transform_point(&self, pt: &Point<N, D>) -> Point<N, D> {
        pt - &self.vector
    }
}

impl<N: Scalar + Eq, const D: usize> Eq for Translation<N, D>
// where DefaultAllocator: Allocator<N, D>
{
}

impl<N: Scalar + PartialEq, const D: usize> PartialEq for Translation<N, D>
// where
//     DefaultAllocator: Allocator<N, D>,
{
    #[inline]
    fn eq(&self, right: &Translation<N, D>) -> bool {
        self.vector == right.vector
    }
}

impl<N: Scalar + AbsDiffEq, const D: usize> AbsDiffEq for Translation<N, D>
where
    N::Epsilon: Copy,
{
    type Epsilon = N::Epsilon;

    #[inline]
    fn default_epsilon() -> Self::Epsilon {
        N::default_epsilon()
    }

    #[inline]
    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.vector.abs_diff_eq(&other.vector, epsilon)
    }
}

impl<N: Scalar + RelativeEq, const D: usize> RelativeEq for Translation<N, D>
where
    N::Epsilon: Copy,
{
    #[inline]
    fn default_max_relative() -> Self::Epsilon {
        N::default_max_relative()
    }

    #[inline]
    fn relative_eq(
        &self,
        other: &Self,
        epsilon: Self::Epsilon,
        max_relative: Self::Epsilon,
    ) -> bool {
        self.vector
            .relative_eq(&other.vector, epsilon, max_relative)
    }
}

impl<N: Scalar + UlpsEq, const D: usize> UlpsEq for Translation<N, D>
where
    N::Epsilon: Copy,
{
    #[inline]
    fn default_max_ulps() -> u32 {
        N::default_max_ulps()
    }

    #[inline]
    fn ulps_eq(&self, other: &Self, epsilon: Self::Epsilon, max_ulps: u32) -> bool {
        self.vector.ulps_eq(&other.vector, epsilon, max_ulps)
    }
}

/*
 *
 * Display
 *
 */
impl<N: Scalar + fmt::Display, const D: usize> fmt::Display for Translation<N, D>
// where
//     DefaultAllocator: Allocator<N, D> + Allocator<usize, D>,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let precision = f.precision().unwrap_or(3);

        writeln!(f, "Translation {{")?;
        write!(f, "{:.*}", precision, self.vector)?;
        writeln!(f, "}}")
    }
}
