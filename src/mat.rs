/*
 *  Copyright (C) 2021 William Youmans
 *
 *  This program is free software: you can redistribute it and/or modify
 *  it under the terms of the GNU General Public License as published by
 *  the Free Software Foundation, either version 3 of the License, or
 *  (at your option) any later version.
 *
 *  This program is distributed in the hope that it will be useful,
 *  but WITHOUT ANY WARRANTY; without even the implied warranty of
 *  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 *  GNU General Public License for more details.
 *
 *  You should have received a copy of the GNU General Public License
 *  along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod generic;
mod ops;

use crate::New;
use inertia_algebra::*;
use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

pub trait IntoMatSpace: Ring + Sized {
    type Inner: MatrixSpace<Self>; 
}

pub type InnerMatSpace<T> = <T as IntoMatSpace>::Inner;
pub type InnerMat<T> = <InnerMatSpace<T> as MatrixSpace<T>>::Element;

pub trait MatrixSpace<T: Ring>:
    AdditiveGroupAbelian<Element=<Self as MatrixSpace<T>>::Element>
{
    type Element: MatrixSpaceElement<T, Parent=Self>;

    fn init<D: Into<u64>>(ring: &T, nrows: D, ncols: D) -> Self;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    fn nrows(&self) -> usize;

    fn ncols(&self) -> usize;

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

pub trait MatrixSpaceElement<T: Ring>:
    AdditiveGroupAbelianElement<Parent=<Self as MatrixSpaceElement<T>>::Parent>
{
    type Parent: MatrixSpace<T, Element=Self>;

    //type BorrowCoeff<'a>: Deref<Target=T>;
    //type BorrowCoeffMut<'a>: DerefMut<Target=T>;
    
    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    fn len(&self) -> usize;
    
    fn nrows(&self) -> usize;

    fn ncols(&self) -> usize;

    fn get_entry(&self, i: usize, j: usize) -> Option<Elem<T>>;

    fn set_entry(&mut self, i: usize, j: usize, entry: Elem<T>) -> Option<Elem<T>>;
    
    fn get_entries(&self) -> Vec<Elem<T>>;
    
    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

///////////////////////////////////////////////////////////////////
// MatSpace<T>
///////////////////////////////////////////////////////////////////

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatSpace<T: IntoMatSpace> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerMatSpace<T>: Serialize",
            deserialize = "InnerMatSpace<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerMatSpace<T>,
}

impl<T: IntoMatSpace> MatSpace<T> {
    #[inline]
    pub fn inner(&self) -> &InnerMatSpace<T> {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut InnerMatSpace<T> {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> InnerMatSpace<T> {
        self.inner
    }
}

impl<S, T: IntoMatSpace> New<S> for MatSpace<T>
where
    InnerMatSpace<T>: New<S>
{
    #[inline]
    fn new(&self, val: S) -> Mat<T> {
        Mat {
            inner: self.inner().new(val)
        }
    }
}

impl<T: IntoMatSpace> fmt::Display for MatSpace<T> where
    InnerMatSpace<T>: fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<T: IntoMatSpace> Eq for MatSpace<T> where InnerMatSpace<T>: Eq {}

impl<T: IntoMatSpace> PartialEq for MatSpace<T> 
where 
    InnerMatSpace<T>: PartialEq 
{
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.inner() == rhs.inner()
    }
}

impl<T: IntoMatSpace> Hash for MatSpace<T> 
where
    InnerMatSpace<T>: Hash
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: IntoMatSpace> Parent for MatSpace<T> {
    type Element = Mat<T>;
}

impl<T: IntoMatSpace> Identity<Additive> for MatSpace<T> {
    #[inline]
    fn identity(&self) -> Mat<T> {
        Mat {
            inner: self.inner().zero()
        }
    }
}

impl<T: IntoMatSpace> Divisible<Additive> for MatSpace<T> {}

impl<T: IntoMatSpace> Associative<Additive> for MatSpace<T> {}

impl<T: IntoMatSpace> Commutative<Additive> for MatSpace<T> {}

/*
impl<T: IntoMatSpace> MatrixSpace<T> for MatSpace<T> {
    type Element = Mat<T>;

    #[inline]
    fn new(ring: &BaseRing<T>, nrows: u32, ncols: u32) -> Self {
        MatSpace {
            inner: InnerMatSpace::<T>::new(ring, nrows, ncols),
        }
    }

    #[inline]
    fn base_ring(&self) -> &BaseRing<T> {
        self.inner().base_ring()
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.inner().nrows()
    }

    #[inline]
    fn ncols(&self) -> usize {
        self.inner().ncols()
    }
    
    #[inline]
    fn is_generic(&self) -> bool {
        self.inner().is_generic()
    }
}
*/

