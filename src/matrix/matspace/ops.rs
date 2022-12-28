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

use crate::ops::*;
use crate::poly::*;

// TODO: 
// - Evaluation and exponentiation.
// - AssignOp impls for ring elements: impl `AssignOp<T, T>`, `AssignOp<T, Poly<T>>` 
//   and manymore combinations for `Poly<T>`.
// - Maybe make more generic if possible, like `impl Op<Poly<S>> for Poly<T>` etc.


macro_rules! derive_ops {
    (
        $flag:ident
        $(
            $op:ident, $meth:ident;
        )*
    ) => (
        derive_ops! {
            $flag
            $($op, $op, $meth;)*
        }
    );
    (
        op
        $(
            $op:ident, $bound:ident, $meth:ident;
        )*
    ) => ($(
        
        impl<'a, T: RingElement> $op for Poly<T>
        where
            InnerPoly<T>: $bound<Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            #[inline]
            fn $meth(self, rhs: Self) -> Self::Output {
                Poly {
                    inner: self.into_inner().$meth(rhs.into_inner()),
                }
            }
        }
       
        impl<'a, T: RingElement> $op for &'a Poly<T>
        where
            &'a InnerPoly<T>: $bound<Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            #[inline]
            fn $meth(self, rhs: Self) -> Self::Output {
                Poly {
                    inner: self.inner().$meth(rhs.inner()),
                }
            }
        }

        impl<'a, T: RingElement> $op<&'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            #[inline]
            fn $meth(self, rhs: &'a Poly<T>) -> Self::Output {
                Poly {
                    inner: self.into_inner().$meth(rhs.inner()),
                }
            }
        }

        impl<'a, T: RingElement> $op<Poly<T>> for &'a Poly<T>
        where
            &'a InnerPoly<T>: $bound<InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            #[inline]
            fn $meth(self, rhs: Poly<T>) -> Self::Output {
                Poly {
                    inner: self.inner().$meth(rhs.into_inner()),
                }
            }
        }
    )*);
    (
        op_assign
        $(
            $op_assign:ident, $bound:ident, $meth_assign:ident;
        )*
    ) => ($(
        impl<T: RingElement> $op_assign for Poly<T>
        where
            InnerPoly<T>: $bound,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: Self) {
                self.inner_mut().$meth_assign(rhs.into_inner());
            }
        }
        
        impl<'a, T: RingElement> $op_assign<&'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a InnerPoly<T>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: &'a Poly<T>) {
                self.inner_mut().$meth_assign(rhs.inner());
            }
        }
    )*);
    (
        from
        $(
            $op_from:ident, $bound:ident, $meth_from:ident;
        )*
    ) => ($(   
        impl<T: RingElement> $op_from for Poly<T>
        where
            InnerPoly<T>: $bound,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: Self) {
                self.inner_mut().$meth_from(lhs.into_inner());
            }
        }
        
        impl<'a, T: RingElement> $op_from<&'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a InnerPoly<T>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: &'a Poly<T>) {
                self.inner_mut().$meth_from(lhs.inner());
            }
        }
    )*);
    (
        assign_op
        $(
            $assign_op:ident, $bound:ident, $assign_meth:ident;
        )*
    ) => ($(   
        impl<T: RingElement> $assign_op for Poly<T>
        where
            InnerPoly<T>: $bound,
        {
            #[inline]
            fn $assign_meth(&mut self, lhs: Self, rhs: Self) {
                self.inner_mut().$assign_meth(lhs.into_inner(), rhs.into_inner());
            }
        }
        
        impl<'a, T: RingElement> $assign_op<&'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a InnerPoly<T>>,
        {
            #[inline]
            fn $assign_meth(&mut self, lhs: &'a Poly<T>, rhs: Self) {
                self.inner_mut().$assign_meth(lhs.inner(), rhs.into_inner());
            }
        }
        
        impl<'a, T: RingElement> $assign_op<Poly<T>, &'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<InnerPoly<T>, &'a InnerPoly<T>>,
        {
            #[inline]
            fn $assign_meth(&mut self, lhs: Self, rhs: &'a Poly<T>) {
                self.inner_mut().$assign_meth(lhs.into_inner(), rhs.inner());
            }
        }
        
        impl<'a, T: RingElement> $assign_op<&'a Poly<T>, &'a Poly<T>> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a InnerPoly<T>, &'a InnerPoly<T>>,
        {
            #[inline]
            fn $assign_meth(&mut self, lhs: &'a Poly<T>, rhs: &'a Poly<T>) {
                self.inner_mut().$assign_meth(lhs.inner(), rhs.inner());
            }
        }
    )*)
}

