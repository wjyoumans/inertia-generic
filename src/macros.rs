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

macro_rules! derive_unop {
    (
        $ident:ident< $gen:ident: $bound:ident>, $inner:ident<$_:ident>;
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
    ) => {
        impl<T: $bound> $op for $ident<T>
        where
            $inner<T>: $op<Output=$inner<T>>
        {
            type Output = $ident<T>;
            fn $meth(self) -> Self {
                $ident { 
                    inner: self.into_inner().$meth()
                }
            }
        }

        impl<T: $bound> $op_assign for $ident<T> 
        where
            $inner<T>: $op_assign
        {
            #[inline]
            fn $meth_assign(&mut self) {
                self.inner_mut().$meth_assign();
            }
        }
    }
}

// Derive binary ops 1:1 with inner type. Use where clauses since the inertia_algebra
// trait bounds may not require structures to implement all combinations of ops.
//
// Also, for example this allows deriving Div for Poly if an inner polynomial type
// implements Div (perhaps panicking if not exact, or dropping remainder, etc)
// even though its ordinarily not defined
macro_rules! derive_binop {
    (
        $ident:ident< $gen:ident: $bound:ident>, $inner:ident<$_:ident>;
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
        $assign_op:ident, $assign_meth:ident
    ) => {
       
        impl<T: $bound> $op_assign for $ident<T>
        where
            $inner<T>: $op_assign
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: Self) {
                self.inner_mut().$meth_assign(rhs.into_inner());
            }
        }
        
        impl<'a, T: $bound> $op_assign<&'a $ident<T>> for $ident<T>
        where
            $inner<T>: $op_assign<&'a $inner<T>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: &'a $ident<T>) {
                self.inner_mut().$meth_assign(rhs.inner());
            }
        }
        
        impl<T: $bound> $op_from for $ident<T>
        where
            $inner<T>: $op_from
        {
            #[inline]
            fn $meth_from(&mut self, lhs: Self) {
                self.inner_mut().$meth_from(lhs.into_inner());
            }
        }
        
        impl<'a, T: $bound> $op_from<&'a $ident<T>> for $ident<T>
        where
            $inner<T>: $op_from<&'a $inner<T>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: &'a $ident<T>) {
                self.inner_mut().$meth_from(lhs.inner());
            }
        }
        
        impl<T: $bound> $op for $ident<T> 
        where
            $inner<T>: $op<Output=$inner<T>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: Self) -> Self::Output {
                $ident {
                    inner: self.into_inner().$meth(rhs.into_inner())
                }
            }
        }

        impl<'a, T: $bound> $op<&'a $ident<T>> for $ident<T>
        where
            $inner<T>: $op<&'a $inner<T>, Output=$inner<T>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &'a Self) -> Self::Output {
                $ident {
                    inner: self.into_inner().$meth(rhs.inner())
                }
            }
        }
        
        impl<'a, T: $bound> $op<$ident<T>> for &'a $ident<T>
        where
            &'a $inner<T>: $op<$inner<T>, Output=$inner<T>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: $ident<T>) -> Self::Output {
                $ident {
                    inner: self.inner().$meth(rhs.into_inner())
                }
            }
        }
        
        impl<'a, 'b, T: $bound> $op<&'b $ident<T>> for &'a $ident<T>
        where
            &'a $inner<T>: $op<&'b $inner<T>, Output=$inner<T>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &'b $ident<T>) -> Self::Output {
                $ident {
                    inner: self.inner().$meth(rhs.inner())
                }
            }
        }

        /*
        impl<T: $bound> $op_assign<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: OwnedElement<Elem<T>>) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
        
        impl<'a, T: $bound> $op_assign<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: BorrowedElement<'a, Elem<T>>) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
        
        impl<T: $bound> $op_from<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: OwnedElement<Elem<T>>) {
                self.inner_mut().$meth_from(lhs);
            }
        }
        
        impl<'a, T: $bound> $op_from<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: BorrowedElement<'a, Elem<T>>) {
                self.inner_mut().$meth_from(lhs);
            }
        }

        impl<T: $bound> $op<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(mut self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<'a, T: $bound> $op<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(mut self, rhs: BorrowedElement<'a, Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<OwnedElement<Elem<T>>> for &$ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        
        impl<'a, T: $bound> $op<BorrowedElement<'a, Elem<T>>> for &$ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(self, rhs: BorrowedElement<'a, Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }

        impl<T: $bound> $op<$ident<T>> for OwnedElement<Elem<T>>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<'a, T: $bound> $op<$ident<T>> for BorrowedElement<'a, Elem<T>>
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for OwnedElement<Elem<T>>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        
        impl<'a, T: $bound> $op<&$ident<T>> for BorrowedElement<'a, Elem<T>> 
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        */
    }
}

