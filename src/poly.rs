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
use inertia_algebra::ops::{Deref, DerefMut};
use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};


pub trait IntoPolyRing: Ring + Sized {
    type Inner: PolynomialRing<Self>; 
}

pub type InnerPolyRing<T> = <T as IntoPolyRing>::Inner;
pub type InnerPoly<T> = <InnerPolyRing<T> as PolynomialRing<T>>::Element;

pub trait PolynomialRing<T: Ring>:
    Ring<Element=<Self as PolynomialRing<T>>::Element>
{
    type Element: PolynomialRingElement<T, Parent=Self>;

    /// Initialize the `PolynomialRing` from a base ring and a `String` representing
    /// the variable used for printing.
    fn init<S: Into<String>>(ring: &T, var: S) -> Self;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    /// Return the variable as a `String`.
    fn var(&self) -> String;

    fn set_var<S: Into<String>>(&mut self, var: S);

    #[inline]
    fn nvars(&self) -> i64 {
        1
    }

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

pub trait PolynomialRingElement<T: Ring>:
    RingElement<Parent=<Self as PolynomialRingElement<T>>::Parent>
{
    type Parent: PolynomialRing<T, Element=Self>;
    
    type Borrow<'a>: Deref<Target=Elem<T>> where T: 'a;
    type BorrowMut<'a>: DerefMut<Target=Elem<T>> where T: 'a;
    
    //fn parent(&self) -> <Self as PolynomialRingElement<T>>::Parent;

    /// Return a reference to the base ring.
    fn base_ring(&self) -> &T;

    /// Return the variable as a `String`.
    fn var(&self) -> String;

    /// Return the degree of the polynomial, which is always the length - 1.
    #[inline]
    fn degree(&self) -> i64 {
        (self.len() - 1).try_into().unwrap()
    }

    /// Return the length of the polynomial, which is always the degree + 1.
    fn len(&self) -> usize;

    fn get_coefficient(&self, i: usize) -> Elem<T>;
    
    #[inline]
    fn get_coeff(&self, i: usize) -> Elem<T> {
        self.get_coefficient(i)
    }

    fn set_coefficient(&mut self, i: usize, coeff: Elem<T>);
    
    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: Elem<T>) {
        self.set_coefficient(i, coeff);
    }

    fn get_coefficients(&self) -> Vec<Elem<T>>;
    
    #[inline]
    fn get_coeffs(&self) -> Vec<Elem<T>> {
        self.get_coefficients()
    }

    #[inline]
    fn is_generic(&self) -> bool {
        false
    }
}

///////////////////////////////////////////////////////////////////
// PolyRing<T>
///////////////////////////////////////////////////////////////////

//#[derive(Clone, Debug)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolyRing<T: IntoPolyRing> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerPolyRing<T>: Serialize",
            deserialize = "InnerPolyRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerPolyRing<T>,
}

impl<T: IntoPolyRing> PolyRing<T> {
    #[inline]
    pub fn inner(&self) -> &InnerPolyRing<T> {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut InnerPolyRing<T> {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> InnerPolyRing<T> {
        self.inner
    }
}

impl<S, T: IntoPolyRing> New<S> for PolyRing<T>
where
    InnerPolyRing<T>: New<S>
{
    #[inline]
    fn new(&self, val: S) -> Poly<T> {
        Poly {
            inner: self.inner().new(val)
        }
    }
}

impl<T: IntoPolyRing> fmt::Display for PolyRing<T> where
    InnerPolyRing<T>: fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<T: IntoPolyRing> Eq for PolyRing<T> where InnerPolyRing<T>: Eq {}

impl<T: IntoPolyRing> PartialEq for PolyRing<T> 
where 
    InnerPolyRing<T>: PartialEq 
{
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.inner() == rhs.inner()
    }
}

impl<T: IntoPolyRing> Hash for PolyRing<T> 
where
    InnerPolyRing<T>: Hash
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: IntoPolyRing> Parent for PolyRing<T> {
    type Element = Poly<T>;
}

impl<T: IntoPolyRing> Identity<Additive> for PolyRing<T> {
    #[inline]
    fn identity(&self) -> Poly<T> {
        Poly {
            inner: self.inner().zero()
        }
    }
}

impl<T: IntoPolyRing> Divisible<Additive> for PolyRing<T> {}

impl<T: IntoPolyRing> Associative<Additive> for PolyRing<T> {}

impl<T: IntoPolyRing> Commutative<Additive> for PolyRing<T> {}

impl<T: IntoPolyRing> Identity<Multiplicative> for PolyRing<T> {
    #[inline]
    fn identity(&self) -> Poly<T> {
        Poly {
            inner: self.inner().one()
        }
    }
}

impl<T: IntoPolyRing> Associative<Multiplicative> for PolyRing<T> {}

impl<T: IntoPolyRing> Commutative<Multiplicative> for PolyRing<T> {}

impl<T: IntoPolyRing> Distributive for PolyRing<T> {}

impl<T: IntoPolyRing> PolynomialRing<T> for PolyRing<T> {
    type Element = Poly<T>;

