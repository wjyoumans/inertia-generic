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


use inertia_traits::*;
//use crate::ops::*;
//use crate::traits::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

///////////////////////////////////////////////////////////////////
// GenericPolyRing<T>
///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct GenericMatCtx<T: RingElement> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "BaseRing<T>: Serialize",
            deserialize = "BaseRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) base_ring: BaseRing<T>,
    pub(crate) nrows: u32,
    pub(crate) ncols: u32
}

impl<T: RingElement> GenericMatCtx<T> {
    pub fn new(base_ring: BaseRing<T>, nrows: u32, ncols: u32) -> Self {
        GenericMatCtx { base_ring, nrows, ncols }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericMatSpace<T: RingElement> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "BaseRing<T>: Serialize",
            deserialize = "BaseRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericMatCtx<T>>,
}

impl<T: RingElement> Eq for GenericMatSpace<T> where BaseRing<T>: PartialEq {}

impl<T: RingElement> PartialEq for GenericMatSpace<T> 
where
    BaseRing<T>: PartialEq
{
    #[inline]
    fn eq(&self, rhs: &GenericMatSpace<T>) -> bool {
        Rc::ptr_eq(&self.ctx, &rhs.ctx) ||
            (self.base_ring() == rhs.base_ring() && self.nrows() == rhs.nrows() &&
             self.ncols() == rhs.ncols())
    }
}

impl<T: RingElement> fmt::Display for GenericMatSpace<T> 
where
    BaseRing<T>: fmt::Display
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Space of {} by {} matrices over {}",
            self.nrows(),
            self.ncols(),
            self.base_ring()
        )
    }
}

impl<T: RingElement> Hash for GenericMatSpace<T> 
where
    BaseRing<T>: Hash
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl<T: RingElement> Parent for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    #[inline]
    fn default(&self) -> GenericMat<T> {
        let dim = self.nrows() * self.ncols();
        let mut entries = Vec::with_capacity(dim);

        let zero = self.base_ring().zero();
        entries.resize(dim, zero);
        GenericMat {
            entries,
            ctx: Rc::clone(&self.ctx)
        }
    }
}

impl<T: RingElement> MatrixSpace<T> for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    #[inline]
    fn new(base_ring: &BaseRing<T>, nrows: u32, ncols: u32) -> Self {
        GenericMatSpace {
            ctx: Rc::new(GenericMatCtx::new(base_ring.clone(), nrows, ncols))
        }
    }

    #[inline]
    fn base_ring(&self) -> &BaseRing<T> {
        &self.ctx.base_ring
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.ctx.nrows as usize
    }
    
    #[inline]
    fn ncols(&self) -> usize {
        self.ctx.ncols as usize
    }
    
    #[inline]
    fn is_generic(&self) -> bool { true }
}

///////////////////////////////////////////////////////////////////
// GenericMat<T>
///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericMat<T: RingElement> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "BaseRing<T>: Serialize",
            deserialize = "BaseRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericMatCtx<T>>,
    pub(crate) entries: Vec<T>,
}

impl<T: RingElement> Eq for GenericMat<T> {}

impl<S: RingElement, T: RingElement> PartialEq<GenericMat<S>> for GenericMat<T>
where
    T: PartialEq<S>,
{
    fn eq(&self, rhs: &GenericMat<S>) -> bool {
        let nrows = self.nrows();
        if rhs.nrows() != nrows {
            return false;
        }
        let ncols = self.ncols();
        if rhs.ncols() != ncols {
            return false;
        }
        let len = nrows * ncols - 1;

        let e1 = &self.entries;
        let e2 = &rhs.entries;
        for i in 0..len {
            if e1[i] != e2[i] {
                return false;
            }
        }
        true
    }
}

impl<S: RingElement, T: RingElement> PartialEq<&GenericMat<S>> for GenericMat<T>
where
    T: PartialEq<S>,
{
    #[inline]
    fn eq(&self, rhs: &&GenericMat<S>) -> bool {
        (&self).eq(rhs)
    }
}

