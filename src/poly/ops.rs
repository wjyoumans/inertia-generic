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

use inertia_algebra::ops::*;
use crate::poly::*;


derive_unop! {
    Poly<T: PolyableRing>, InnerPoly<T>;
    Neg, neg
    NegAssign, neg_assign
}

derive_binop! {
    Poly<T: PolyableRing>, InnerPoly<T>;
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
}

derive_binop! {
    Poly<T: PolyableRing>, InnerPoly<T>;
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
    AssignSub, assign_sub
}

derive_binop! {
    Poly<T: PolyableRing>, InnerPoly<T>;
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
    AssignMul, assign_mul
}