// impl OpAssign, OpFrom => derive rest
macro_rules! forward_binop {
    (
        $ident:ident<$gen:ident : $bound:ident>
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
    ) => {
        impl<T: $bound> $op for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: Self) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }

        impl<T: $bound> $op<&$ident<T>> for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: &Self) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<$ident<T>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }

        impl<T: $bound> $op_assign<$ident<T>> for $ident<T> {
            #[inline]
            fn $meth_assign(&mut self, rhs: $ident<T>) {
                self.$meth_assign(&rhs);
            }
        }
        
        impl<T: $bound> $op_from<$ident<T>> for $ident<T> {
            #[inline]
            fn $meth_from(&mut self, rhs: $ident<T>) {
                self.$meth_from(&rhs);
            }
        }

        /*
        impl<T: $bound> $op<$ident<T>> for OwnedElement<Elem<T>> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<$ident<T>> for BorrowedElement<'_, Elem<T>> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for OwnedElement<Elem<T>> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for BorrowedElement<'_, Elem<T>> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }


        impl<T: $bound> $op<OwnedElement<Elem<T>>> for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<BorrowedElement<'_, Elem<T>>> for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: BorrowedElement<Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<OwnedElement<Elem<T>>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        
        impl<T: $bound> $op<BorrowedElement<'_, Elem<T>>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: BorrowedElement<Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
    */
    }
}

/* OLD, DELETE
// derive binops for wrapper type assuming wrapped type implements 
// all combinations of ops. I.e. derive Add for Poly using Add impl for
// GenericPoly.
//
// Want to just derive Op<A> for B wherever Inner<B>: Op<Inner<A>> but
// was getting recursion in checking predicates, so just derive all from 
macro_rules! derive_binop {
    (
        $ident:ident< $gen:ident: $bound:ident>, $inner:ident<$_:ident>;
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
        $assign_op:ident, $assign_meth:ident
    ) => {
       
        /*
        impl<T: $bound> $op_assign for $ident<T>
        //where
        //    $inner<T>: $op_assign
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: Self) {
                self.inner_mut().$meth_assign(rhs.into_inner());
            }
        }
        */
        
        impl<'a, T: $bound> $op_assign<&'a $ident<T>> for $ident<T>
        //where
        //    $inner<T>: $op_assign<&'a $inner<T>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: &'a $ident<T>) {
                self.inner_mut().$meth_assign(rhs.inner());
            }
        }
        
        /*
        impl<T: $bound> $op_from for $ident<T>
        //where
        //    $inner<T>: $op_from
        {
            #[inline]
            fn $meth_from(&mut self, lhs: Self) {
                self.inner_mut().$meth_from(lhs.into_inner());
            }
        }
        */
        
        impl<'a, T: $bound> $op_from<&'a $ident<T>> for $ident<T>
        //where
        //    $inner<T>: $op_from<&'a $inner<T>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: &'a $ident<T>) {
                self.inner_mut().$meth_from(lhs.inner());
            }
        }
        
        impl<T: $bound> $op for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: Self) -> Self::Output {
                self.$meth_assign(&rhs);
                self
            }
        }

        /*
        impl<T: $bound> $op<&$ident<T>> for $ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(mut self, rhs: &Self) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<$ident<T>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for &$ident<T> {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        */

        /*
        impl<T: $bound> $op_assign<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: OwnedElement<Elem<T>>) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
        
        impl<'a, T: $bound> $op_assign<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: BorrowedElement<'a, Elem<T>>) {
                self.inner_mut().$meth_assign(rhs);
            }
        }
        
        impl<T: $bound> $op_from<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: OwnedElement<Elem<T>>) {
                self.inner_mut().$meth_from(lhs);
            }
        }
        
        impl<'a, T: $bound> $op_from<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>,
        {
            #[inline]
            fn $meth_from(&mut self, lhs: BorrowedElement<'a, Elem<T>>) {
                self.inner_mut().$meth_from(lhs);
            }
        }

        impl<T: $bound> $op<OwnedElement<Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(mut self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<'a, T: $bound> $op<BorrowedElement<'a, Elem<T>>> for $ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(mut self, rhs: BorrowedElement<'a, Elem<T>>) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: $bound> $op<OwnedElement<Elem<T>>> for &$ident<T>
        where
            $inner<T>: $op_assign<OwnedElement<Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(self, rhs: OwnedElement<Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        
        impl<'a, T: $bound> $op<BorrowedElement<'a, Elem<T>>> for &$ident<T>
        where
            $inner<T>: $op_assign<BorrowedElement<'a, Elem<T>>>,
        {
            type Output = $ident<T>;
            fn $meth(self, rhs: BorrowedElement<'a, Elem<T>>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }

        impl<T: $bound> $op<$ident<T>> for OwnedElement<Elem<T>>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<'a, T: $bound> $op<$ident<T>> for BorrowedElement<'a, Elem<T>>
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, mut rhs: $ident<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: $bound> $op<&$ident<T>> for OwnedElement<Elem<T>>
        where
            $inner<T>: $op_from<OwnedElement<Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        
        impl<'a, T: $bound> $op<&$ident<T>> for BorrowedElement<'a, Elem<T>> 
        where
            $inner<T>: $op_from<BorrowedElement<'a, Elem<T>>>
        {
            type Output = $ident<T>;
            #[inline]
            fn $meth(self, rhs: &$ident<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        */
    }
}
*/
