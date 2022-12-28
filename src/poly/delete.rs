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


//use crate::Coerce;
use crate::polynomial::*;

use inertia_algebra::*;
use inertia_algebra::structures::*;
use inertia_algebra::properties::*;

use std::fmt;
use std::hash::{Hash, Hasher};

#[cfg(feature = "serde")]
use serde::{Serialize, Deserialize};

//mod coerce;
mod ops;

///////////////////////////////////////////////////////////////////
// PolyRing<T>
///////////////////////////////////////////////////////////////////

//#[derive(Clone, Debug)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct PolyRing<T: PolyableRing> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerPolyRing<T>: Serialize",
            deserialize = "InnerPolyRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerPolyRing<T>,
}

impl<T: PolyableRing> fmt::Display for PolyRing<T> where
    InnerPolyRing<T>: fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.inner())
    }
}

impl<T: PolyableRing> Eq for PolyRing<T> where InnerPolyRing<T>: Eq {}

impl<T: PolyableRing> PartialEq for PolyRing<T> 
where 
    InnerPolyRing<T>: PartialEq 
{
    #[inline]
    fn eq(&self, rhs: &Self) -> bool {
        self.inner() == rhs.inner()
    }
}

impl<T: PolyableRing> Hash for PolyRing<T> 
where
    InnerPolyRing<T>: Hash
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: PolyableRing> Parent for PolyRing<T> {
    type Element = Poly<T>;
}

impl<T: PolyableRing> Identity<Additive> for PolyRing<T> {
    #[inline]
    fn identity(&self) -> Poly<T> {
        Poly {
            inner: self.inner().zero()
        }
    }
}

impl<T: PolyableRing> Divisible<Additive> for PolyRing<T> {}
impl<T: PolyableRing> Associative<Additive> for PolyRing<T> {}
impl<T: PolyableRing> Commutative<Additive> for PolyRing<T> {}

impl<T: PolyableRing> Identity<Multiplicative> for PolyRing<T> {
    #[inline]
    fn identity(&self) -> Poly<T> {
        Poly {
            inner: self.inner().one()
        }
    }
}

impl<T: PolyableRing> Associative<Multiplicative> for PolyRing<T> {}
impl<T: PolyableRing> Commutative<Multiplicative> for PolyRing<T> {}

impl<T: PolyableRing> Distributive for PolyRing<T> {}

impl<T: PolyableRing> PolynomialRing<T> for PolyRing<T> {
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

impl<T: PolyableRing> PolyRing<T> {
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

/*
impl<T: PolyableRing> PolyRing<T> {
    pub fn new<S>(&self, val: S) -> Poly<T> 
    where
        Self: Coerce<S, Output=Poly<T>>
    {
        self.coerce(val)
    }
}
*/


///////////////////////////////////////////////////////////////////
// Poly<T>
///////////////////////////////////////////////////////////////////

//#[derive(Clone, Debug)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct Poly<T: PolyableRing> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "InnerPoly<T>: Serialize",
            deserialize = "InnerPoly<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) inner: InnerPoly<T>,
}

impl<T: PolyableRing> fmt::Display for Poly<T>
where
    InnerPoly<T>: fmt::Display,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.inner().fmt(f)
    }
}

impl<T: PolyableRing> Eq for Poly<T> {}

impl<S: PolyableRing, T: PolyableRing> PartialEq<Poly<S>> for Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: PolyableRing, T: PolyableRing> PartialEq<&Poly<S>> for Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &&Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<S: PolyableRing, T: PolyableRing> PartialEq<Poly<S>> for &Poly<T>
where
    InnerPoly<T>: PartialEq<InnerPoly<S>>,
{
    #[inline]
    fn eq(&self, other: &Poly<S>) -> bool {
        self.inner().eq(other.inner())
    }
}

impl<T: PolyableRing + Hash> Hash for Poly<T>
where
    InnerPoly<T>: Hash,
{
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.inner().hash(state)
    }
}

impl<T: PolyableRing> Element for Poly<T> {
    type Parent = PolyRing<T>;
 
    #[inline]
    fn parent(&self) -> PolyRing<T> {
        PolyRing {
            inner: self.inner().parent(),
        }
    }
}

impl<T: PolyableRing> Operation<Additive> for Poly<T> {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Poly {
            inner: self.inner().op(Additive, right.inner())
        }
    }
}

impl<T: PolyableRing> IsIdentity<Additive> for Poly<T> {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_zero()
    }
}

impl<T: PolyableRing> TwoSidedInverse<Additive> for Poly<T> {
    #[inline]
    fn two_sided_inverse(&self) -> Self {
        Poly {
            inner: self.inner().two_sided_inverse()
        }
    }
}

impl<T: PolyableRing> Operation<Multiplicative> for Poly<T> {
    #[inline]
    fn operate(&self, right: &Self) -> Self {
        Poly {
            inner: self.inner().op(Multiplicative, right.inner())
        }
    }
}

impl<T: PolyableRing> IsIdentity<Multiplicative> for Poly<T> {
    #[inline]
    fn is_identity(&self) -> bool {
        self.inner().is_one()
    }
}

impl<T: PolyableRing> PolynomialRingElement<T> for Poly<T> {
    type Parent = PolyRing<T>;

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

impl<T: PolyableRing> Poly<T> {
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
}

/*
impl<T: PolyableRing> Poly<T> {
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

}*/
