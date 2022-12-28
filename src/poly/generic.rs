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
//use crate::structures::*;

use crate::poly::*;

use inertia_algebra::*;
use inertia_algebra::structures::*;
use inertia_algebra::properties::*;

use std::cell::RefCell;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod ops;
//mod coerce;

///////////////////////////////////////////////////////////////////
// GenericPolyRing<T>
///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct GenericPolyCtx<T> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    pub(crate) base_ring: T,
    pub(crate) var: RefCell<String>,
}

impl<T> GenericPolyCtx<T> {
    pub fn new<V: Into<String>>(base_ring: T, var: V) -> Self {
        GenericPolyCtx {
            base_ring,
            var: RefCell::new(var.into())
        }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericPolyRing<T> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericPolyCtx<T>>,
}

impl<T: Ring + fmt::Display> fmt::Display for GenericPolyRing<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Univariate polynomial ring in {} over {}",
            self.var(),
            self.base_ring()
        )
    }
}

impl<T: Ring + PartialEq> Eq for GenericPolyRing<T> {}

impl<T: Ring + PartialEq> PartialEq for GenericPolyRing<T> {
    #[inline]
    fn eq(&self, rhs: &GenericPolyRing<T>) -> bool {
        Rc::ptr_eq(&self.ctx, &rhs.ctx) ||
            self.base_ring() == rhs.base_ring()
    }
}

impl<T: Ring + Hash> Hash for GenericPolyRing<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nvars().hash(state);
    }
}

impl<T: Ring> Parent for GenericPolyRing<T> {
    type Element = GenericPoly<T>;
}

impl<T: Ring> Identity<Additive> for GenericPolyRing<T> {
    #[inline]
    fn identity(&self) -> GenericPoly<T> {
        GenericPoly {
            coeffs: vec![self.base_ring().zero()],
            ctx: Rc::clone(&self.ctx),
        }
    }
}

impl<T: Ring> Divisible<Additive> for GenericPolyRing<T> {}
impl<T: Ring> Associative<Additive> for GenericPolyRing<T> {}
impl<T: Ring> Commutative<Additive> for GenericPolyRing<T> {}

impl<T: Ring> Identity<Multiplicative> for GenericPolyRing<T> {
    #[inline]
    fn identity(&self) -> GenericPoly<T> {
        GenericPoly {
            coeffs: vec![self.base_ring().one()],
            ctx: Rc::clone(&self.ctx),
        }
    }
}

impl<T: Ring> Associative<Multiplicative> for GenericPolyRing<T> {}
impl<T: Ring> Commutative<Multiplicative> for GenericPolyRing<T> {}

impl<T: Ring> Distributive for GenericPolyRing<T> {}

impl<T: Ring> PolynomialRing<T> for GenericPolyRing<T> {
    type Element = GenericPoly<T>;

    #[inline]
    fn init<S: Into<String>>(base_ring: &T, var: S) -> Self {
        GenericPolyRing {
            ctx: Rc::new(GenericPolyCtx::new(base_ring.clone(), var))
        }
    }

    #[inline]
    fn base_ring(&self) -> &T {
        &self.ctx.base_ring
    }

    #[inline]
    fn var(&self) -> String {
        self.ctx.var.borrow().clone()
    }

    #[inline]
    fn set_var<S: Into<String>>(&mut self, var: S) {
        self.ctx.var.replace(var.into());
    }
    
    #[inline]
    fn is_generic(&self) -> bool { true }
}

impl<T: Ring> New<&GenericPoly<T>> for GenericPolyRing<T> {
    #[inline]
    fn new(&self, val: &GenericPoly<T>) -> GenericPoly<T> {
        val.clone()
    }
}

impl<T> New<&Poly<T>> for GenericPolyRing<T>
where
    T: PolyableRing<InnerPolyRing=GenericPolyRing<T>>
{
    #[inline]
    fn new(&self, val: &Poly<T>) -> GenericPoly<T> {
        val.inner().clone()
    }
}

impl<S, T, const CAP: usize> New<[S; CAP]> for GenericPolyRing<T>
where
    T: Ring + New<S>,
{
    fn new(&self, coeffs: [S; CAP]) -> GenericPoly<T> {
        let mut res = self.zero();
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, self.base_ring().new(x));
        }
        res
    }
}

impl<'a, S, T> New<&'a [S]> for GenericPolyRing<T>
where
    T: Ring + New<&'a S>,
{
    fn new(&self, coeffs: &'a [S]) -> GenericPoly<T> {
        let mut res = self.zero();
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, self.base_ring().new(x));
        }
        res
    }
}