///////////////////////////////////////////////////////////////////
// Mat<T>
///////////////////////////////////////////////////////////////////

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat<T: IntoMatSpace> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerMat<T>: Serialize",
            deserialize = "InnerMat<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerMat<T>,
}

impl<T: IntoMatSpace> Mat<T> {
    #[inline]
    pub fn inner(&self) -> &InnerMat<T> {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut InnerMat<T> {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> InnerMat<T> {
        self.inner
    }
}

impl<T: IntoMatSpace> fmt::Display for Mat<T>
where
    InnerMat<T>: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner().fmt(f)
    }
}

impl<T: IntoMatSpace> Eq for Mat<T> {}

impl<S: IntoMatSpace, T: IntoMatSpace> PartialEq<Mat<S>> for Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: IntoMatSpace, T: IntoMatSpace> PartialEq<&Mat<S>> for Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &&Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: IntoMatSpace, T: IntoMatSpace> PartialEq<Mat<S>> for &Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<T: IntoMatSpace + Hash> Hash for Mat<T>
where
    InnerMat<T>: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: IntoMatSpace> Element for Mat<T> {
    type Parent = MatSpace<T>;

    #[inline]
    fn parent(&self) -> MatSpace<T> {
        MatSpace {
            inner: self.inner().parent(),
        }
    }
}

impl<T: IntoMatSpace> Operation<Additive> for Mat<T> {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Mat {
            inner: self.inner().op(Additive, right.inner())
        }
    }
}

impl<T: IntoMatSpace> IsIdentity<Additive> for Mat<T> {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_zero()
    }
}

impl<T: IntoMatSpace> TwoSidedInverse<Additive> for Mat<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        Mat {
            inner: self.inner().two_sided_inverse()
        }
    }
}

/*
impl<T: IntoMatSpace> MatrixSpaceElement<T> for Mat<T> {
    type Parent = MatSpace<T>;

    #[inline]
    fn base_ring(&self) -> &BaseRing<T> {
        self.inner().base_ring()
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner().len()    
    }

    #[inline]
    fn nrows(&self) -> usize {
       self.inner().nrows() 
    }

    #[inline]
    fn ncols(&self) -> usize {
       self.inner().ncols() 
    }

    #[inline]
    fn entry(&self, i: usize, j: usize) -> Option<&T> {
        self.inner().entry(i, j)
    }

    #[inline]
    fn entry_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        self.inner_mut().entry_mut(i, j)
    }

    #[inline]
    fn set_entry(&mut self, i: usize, j: usize, entry: T) -> Option<T> {
        self.inner_mut().set_entry(i, j, entry)
    }

    #[inline]
    fn entries(&self) -> &Vec<T> {
        self.inner().entries()
    }

    #[inline]
    fn entries_mut(&mut self) -> &mut Vec<T> {
        self.inner_mut().entries_mut()
    }

    #[inline]
    fn into_entries(self) -> Vec<T> {
        self.into_inner().into_entries()
    }

    #[inline]
    fn is_generic(&self) -> bool {
        self.inner().is_generic()
    }
}
*/
