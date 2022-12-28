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

mod generic;
pub use generic::{GenericMatSpace, GenericMat};

use inertia_traits::*;
use inertia_traits::ops::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::*;

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

///////////////////////////////////////////////////////////////////
// MatSpace<T>
///////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct MatSpace<T: RingElement> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerMatSpace<T>: Serialize",
            deserialize = "InnerMatSpace<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerMatSpace<T>,
}

impl<T: RingElement> MatSpace<T> {
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

    #[inline]
    pub fn infer(elem: T, nrows: u32, ncols: u32) -> Self {
        MatSpace {
            inner: InnerMatSpace::<T>::new(&elem.parent(), nrows, ncols),
        }
    }
}

impl<T: RingElement> fmt::Display for MatSpace<T> where
    InnerMatSpace<T>: fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<T: RingElement> Eq for MatSpace<T> where InnerMatSpace<T>: Eq {}

impl<T: RingElement> PartialEq for MatSpace<T> 
where 
    InnerMatSpace<T>: PartialEq 
{
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.inner() == rhs.inner()
    }
}

impl<T: RingElement> Hash for MatSpace<T> 
where
    InnerMatSpace<T>: Hash
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: RingElement> Parent for MatSpace<T> {
    type Element = Mat<T>;

    #[inline]
    fn default(&self) -> Self::Element {
        Mat {
            inner: self.inner().default(),
        }
    }
}

impl<T: RingElement> MatrixSpace<T> for MatSpace<T> {
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

///////////////////////////////////////////////////////////////////
// Mat<T>
///////////////////////////////////////////////////////////////////

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Mat<T: RingElement> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerMat<T>: Serialize",
            deserialize = "InnerMat<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerMat<T>,
}

impl<T: RingElement> fmt::Display for Mat<T>
where
    InnerMat<T>: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner().fmt(f)
    }
}

impl<T: RingElement> Eq for Mat<T> {}

impl<S: RingElement, T: RingElement> PartialEq<Mat<S>> for Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: RingElement, T: RingElement> PartialEq<&Mat<S>> for Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &&Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: RingElement, T: RingElement> PartialEq<Mat<S>> for &Mat<T>
where
    InnerMat<T>: PartialEq<InnerMat<S>>,
{
    fn eq(&self, other: &Mat<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<T: RingElement + Hash> Hash for Mat<T>
where
    InnerMat<T>: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: RingElement> Mat<T> {
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

impl<T: RingElement> Element for Mat<T> {
    type Parent = MatSpace<T>;

    #[inline]
    fn parent(&self) -> MatSpace<T> {
        MatSpace {
            inner: self.inner().parent(),
        }
    }
}

impl<T: RingElement> MatrixSpaceElement<T> for Mat<T> {
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


///////////////////////////////////////////////////////////////////
// Coercion
///////////////////////////////////////////////////////////////////

impl<'a, S, T> Coerce<'a, S> for MatSpace<T> 
where
    T: RingElement,
    InnerMatSpace<T>: Coerce<'a, S> 
{
    #[inline]
    fn coerce(&self, value: S) -> Mat<T> {
        Mat { inner: self.inner().coerce(value) }
    }
}

///////////////////////////////////////////////////////////////////
// Ops
///////////////////////////////////////////////////////////////////

derive_binop! {
    Mat<T: RingElement>, InnerMat<T>;
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
}

derive_binop! {
    Mat<T: RingElement>, InnerMat<T>;
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
    AssignSub, assign_sub
}

derive_binop! {
    Mat<T: RingElement>, InnerMat<T>;
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
    AssignMul, assign_mul
}

derive_binop! {
    Mat<T: RingElement>, InnerMat<T>;
    Div, div
    DivAssign, div_assign
    DivFrom, div_from
    AssignDiv, assign_div
}

derive_binop! {
    Mat<T: RingElement>, InnerMat<T>;
    Rem, rem
    RemAssign, rem_assign
    RemFrom, rem_from
    AssignRem, assign_rem
}
