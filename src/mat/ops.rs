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
use crate::mat::*;

derive_unop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Neg, neg
    NegAssign, neg_assign
}

derive_unop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Inv, inv
    InvAssign, inv_assign
}

derive_binop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
    AssignAdd, assign_add
}

derive_binop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
    AssignSub, assign_sub
}

derive_binop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
    AssignMul, assign_mul
}

derive_binop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Div, div
    DivAssign, div_assign
    DivFrom, div_from
    AssignDiv, assign_div
}

derive_binop! {
    Mat<T: IntoMatSpace>, InnerMat<T>;
    Rem, rem
    RemAssign, rem_assign
    RemFrom, rem_from
    AssignRem, assign_rem
}
