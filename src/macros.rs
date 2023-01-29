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