/*
impl<A: RingElement, B: RingElement> Add<B> for Poly<A>
where
    InnerPoly<A>: Add<B, Output = InnerPoly<A>>
{
    type Output = Poly<A>;
    fn add(self, rhs: B) -> Self::Output {
        Poly { inner: self.into_inner().add(rhs) }
    }
}

impl<A: RingElement, B: RingElement> Add<Poly<B>> for A
where
    A: Add<InnerPoly<B>, Output = InnerPoly<B>>
{
    type Output = Poly<B>;
    fn add(self, rhs: Poly<B>) -> Self::Output {
        Poly { inner: self.add(rhs.into_inner()) }
    }
}*/

// Impl inertia_algebra::ops for Poly<T> wherever InnerPoly<T> impls them.

derive_ops! {
    op
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
    Rem, rem;
}

derive_ops! {
    op_assign
    AddAssign, add_assign;
    SubAssign, sub_assign;
    MulAssign, mul_assign;
    DivAssign, div_assign;
    RemAssign, rem_assign;
}

derive_ops! {
    from
    AddFrom, add_from;
    SubFrom, sub_from;
    MulFrom, mul_from;
    DivFrom, div_from;
    RemFrom, rem_from;
}

derive_ops! {
    assign_op
    AssignAdd, assign_add;
    AssignSub, assign_sub;
    AssignMul, assign_mul;
    AssignDiv, assign_div;
    AssignRem, assign_rem;
}

macro_rules! derive_ring_ops {
    (
        // polynomial + ring element
        $flag:ident
        $(
            $op:ident, $meth:ident;
        )*
    ) => (
        derive_ring_ops! {
            $flag
            $($op, $op, $meth;)*
        }
    );
    (
        // polynomial + ring element
        right_op
        $(
            $op:ident, $bound:ident, $meth:ident;
        )*
    ) => ($(
        impl<T: RingElement> $op<T> for Poly<T>
        where
            InnerPoly<T>: $bound<T, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: T) -> Self::Output {
                Poly {
                    inner: self.into_inner().$meth(rhs)
                    //inner: self.$meth(rhs.into_inner()),
                }
            }
        }
        
        impl<'a, T: RingElement> $op<&'a T> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a T, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: &'a T) -> Self::Output {
                Poly {
                    inner: self.into_inner().$meth(rhs)
                }
            }
        }
        
        impl<'a, T: RingElement> $op<T> for &'a Poly<T>
        where
            &'a InnerPoly<T>: $bound<T, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: T) -> Self::Output {
                Poly {
                    inner: self.inner().$meth(rhs)
                }
            }
        }
        
        impl<'a, T: RingElement> $op<&'a T> for &'a Poly<T>
        where
            &'a InnerPoly<T>: $bound<&'a T, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: &'a T) -> Self::Output {
                Poly {
                    inner: self.inner().$meth(rhs)
                }
            }
        }
    )*);
    (
        // ring element + polynomial
        left_op
        $(
            $op:ident, $bound:ident, $meth:ident;
        )*
    ) => ($(

        impl<T: RingElement> $op<Poly<T>> for T
        where
            T: $bound<InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: Poly<T>) -> Self::Output {
                Poly {
                    inner: self.$meth(rhs.into_inner()),
                }
            }
        }
        
        impl<'a, T: RingElement> $op<&'a Poly<T>> for T
        where
            T: $bound<&'a InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: &'a Poly<T>) -> Self::Output {
                Poly {
                    inner: self.$meth(rhs.inner()),
                }
            }
        }
        
        impl<'a, T: RingElement> $op<Poly<T>> for &'a T
        where
            &'a T: $bound<InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: Poly<T>) -> Self::Output {
                Poly {
                    inner: self.$meth(rhs.into_inner()),
                }
            }
        }
        
        impl<'a, T: RingElement> $op<&'a Poly<T>> for &'a T
        where
            &'a T: $bound<&'a InnerPoly<T>, Output = InnerPoly<T>>,
        {
            type Output = Poly<T>;
            fn $meth(self, rhs: &'a Poly<T>) -> Self::Output {
                Poly {
                    inner: self.$meth(rhs.inner()),
                }
            }
        }
    )*);
    (
        op_assign
        $(
            $op_assign:ident, $bound:ident, $meth_assign:ident;
        )*
    ) => ($(
        impl<T: RingElement> $op_assign<T> for Poly<T>
        where
            InnerPoly<T>: $bound<T>,
        {
            fn $meth_assign(&mut self, rhs: T) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
        
        impl<'a, T: RingElement> $op_assign<&'a T> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a T>,
        {
            fn $meth_assign(&mut self, rhs: &'a T) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
    )*);
    (
        from
        $(
            $op_from:ident, $bound:ident, $meth_from:ident;
        )*
    ) => ($(   
        impl<T: RingElement> $op_from<T> for Poly<T>
        where
            InnerPoly<T>: $bound<T>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: T) {
                self.inner_mut().$meth_from(lhs);
            }
        }
        
        impl<'a, T: RingElement> $op_from<&'a T> for Poly<T>
        where
            InnerPoly<T>: $bound<&'a T>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: &'a T) {
                self.inner_mut().$meth_from(lhs);
            }
        }
    )*);

    // assign_op has a lot of combinations and probably won't see much use so 
    // don't bother for now.
}

