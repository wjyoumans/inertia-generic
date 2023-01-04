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

use crate::mat::generic::GenericMat;

// Neg
impl<T: Ring> NegAssign for GenericMat<T> {
    #[inline]
    fn neg_assign(&mut self) {
        unimplemented!()
    }
}

impl<T: Ring> Neg for GenericMat<T> {
    type Output = GenericMat<T>;
    #[inline]
    fn neg(mut self) -> Self::Output {
        self.neg_assign();
        self
    }
}

impl<T: Ring> Neg for &GenericMat<T> {
    type Output = GenericMat<T>;
    #[inline]
    fn neg(self) -> Self::Output {
        let res = self.clone();
        res.neg()
    }
}

// Add
impl<T: Ring> AddAssign<&GenericMat<T>> for GenericMat<T> {
    fn add_assign(&mut self, _rhs: &GenericMat<T>) {
        unimplemented!()
    }
}

impl<T: Ring> AddFrom<&GenericMat<T>> for GenericMat<T> {
    fn add_from(&mut self, _lhs: &GenericMat<T>) {
        unimplemented!()
    }
}

forward_binop! {
    GenericMat<T: Ring>
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
}

// Sub

impl<T: Ring> SubAssign<&Self> for GenericMat<T> {
    fn sub_assign(&mut self, _rhs: &Self) {
        unimplemented!()
    }
}

impl<T: Ring> SubFrom<&Self> for GenericMat<T> {
    fn sub_from(&mut self, _lhs: &Self) {
        unimplemented!()
    }
}

forward_binop! {
    GenericMat<T: Ring>
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
}

/* TODO: op guard to check matrix dim
// Mul

impl<T: Ring> MulAssign<&Self> for GenericMat<T> {
    fn mul_assign(&mut self, _rhs: &Self) {
        unimplemented!()
    }
}

impl<T: Ring> MulFrom<&Self> for GenericMat<T> {
    fn mul_from(&mut self, _lhs: &Self) {
        unimplemented!()
    }
}

forward_binop! {
    GenericMat<T: Ring>
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
}
*/
