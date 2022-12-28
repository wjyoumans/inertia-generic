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


use crate::Coerce;
use crate::polynomial::generic::{GenericPoly, GenericPolyRing};
use crate::structures::*;

use std::rc::Rc;

impl<T: Ring> Coerce<GenericPoly<T>> for GenericPolyRing<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn coerce(&self, val: GenericPoly<T>) -> GenericPoly<T> {
        val
    }
}


impl<T: Ring> Coerce<&GenericPoly<T>> for GenericPolyRing<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn coerce(&self, val: &GenericPoly<T>) -> GenericPoly<T> {
        val.clone()
    }
    
}

/*
impl<T: Ring> Coerce<Elem<T>> for GenericPolyRing<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn coerce(&self, val: Elem<T>) -> GenericPoly<T> {
        unimplemented!()
    }
}

impl<T: Ring> Coerce<&Elem<T>> for GenericPolyRing<T> {
    type Output = GenericPoly<T>;
    #[inline]
    fn coerce(&self, val: &Elem<T>) -> GenericPoly<T> {
        unimplemented!()
    }
}
*/

impl<S, T, const CAP: usize> Coerce<[S; CAP]> for GenericPolyRing<T>
where
    T: Ring + Coerce<S, Output=<T as Ring>::Element>,
{
    type Output = GenericPoly<T>;
    fn coerce(&self, coeffs: [S; CAP]) -> GenericPoly<T> {
        let mut res = self.zero();
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, self.base_ring().coerce(x));
        }
        res
    }
}

impl<'a, S, T> Coerce<&'a [S]> for GenericPolyRing<T>
where
    T: Ring + Coerce<&'a S, Output=<T as Ring>::Element>,
{
    type Output = GenericPoly<T>;
    fn coerce(&self, coeffs: &'a [S]) -> GenericPoly<T> {
        let mut res = self.zero();
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, self.base_ring().coerce(x));
        }
        res
    }
}

impl<S, T> Coerce<Vec<S>> for GenericPolyRing<T> 
where
    T: Ring<Element=S>
{
    type Output = GenericPoly<T>;
    #[inline]
    fn coerce(&self, coeffs: Vec<S>) -> GenericPoly<T> {
        GenericPoly {
            coeffs,
            ctx: Rc::clone(&self.ctx),
        }
    }
}