derive_ring_ops! {
    right_op
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
    Rem, rem;
}

derive_ring_ops! {
    left_op
    Add, add;
    Sub, sub;
    Mul, mul;
    Div, div;
    Rem, rem;
}

derive_ring_ops! {
    op_assign
    AddAssign, add_assign;
    SubAssign, sub_assign;
    MulAssign, mul_assign;
    DivAssign, div_assign;
    RemAssign, rem_assign;
}

derive_ring_ops! {
    from
    AddFrom, add_from;
    SubFrom, sub_from;
    MulFrom, mul_from;
    DivFrom, div_from;
    RemFrom, rem_from;
}


/// Impls of [std::ops] wrapping [inertia_algebra::ops] when possible.
mod std_ops {
    use super::*;
    use std::ops::Add as AddStd;
    use std::ops::Sub as SubStd;
    use std::ops::Mul as MulStd;
    use std::ops::Div as DivStd;
    use std::ops::Rem as RemStd;

    use std::ops::SubAssign as SubAssignStd;
    use std::ops::AddAssign as AddAssignStd;
    use std::ops::MulAssign as MulAssignStd;
    use std::ops::DivAssign as DivAssignStd;
    use std::ops::RemAssign as RemAssignStd;

    derive_ops! {
        op
        AddStd, Add, add;
        SubStd, Sub, sub;
        MulStd, Mul, mul;
        DivStd, Div, div;
        RemStd, Rem, rem;
    }

    derive_ops! {
        op_assign
        AddAssignStd, AddAssign, add_assign;
        SubAssignStd, SubAssign, sub_assign;
        MulAssignStd, MulAssign, mul_assign;
        DivAssignStd, DivAssign, div_assign;
        RemAssignStd, RemAssign, rem_assign;
    }

    derive_ring_ops! {
        right_op
        AddStd, Add, add;
        SubStd, Sub, sub;
        MulStd, Mul, mul;
        DivStd, Div, div;
        RemStd, Rem, rem;
    }

    derive_ring_ops! {
        op_assign
        AddAssignStd, AddAssign, add_assign;
        SubAssignStd, SubAssign, sub_assign;
        MulAssignStd, MulAssign, mul_assign;
        DivAssignStd, DivAssign, div_assign;
        RemAssignStd, RemAssign, rem_assign;
    }
}