impl<S, T> New<Vec<S>> for GenericPolyRing<T> 
where
    T: Ring<Element=S>
{
    #[inline]
    fn new(&self, coeffs: Vec<S>) -> GenericPoly<T> {
        GenericPoly {
            coeffs,
            ctx: Rc::clone(&self.ctx),
        }
    }
}

///////////////////////////////////////////////////////////////////
// GenericPoly<T>
///////////////////////////////////////////////////////////////////

//#[derive(Debug, Clone)]
#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericPoly<T: Ring> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericPolyCtx<T>>,
    pub(crate) coeffs: Vec<Elem<T>>,
}

impl<T: Ring> fmt::Display for GenericPoly<T>
where
    <T as Ring>::Element: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let coeffs = &self.coeffs;
        let len = coeffs.len();
        let x = self.var();

        let mut out = Vec::with_capacity(len);
        if len == 0  {
            panic!("How is the length = 0?")
        } else if len == 1 || (len > 1 && !coeffs[0].is_zero()) {
            out.push(format!("{}", coeffs[0]));
        }
        if len > 1 && !coeffs[1].is_zero() {
            if coeffs[1].is_one() {
                out.push(format!("{}", x));
            } else {
                out.push(format!("{}*{}", coeffs[1], x));
            }
        }
        if len > 2 {
            for i in 2..len {
                if !coeffs[i].is_zero() {
                    if coeffs[i].is_one() {
                        out.push(format!("{}^{}", x, i));
                    } else {
                        out.push(format!("{}*{}^{}", coeffs[i], x, i));
                    }
                }
            }
        }
        out.reverse();
        write!(f, "{}", out.join(" + "))
    }
}

impl<T: Ring> Eq for GenericPoly<T> {}

impl<S: Ring, T: Ring> PartialEq<GenericPoly<S>> for GenericPoly<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
{
    fn eq(&self, rhs: &GenericPoly<S>) -> bool {
        let len = self.len();
        if rhs.len() != len {
            return false;
        }

        let c1 = &self.coeffs;
        let c2 = &rhs.coeffs;
        for i in 0..len {
            if c1[i] != c2[i] {
                return false;
            }
        }
        true
    }
}

impl<S: Ring, T: Ring> PartialEq<&GenericPoly<S>> for GenericPoly<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
{
    #[inline]
    fn eq(&self, rhs: &&GenericPoly<S>) -> bool {
        (&self).eq(rhs)
    }
}

impl<S: Ring, T: Ring> PartialEq<GenericPoly<S>> for &GenericPoly<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
{
    #[inline]
    fn eq(&self, rhs: &GenericPoly<S>) -> bool {
        self.eq(&rhs)
    }
}

impl<T: Ring + Hash> Hash for GenericPoly<T>
where
    <T as Ring>::Element: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.coefficients().hash(state);
    }
}

