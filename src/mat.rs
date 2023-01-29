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


use inertia_algebra::*;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::rc::Rc;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

mod ops;

///////////////////////////////////////////////////////////////////
// GenericMatSpace<T>
///////////////////////////////////////////////////////////////////

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub(crate) struct GenericMatCtx<T> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "BaseRing<T>: Serialize",
            deserialize = "BaseRing<T>: Deserialize<'de>",
        ))
    )]
    pub(crate) base_ring: T,
    pub(crate) nrows: u64,
    pub(crate) ncols: u64
}

impl<T> GenericMatCtx<T> {
    pub fn new(base_ring: T, nrows: u64, ncols: u64) -> Self {
        GenericMatCtx { base_ring, nrows, ncols }
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericMatSpace<T> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericMatCtx<T>>,
}

impl<T: Ring + fmt::Display> fmt::Display for GenericMatSpace<T> {
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

impl<T: Ring + PartialEq> Eq for GenericMatSpace<T> {}

impl<T: Ring + PartialEq> PartialEq for GenericMatSpace<T> {
    #[inline]
    fn eq(&self, rhs: &GenericMatSpace<T>) -> bool {
        Rc::ptr_eq(&self.ctx, &rhs.ctx) || 
            (self.base_ring() == rhs.base_ring() 
             && self.nrows() == rhs.nrows() 
             && self.ncols() == rhs.ncols())
    }
}

impl<T: Ring + Hash> Hash for GenericMatSpace<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.base_ring().hash(state);
        self.nrows().hash(state);
        self.ncols().hash(state);
    }
}

impl<T: Ring> Parent for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    /*
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
    */
}

impl<T: Ring> Identity<Additive> for GenericMatSpace<T> {
    #[inline]
    fn identity(&self) -> GenericMat<T> {
        unimplemented!()
        /* TODO
        GenericMat {
            entries: vec![self.base_ring().zero()],
            ctx: Rc::clone(&self.ctx),
        }*/
    }
}

impl<T: Ring> Divisible<Additive> for GenericMatSpace<T> {}

impl<T: Ring> Associative<Additive> for GenericMatSpace<T> {}

impl<T: Ring> Commutative<Additive> for GenericMatSpace<T> {}

impl<T: Ring> MatrixSpace<T> for GenericMatSpace<T> {
    type Element = GenericMat<T>;

    #[inline]
    fn init<D: Into<u64>>(base_ring: &T, nrows: D, ncols: D) -> Self {
        GenericMatSpace {
            ctx: Rc::new(
                GenericMatCtx::new(
                     base_ring.clone(), 
                     nrows.into(), 
                     ncols.into()
                )
            )
        }
    }

    #[inline]
    fn base_ring(&self) -> &T {
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

#[derive(Clone)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
pub struct GenericMat<T: Ring> {
    #[cfg_attr(
        feature = "serde",
        serde(bound(
            serialize = "T: Serialize",
            deserialize = "T: Deserialize<'de>",
        ))
    )]
    pub(crate) ctx: Rc<GenericMatCtx<T>>,
    pub(crate) entries: Vec<Elem<T>>,
}

impl<T: Ring> fmt::Display for GenericMat<T>
where
    <T as Ring>::Element: fmt::Display
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let entries = self.get_entries();
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


impl<T: Ring> Eq for GenericMat<T> {}

impl<S: Ring, T: Ring> PartialEq<GenericMat<S>> for GenericMat<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
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

impl<S: Ring, T: Ring> PartialEq<&GenericMat<S>> for GenericMat<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
{
    #[inline]
    fn eq(&self, rhs: &&GenericMat<S>) -> bool {
        (&self).eq(rhs)
    }
}

impl<S: Ring, T: Ring> PartialEq<GenericMat<S>> for &GenericMat<T>
where
    <T as Ring>::Element: PartialEq<<S as Ring>::Element>,
{
    #[inline]
    fn eq(&self, rhs: &GenericMat<S>) -> bool {
        self.eq(&rhs)
    }
}

impl<T: Ring + Hash> Hash for GenericMat<T>
where
    <T as Ring>::Element: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.parent().hash(state);
        self.get_entries().hash(state);
    }
}

impl<T: Ring> Element for GenericMat<T> {
    type Parent = GenericMatSpace<T>;

    #[inline]
    fn parent(&self) -> GenericMatSpace<T> {
        GenericMatSpace {
            ctx: Rc::clone(&self.ctx),
        }
    }
}

impl<T: Ring> Operation<Additive> for GenericMat<T> {
    fn operate(&self, _right: &Self) -> Self {
        unimplemented!()
    }
}

impl<T: Ring> IsIdentity<Additive> for GenericMat<T> {
    fn is_identity(&self) -> bool {
        unimplemented!()
    }
}

impl<T: Ring> TwoSidedInverse<Additive> for GenericMat<T> {
    fn two_sided_inverse(&self) -> Self {
        unimplemented!()
    }
}

impl<T: Ring> MatrixSpaceElement<T> for GenericMat<T> {
    type Parent = GenericMatSpace<T>;

    #[inline]
    fn base_ring(&self) -> &T {
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
    fn get_entry(&self, _i: usize, _j: usize) -> Option<Elem<T>> {
        unimplemented!()
        //self.entries.get(i*self.ncols() + j)
    }

    #[inline]
    fn set_entry(&mut self, i: usize, j: usize, entry: Elem<T>) -> Option<Elem<T>> {
        let idx = i*self.ncols() + j;
        if idx >= self.len() {
            None
        } else {
            self.entries.push(entry);
            Some(self.entries.swap_remove(idx))
        }
    }

    #[inline]
    fn get_entries(&self) -> Vec<Elem<T>> {
        self.entries.clone()
    }

    /*
    #[inline]
    fn entry(&self, i: usize, j: usize) -> Option<&T> {
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
    }
    */
}