impl<S: RingElement, T: RingElement> PartialEq<GenericMat<S>> for &GenericMat<T>
where
    T: PartialEq<S>,
{
    #[inline]
    fn eq(&self, rhs: &GenericMat<S>) -> bool {
        self.eq(&rhs)
    }
}

impl<T: RingElement + fmt::Display> fmt::Display for GenericMat<T>
where
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let entries = self.entries();
        let m = self.nrows();
        let n = self.ncols();

        let mut out = Vec::with_capacity(m*n);

        for i in 0..m {
            let mut row = Vec::with_capacity(n + 2);
            row.push("[".to_string());
            for j in 0..n {
                row.push(format!(" {} ", entries[i*n+j]));
            }
            if i == m - 1 {
                row.push("]".to_string());
            } else {
                row.push("]\n".to_string());
            }
            out.push(row.join(""));
        }
        write!(f, "{}", out.join(""))
    }
}

impl<T: RingElement + Hash> Hash for GenericMat<T>
where
    BaseRing<T>: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.entries().hash(state);
    }
}

impl<T: RingElement> Element for GenericMat<T> {
    type Parent = GenericMatSpace<T>;

    #[inline]
    fn parent(&self) -> GenericMatSpace<T> {
        GenericMatSpace {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

impl<T: RingElement> MatrixSpaceElement<T> for GenericMat<T> {
    type Parent = GenericMatSpace<T>;

    #[inline]
    fn base_ring(&self) -> &BaseRing<T> {
        &self.ctx.base_ring
    }

    #[inline]
    fn len(&self) -> usize {
        self.entries.len()
    }

    #[inline]
    fn nrows(&self) -> usize {
        self.ctx.nrows as usize
    }
    
    #[inline]
    fn ncols(&self) -> usize {
        self.ctx.ncols as usize
    }

    #[inline]
    fn entry(&self, i: usize, j: usize) -> Option<&T> {
        self.entries.get(i*self.ncols() + j)
    }

    #[inline]
    fn entry_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
        let n = self.ncols();
        self.entries.get_mut(i*n + j)
    }
    
    #[inline]
    fn into_entries(self) -> Vec<T> {
        self.entries
    }

    #[inline]
    fn entries(&self) -> &Vec<T> {
        &self.entries
    }

    #[inline]
    fn entries_mut(&mut self) -> &mut Vec<T> {
        &mut self.entries
    }

    fn set_entry(&mut self, i: usize, j: usize, entry: T) -> Option<T> {
        let idx = i*self.ncols() + j;
        if idx >= self.len() {
            None
        } else {
            self.entries.push(entry);
            Some(self.entries.swap_remove(idx))
        }
    }

    #[inline]
    fn is_generic(&self) -> bool { true }
}

/*
impl<T: RingElement> GenericMat<T>
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
    fn resize(&mut self, new_len: usize) {
        self.coeffs.resize(new_len, self.base_ring().zero());
    }

    #[allow(dead_code)]
    #[inline]
    fn rotate_right(&mut self, k: usize) {
        self.coeffs.rotate_right(k);
    }

    #[allow(dead_code)]
    #[inline]
    fn rotate_left(&mut self, k: usize) {
        self.coeffs.rotate_left(k);
    }


    /*
    // TODO: use addmul/muladd? Better choice of val/ref for inputs? assign?
    pub fn mul_classical<'a>(&'a self, rhs: &'a GenericMat<T>) -> Self
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
    */
   
    /*
    pub fn mul_assign_classical<'a>(&mut self, rhs: &'a GenericMat<T>)
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
    }*/

    //pub fn mul_karatsuba(&self, rhs: &GenericMat<T>) -> Self {
    //}
}
*/
///////////////////////////////////////////////////////////////////
// Coercion
///////////////////////////////////////////////////////////////////

/*
impl<T: RingElement> Coerce<GenericMat<T>> for GenericMatSpace<T> {
    #[inline]
    fn coerce(&self, arg: GenericMat<T>) -> GenericMat<T> {
        arg
    }
}

impl<T: RingElement> Coerce<&GenericMat<T>> for GenericMatSpace<T> {
    #[inline]
    fn coerce(&self, arg: &GenericMat<T>) -> GenericMat<T> {
        arg.clone()
    }
}

impl<T: RingElement> Coerce<&T> for GenericMatSpace<T> {
    fn coerce(&self, coeff: &T) -> GenericMat<T> {
        let mut res = self.default();
        res.set_coeff(0, coeff.clone());
        res
    }
}

impl<A, T: RingElement, const CAP: usize> Coerce<[A; CAP]> for GenericMatSpace<T>
where
    BaseRing<T>: Coerce<A>,
{
    fn coerce(&self, coeffs: [A; CAP]) -> GenericMat<T> {
        let mut res = self.default();
        for (i, x) in coeffs.into_iter().enumerate() {
            res.set_coeff(i, self.base_ring().coerce(x));
        }
        res
    }
}

impl<'a, A, T: RingElement> Coerce<&'a [A]> for GenericMatSpace<T>
where
    BaseRing<T>: Coerce<&'a A>,
{
    fn coerce(&self, coeffs: &'a [A]) -> GenericMat<T> {
        let mut res = self.default();
        for (i, x) in coeffs.iter().enumerate() {
            res.set_coeff(i, self.base_ring().coerce(x));
        }
        res
    }
}

impl<T: RingElement> Coerce<Vec<T>> for GenericMatSpace<T> {
    fn coerce(&self, coeffs: Vec<T>) -> GenericMat<T> {
        GenericMat {
            parent: Rc::clone(&self.inner),
            coeffs,
        }
    }
}
*/

///////////////////////////////////////////////////////////////////
// Operations
///////////////////////////////////////////////////////////////////

/*
macro_rules! forward_binop {
    (
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
    ) => {
        forward_binop! {
            $op, $meth
            $op_assign, $meth_assign
        }
    };
    (
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
    ) => {
        impl<T: RingElement> $op for GenericMat<T>
        where 
            GenericMat<T>: $op_assign 
        {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(mut self, rhs: Self) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }

        impl<T: RingElement> $op<&GenericMat<T>> for GenericMat<T> 
        where 
            GenericMat<T>: $op_assign 
        {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(mut self, rhs: &Self) -> Self::Output {
                self.$meth_assign(rhs.clone());
                self
            }
        }
        
        impl<T: RingElement> $op<GenericMat<T>> for &GenericMat<T> 
        where 
            GenericMat<T>: $op_assign
        {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: GenericMat<T>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        
        impl<T: RingElement> $op<&GenericMat<T>> for &GenericMat<T> 
        where 
            GenericMat<T>: $op_assign
        {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: &GenericMat<T>) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs.clone());
                res
            }
        }

        impl<T: RingElement> $op_assign<&GenericMat<T>> for GenericMat<T>
        where
            GenericMat<T>: $op_assign
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: &GenericMat<T>) {
                self.$meth_assign(rhs.clone());
            }
        }
    }
}

// Derive all other ops from above
macro_rules! forward_ringelem_binop {
    (
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
        $op_from:ident, $meth_from:ident
    ) => {
        forward_ringelem_binop! {
            $op, $meth
            $op_assign, $meth_assign
        }
        
        impl<T: RingElement> $op<GenericMat<T>> for T where T: $op_from {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, mut rhs: GenericMat<T>) -> Self::Output {
                rhs.$meth_from(self);
                rhs
            }
        }
        
        impl<T: RingElement> $op<&GenericMat<T>> for T where T: $op_from {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: &GenericMat<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self);
                res
            }
        }
        
        impl<T: RingElement> $op<GenericMat<T>> for &T where T: $op_from {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, mut rhs: GenericMat<T>) -> Self::Output {
                rhs.$meth_from(self.clone());
                rhs
            }
        }
        
        impl<T: RingElement> $op<&GenericMat<T>> for &T where T: $op_from {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: &GenericMat<T>) -> Self::Output {
                let mut res = rhs.clone();
                res.$meth_from(self.clone());
                res
            }
        }

        impl<T: RingElement> $op_from<&T> for GenericMat<T> where T: $op_from {
            fn $meth_from(&mut self, rhs: &T) {
                self.$meth_from(rhs.clone());
            }
        }
    };
    (
        $op:ident, $meth:ident
        $op_assign:ident, $meth_assign:ident
    ) => {
        impl<T: RingElement> $op<T> for GenericMat<T> where T: $op_assign {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(mut self, rhs: T) -> Self::Output {
                self.$meth_assign(rhs);
                self
            }
        }
        
        impl<T: RingElement> $op<&T> for GenericMat<T> where T: $op_assign {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(mut self, rhs: &T) -> Self::Output {
                self.$meth_assign(rhs.clone());
                self
            }
        }

        impl<T: RingElement> $op<T> for &GenericMat<T> where T: $op_assign {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: T) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs);
                res
            }
        }
        
        impl<T: RingElement> $op<&T> for &GenericMat<T> where T: $op_assign {
            type Output = GenericMat<T>;
            #[inline]
            fn $meth(self, rhs: &T) -> Self::Output {
                let mut res = self.clone();
                res.$meth_assign(rhs.clone());
                res
            }
        }
        
        impl<T: RingElement> $op_assign<&T> for GenericMat<T> where T: $op_assign {
            fn $meth_assign(&mut self, rhs: &T) {
                self.$meth_assign(rhs.clone());
            }
        }
        
    }
}

/*
macro_rules! derive_std_binop {
    (
        $(
            $op:ident, $meth:ident
            $op_assign:ident, $meth_assign:ident
        )*
    ) => ($(
        
        impl<T: RingElement> std::ops::$op for GenericMat<T>
        where
            GenericMat<T>: $op,
        {
            type Output = <GenericMat<T> as $op>::Output;
            #[inline]
            fn $meth(self, rhs: Self) -> Self::Output {
                $op::$meth(self, rhs)
            }
        }
        
        impl<'a, T: RingElement> std::ops::$op for &'a GenericMat<T>
        where
            &'a GenericMat<T>: $op,
        {
            type Output = <&'a GenericMat<T> as $op>::Output;
            #[inline]
            fn $meth(self, rhs: Self) -> Self::Output {
                $op::$meth(self, rhs)
            }
        }
       
        impl<'a, T: RingElement> std::ops::$op<&'a GenericMat<T>> for GenericMat<T>
        where
            GenericMat<T>: $op<&'a GenericMat<T>>,
        {
            type Output = <GenericMat<T> as $op<&'a GenericMat<T>>>::Output;
            #[inline]
            fn $meth(self, rhs: &'a Self) -> Self::Output {
                $op::$meth(self, rhs)
            }
        }
        
        impl<'a, T: RingElement> std::ops::$op<GenericMat<T>> for &'a GenericMat<T>
        where
            &'a GenericMat<T>: $op<GenericMat<T>>,
        {
            type Output = <&'a GenericMat<T> as $op<GenericMat<T>>>::Output;
            #[inline]
            fn $meth(self, rhs: GenericMat<T>) -> Self::Output {
                $op::$meth(self, rhs)
            }
        }

        impl<T> std::ops::$op_assign for GenericMat<T>
        where
            T: RingElement,
            GenericMat<T>: $op_assign,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: Self) {
                $op_assign::$meth_assign(self, rhs);
            }
        } 

        impl<'a, T> std::ops::$op_assign<&'a GenericMat<T>> for GenericMat<T>
        where
            T: RingElement,
            GenericMat<T>: $op_assign<&'a GenericMat<T>>,
        {
            #[inline]
            fn $meth_assign(&mut self, rhs: &'a Self) {
                $op_assign::$meth_assign(self, rhs);
            }
        }
    )*);
}*/

impl<T: RingElement> AddAssign for GenericMat<T>
where
    T: AddAssign,
{
    fn add_assign(&mut self, rhs: Self) {
        let max = std::cmp::max(self.len(), rhs.len());
        self.resize(max);

        for (i, c) in rhs.into_coefficients().into_iter().enumerate() {
            self.coeff_mut(i).add_assign(c);
        }
    }
}

impl<T: RingElement> AddAssign<T> for GenericMat<T> where T: AddAssign {
    #[inline]
    fn add_assign(&mut self, rhs: T) {
        self.coeff_mut(0).add_assign(rhs);
    }
}

impl<T: RingElement> AddFrom<T> for GenericMat<T> where T: AddFrom {
    #[inline]
    fn add_from(&mut self, rhs: T) {
        self.coeff_mut(0).add_from(rhs);
    }
}

forward_binop! {
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
}

forward_ringelem_binop! {
    Add, add
    AddAssign, add_assign
    AddFrom, add_from
}

/*
derive_std_binop! {
    Add, add
    AddAssign, add_assign
}*/

impl<T: RingElement> SubAssign for GenericMat<T>
where
    T: SubAssign,
{
    fn sub_assign(&mut self, rhs: Self) {
        let max = std::cmp::max(self.len(), rhs.len());
        self.resize(max);

        for (i, c) in rhs.into_coefficients().into_iter().enumerate() {
            self.coeff_mut(i).sub_assign(c);
        }
    }
}

impl<T: RingElement> SubAssign<T> for GenericMat<T> where T: SubAssign {
    #[inline]
    fn sub_assign(&mut self, rhs: T) {
        self.coeff_mut(0).sub_assign(rhs);
    }
}

impl<T: RingElement> SubFrom<T> for GenericMat<T> where T: SubFrom {
    #[inline]
    fn sub_from(&mut self, rhs: T) {
        self.coeff_mut(0).sub_from(rhs);
    }
}

forward_binop! {
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
}

forward_ringelem_binop! {
    Sub, sub
    SubAssign, sub_assign
    SubFrom, sub_from
}

/*
derive_std_binop! {
    Sub, sub
    SubAssign, sub_assign
}
*/

impl<T: RingElement> Mul for GenericMat<T>
where
{
    type Output = GenericMat<T>;
    fn mul(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<T: RingElement> MulAssign<T> for GenericMat<T> where T: MulAssign {
    #[inline]
    fn mul_assign(&mut self, rhs: T) {
        self.coefficients_mut().into_iter()
            .for_each(|x| x.mul_assign(rhs.clone()));
    }
}

impl<T: RingElement> MulFrom<T> for GenericMat<T> where T: MulFrom {
    #[inline]
    fn mul_from(&mut self, rhs: T) {
        self.coefficients_mut().into_iter()
            .for_each(|x| x.mul_from(rhs.clone()));
    }
}

/*
forward_binop! {
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
}
*/

forward_ringelem_binop! {
    Mul, mul
    MulAssign, mul_assign
    MulFrom, mul_from
}

/*
derive_std_binop! {
    Mul, mul
    MulAssign, mul_assign
}*/

impl<T: RingElement> Div for GenericMat<T>
where
{
    type Output = GenericMat<T>;
    fn div(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<T: RingElement> DivAssign<T> for GenericMat<T> where T: DivAssign {
    #[inline]
    fn div_assign(&mut self, rhs: T) {
        self.coefficients_mut().into_iter()
            .for_each(|x| x.div_assign(rhs.clone()));
    }
}

/*
forward_binop! {
    Div, div
    DivAssign, div_assign
    DivFrom, div_from
}
*/

forward_ringelem_binop! {
    Div, div
    DivAssign, div_assign
}

/*
derive_std_binop! {
    Div, div
    DivAssign, div_assign
}*/

impl<T: RingElement> Rem for GenericMat<T>
where
{
    type Output = GenericMat<T>;
    fn rem(self, _rhs: Self) -> Self::Output {
        unimplemented!()
    }
}

impl<T: RingElement> RemAssign<T> for GenericMat<T> where T: RemAssign {
    #[inline]
    fn rem_assign(&mut self, rhs: T) {
        self.coefficients_mut().into_iter()
            .for_each(|x| x.rem_assign(rhs.clone()));
    }
}

/*
forward_binop! {
    Rem, rem
    RemAssign, rem_assign
    RemFrom, rem_from
}
*/

forward_ringelem_binop! {
    Rem, rem
    RemAssign, rem_assign
}

/*
derive_std_binop! {
    Rem, rem
    RemAssign, rem_assign
}*/
*/