impl<T: Ring> Element for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;

    #[inline]
    fn parent(&self) -> GenericPolyRing<T> {
        GenericPolyRing {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

impl<T: Ring> Operation<Additive> for GenericPoly<T> {
    fn operate(&self, _right: &Self) -> Self {
        unimplemented!()
    }
}

impl<T: Ring> IsIdentity<Additive> for GenericPoly<T> {
    fn is_identity(&self) -> bool {
        self.degree() == 0 && self.coeff(0).unwrap().is_zero()
    }
}

impl<T: Ring> TwoSidedInverse<Additive> for GenericPoly<T> {
    fn two_sided_inverse(&self) -> Self {
        unimplemented!()
    }
}

impl<T: Ring> Operation<Multiplicative> for GenericPoly<T> {
    fn operate(&self, _right: &Self) -> Self {
        unimplemented!()
    }
}

impl<T: Ring> IsIdentity<Multiplicative> for GenericPoly<T> {
    fn is_identity(&self) -> bool {
        self.degree() == 0 && self.coeff(0).unwrap().is_one()
    }
}

impl<T: Ring> PolynomialRingElement<T> for GenericPoly<T> {
    type Parent = GenericPolyRing<T>;

    #[inline]
    fn base_ring(&self) -> &T {
        &self.ctx.base_ring
    }

    #[inline]
    fn var(&self) -> String {
        self.ctx.var.borrow().clone()
    }

    #[inline]
    fn len(&self) -> usize {
        self.coeffs.len()
    }
    
    fn get_coefficient(&self, i: usize) -> Elem<T> {
        let coeff = self.coeffs.get(i);
        if let Some(c) = coeff {
            c.clone()
        } else {
            self.base_ring().zero()
        }
    }
    
    fn set_coefficient(&mut self, i: usize, coeff: Elem<T>) {
        let z = self.base_ring().zero();
        let coeffs = self.coefficients_mut();
        if i >= coeffs.len() {
            coeffs.resize(i + 1, z);
        }
        coeffs.push(coeff);
        let _ = coeffs.swap_remove(i);
        //self.normalize();
    }
    
    #[inline]
    fn get_coefficients(&self) -> Vec<Elem<T>> {
        self.coeffs.clone()
    }

    #[inline]
    fn is_generic(&self) -> bool { true }
}

impl<T: Ring> GenericPoly<T>
where
    //T: PartialEq,
{
    /*
    // TODO: Dont resize? Check what flint does.
    // remove trailing zeros and ensure len >= 1
    fn normalize(&mut self) {
        let len = self.len();
        if len != 1 {
            let z = self.base_ring().default();
            let coeffs = self.coefficients_mut();
            if let Some(pos) = coeffs.iter().rev().position(|x| x != &z) {
                coeffs.truncate(len - pos);
            } else {
                coeffs.clear();
            }
        }
    }*/

    #[inline]
    pub fn resize(&mut self, new_len: usize) {
        self.coeffs.resize(new_len, self.base_ring().zero());
    }

    #[allow(dead_code)]
    #[inline]
    pub fn rotate_right(&mut self, k: usize) {
        self.coeffs.rotate_right(k);
    }

    #[allow(dead_code)]
    #[inline]
    pub fn rotate_left(&mut self, k: usize) {
        self.coeffs.rotate_left(k);
    }

    #[inline]
    pub fn coefficient(&self, i: usize) -> Option<&Elem<T>> {
        self.coeffs.get(i)
    }

    #[inline]
    pub fn coeff(&self, i: usize) -> Option<&Elem<T>> {
        self.coefficient(i)
    }

    #[inline]
    pub fn coefficient_mut(&mut self, i: usize) -> &mut Elem<T> {
        if i >= self.len() {
            self.resize(i+1);
        }
        self.coeffs.get_mut(i).expect("FIXME: Mistake made when resizing!")
    }

    #[inline]
    pub fn coeff_mut(&mut self, i: usize) -> &mut Elem<T> {
        self.coefficient_mut(i)
    }
    
    #[inline]
    pub fn coefficients(&self) -> &Vec<Elem<T>> {
        &self.coeffs
    }

    #[inline]
    pub fn coeffs(&self) -> &Vec<Elem<T>> {
        self.coefficients()
    }

    #[inline]
    pub fn coefficients_mut(&mut self) -> &mut Vec<Elem<T>> {
        &mut self.coeffs
    }

    #[inline]
    pub fn coeffs_mut(&mut self) -> &mut Vec<Elem<T>> {
        self.coefficients_mut()
    }

    #[inline]
    pub fn into_coefficients(self) -> Vec<Elem<T>> {
        self.coeffs
    }

    #[inline]
    pub fn into_coeffs(self) -> Vec<Elem<T>> {
        self.into_coefficients()
    }

    /*
    // TODO: use addmul/muladd? Better choice of val/ref for inputs? assign?
    pub fn mul_classical<'a>(&'a self, rhs: &'a GenericPoly<T>) -> Self
    where
        &'a T: Mul<Output = T>,
        T: AddAssign,
    {
        let mut res = self.parent().zero();
        res.resize(self.len() + rhs.len() - 1);

        for (i, a) in self.coefficients().iter().enumerate() {
            for (j, b) in rhs.coefficients().iter().enumerate() {
                (*res.coeff_mut(i + j)).add_assign(a.mul(b));
            }
        }
        res
    }
   
    pub fn mul_assign_classical<'a>(&mut self, rhs: &'a GenericPoly<T>)
    where
        T: for<'b> MulAssign<&'b T>,
        T: AddAssign<&'a T>,
    {
        let temp = self.clone();
        self.resize(self.len() + rhs.len() - 1);

        for (i, a) in temp.coefficients().iter().enumerate() {
            for (j, b) in rhs.coefficients().iter().enumerate() {
                (*self.coeff_mut(i + j)).add_assign(a.mul(b));
            }
        }
    }
    */

    //pub fn mul_karatsuba(&self, rhs: &GenericPoly<T>) -> Self {
    //}

    /// Make a best guess for an optimal multiplication algorithm.
    pub fn mul_assign_best(&mut self, _rhs: &GenericPoly<T>) {
        unimplemented!()
    }
    
    // TODO: assumes commutative!
    /// Make a best guess for an optimal multiplication algorithm.
    #[inline]
    pub fn mul_from_best(&mut self, lhs: &GenericPoly<T>) {
        self.mul_assign_best(lhs);
    }
}