    #[inline]
    fn init<S: Into<String>>(ring: &T, var: S) -> Self {
        PolyRing {
            inner: InnerPolyRing::<T>::init(ring, var),
        }
    }

    #[inline]
    fn base_ring(&self) -> &T {
        self.inner().base_ring()
    }

    #[inline]
    fn var(&self) -> String {
        self.inner().var()
    }

    #[inline]
    fn set_var<S: Into<String>>(&mut self, var: S) {
        self.inner_mut().set_var(var)
    }
    
    #[inline]
    fn is_generic(&self) -> bool {
        self.inner().is_generic()
    }
}


///////////////////////////////////////////////////////////////////
// Poly<T>
///////////////////////////////////////////////////////////////////

//#[derive(Clone, Debug)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Poly<T: IntoPolyRing> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerPoly<T>: Serialize",
            deserialize = "InnerPoly<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerPoly<T>,
}

impl<T: IntoPolyRing> Poly<T> {
    #[inline]
    pub fn inner(&self) -> &InnerPoly<T> {
        &self.inner
    }

    #[inline]
    pub fn inner_mut(&mut self) -> &mut InnerPoly<T> {
        &mut self.inner
    }

    #[inline]
    pub fn into_inner(self) -> InnerPoly<T> {
        self.inner
    }
    
    /*
    #[inline]
    fn coeff(&self, i: usize) -> Option<&T> {
        self.inner().coeff(i)
    }

    #[inline]
    fn coeff_mut(&mut self, i: usize) -> &mut T {
        self.inner_mut().coeff_mut(i)
    }

    #[inline]
    fn coefficients(&self) -> &Vec<T> {
        self.inner().coefficients()
    }
    
    #[inline]
    fn into_coefficients(self) -> Vec<T> {
        self.into_inner().into_coefficients()
    }

    #[inline]
    fn coefficients_mut(&mut self) -> &mut Vec<T> {
        self.inner_mut().coefficients_mut()
    }

    #[inline]
    fn set_coeff(&mut self, i: usize, coeff: T) -> T {
        self.inner_mut().set_coeff(i, coeff)
    }
    */
}

impl<T: IntoPolyRing> fmt::Display for Poly<T>
where
    InnerPoly<T>: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner().fmt(f)
    }
}

impl<T: IntoPolyRing> Eq for Poly<T> {}

impl<S: IntoPolyRing, T: IntoPolyRing> PartialEq<Poly<S>> for Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: IntoPolyRing, T: IntoPolyRing> PartialEq<&Poly<S>> for Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &&Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: IntoPolyRing, T: IntoPolyRing> PartialEq<Poly<S>> for &Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<T: IntoPolyRing + Hash> Hash for Poly<T>
where
    InnerPoly<T>: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: IntoPolyRing> Element for Poly<T> {
    type Parent = PolyRing<T>;
 
    #[inline]
    fn parent(&self) -> PolyRing<T> {
        PolyRing {
            inner: self.inner().parent(),
        }
    }
}

impl<T: IntoPolyRing> Operation<Additive> for Poly<T> {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Poly {
            inner: self.inner().op(Additive, right.inner())
        }
    }
}

impl<T: IntoPolyRing> IsIdentity<Additive> for Poly<T> {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_zero()
    }
}

impl<T: IntoPolyRing> TwoSidedInverse<Additive> for Poly<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        Poly {
            inner: self.inner().two_sided_inverse()
        }
    }
}

impl<T: IntoPolyRing> Operation<Multiplicative> for Poly<T> {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Poly {
            inner: self.inner().op(Multiplicative, right.inner())
        }
    }
}

impl<T: IntoPolyRing> IsIdentity<Multiplicative> for Poly<T> {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_one()
    }
}

impl<T: IntoPolyRing> PolynomialRingElement<T> for Poly<T> {
    type Parent = PolyRing<T>;

    type Borrow<'a> = &'a Elem<T> where T: 'a;
    type BorrowMut<'a> = &'a mut Elem<T> where T: 'a;

    #[inline]
    fn base_ring(&self) -> &T {
        self.inner().base_ring()
    }

    #[inline]
    fn var(&self) -> String {
        self.inner().var()
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner().len()
    }

    #[inline]
    fn get_coefficient(&self, i: usize) -> Elem<T> {
        self.inner().get_coefficient(i)
    }
    
    #[inline]
    fn set_coefficient(&mut self, i: usize, coeff: Elem<T>) {
        self.inner_mut().set_coefficient(i, coeff);
    }

    #[inline]
    fn get_coefficients(&self) -> Vec<Elem<T>> {
        self.inner().get_coefficients()
    }

    #[inline]
    fn is_generic(&self) -> bool {
        self.inner().is_generic()
    }
}

