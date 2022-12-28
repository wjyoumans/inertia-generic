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
use crate::polynomial::{Poly, PolyRing, PolyableRing};
use crate::structures::*;

/*
impl<T: PolyableRing> Coerce<Poly<T>> for PolyRing<T> {
    type Output = Poly<T>;
    #[inline]
    fn coerce(&self, val: Poly<T>) -> Poly<T> {
        val
    }
}

impl<T: PolyableRing> Coerce<&Poly<T>> for PolyRing<T> {
    type Output = Poly<T>;
    #[inline]
    fn coerce(&self, val: &Poly<T>) -> Poly<T> {
        val.clone()
    }
    
}
*/

impl<S, T: PolyableRing> Coerce<S> for PolyRing<T>
where
    InnerPolyRing<T>: Coerce<S, Output=InnerPoly<T>>
{
    type Output = Poly<T>;
    fn coerce(&self, val: S) -> Self::Output {
        Poly { inner: self.inner().coerce(val) }
    }
}
