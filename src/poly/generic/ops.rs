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


use inertia_algebra::structures::*;
use inertia_algebra::ops::*;

use crate::poly::{PolynomialRingElement, generic::GenericPoly};


// Neg
impl<T: Ring> NegAssign for GenericPoly<T> {
    #[inline]
    fn neg_assign(&mut self) {
        for c in self.coeffs_mut().iter_mut() {
            c.neg_assign();
        }
    }
}

impl<T: Ring> Neg for GenericPoly<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn neg(mut self) -> Self::Output {
        self.neg_assign();
        self
    }
}

impl<T: Ring> Neg for &GenericPoly<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn neg(self) -> Self::Output {
        let res = self.clone();
        res.neg()
    }
}

// Add
impl<T: Ring> AddAssign<&GenericPoly<T>> for GenericPoly<T> {
    fn add_assign(&mut self, rhs: &GenericPoly<T>) {
        let max = std::cmp::max(self.len(), rhs.len());
        self.resize(max);

        for (i, c) in rhs.coefficients().iter().enumerate() {
            self.coeff_mut(i).add_assign(c);
        }
    }
}

impl<T: Ring> AddFrom<&GenericPoly<T>> for GenericPoly<T> {
    fn add_from(&mut self, lhs: &GenericPoly<T>) {
        let max = std::cmp::max(self.len(), lhs.len());
        self.resize(max);

        for (i, c) in lhs.coefficients().iter().enumerate() {
            self.coeff_mut(i).add_from(c);
        }
    }
}

forward_binop! {
    GenericPoly<T: Ring>
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
}

// Sub

impl<T: Ring> SubAssign<&Self> for GenericPoly<T> {
    fn sub_assign(&mut self, rhs: &Self) {
        let max = std::cmp::max(self.len(), rhs.len());
        self.resize(max);

        for (i, c) in rhs.coefficients().iter().enumerate() {
            self.coeff_mut(i).sub_assign(c);
        }
    }
}

impl<T: Ring> SubFrom<&Self> for GenericPoly<T> {
    fn sub_from(&mut self, rhs: &Self) {
        let max = std::cmp::max(self.len(), rhs.len());
        self.resize(max);

        for (i, c) in rhs.coefficients().iter().enumerate() {
            self.coeff_mut(i).sub_from(c);
        }
    }
}

forward_binop! {
    GenericPoly<T: Ring>
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
}

// Mul

impl<T: Ring> MulAssign<&Self> for GenericPoly<T> {
    fn mul_assign(&mut self, _rhs: &Self) {
        //self.mul_assign_best(rhs);
        unimplemented!()
    }
}

impl<T: Ring> MulFrom<&Self> for GenericPoly<T> {
    fn mul_from(&mut self, _lhs: &Self) {
        //self.mul_from_best(lhs);
        unimplemented!()
    }
}

forward_binop! {
    GenericPoly<T: Ring>
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
}
