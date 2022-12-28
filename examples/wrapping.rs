use inertia_algebra::*;
use inertia_algebra::ops::*;
use inertia_algebra::structures::*;
use inertia_algebra::properties::*;

use inertia_generic::New;
use inertia_generic::poly::*;
use inertia_generic::poly::generic::GenericPolyRing;

use std::fmt;
use std::marker::PhantomData;

/// Ring of primitive integers wrapping around at the boundary.
#[derive(Clone, Debug)]
pub struct WrappingRing<T>(PhantomData<T>);

impl<T> WrappingRing<T> {
    pub fn init() -> Self {
        WrappingRing(PhantomData)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Wrap<T>(T);

macro_rules! impl_wrapping {
    ($($t:ident)*) => ($(

        impl Parent for WrappingRing<$t> {
            type Element = Wrap<$t>;
        }
        
        impl Element for Wrap<$t> {
            type Parent = WrappingRing<$t>;
            fn parent(&self) -> Self::Parent {
                WrappingRing::init()
            }
        }

        // Additive properties

        impl Operation<Additive> for Wrap<$t> {
            fn operate(&self, right: &Self) -> Self {
                Wrap(self.0.wrapping_add(right.0))
            }
        }
        
        impl Identity<Additive> for WrappingRing<$t> {
            fn identity(&self) -> Wrap<$t> {
                Wrap(0)
            }
        }
        
        impl IsIdentity<Additive> for Wrap<$t> {
            fn is_identity(&self) -> bool {
                self.0 == 0
            }
        }

        impl TwoSidedInverse<Additive> for Wrap<$t> {
            fn two_sided_inverse(&self) -> Self {
                Wrap(self.0.wrapping_neg())
            }
        }
        
        impl Divisible<Additive> for WrappingRing<$t> {}
        
        impl Associative<Additive> for WrappingRing<$t> {}
        
        impl Commutative<Additive> for WrappingRing<$t> {}
        
        // Multiplicative properties

        impl Operation<Multiplicative> for Wrap<$t> {
            fn operate(&self, right: &Self) -> Self {
                Wrap(self.0.wrapping_mul(right.0))
            }
        }

        impl Identity<Multiplicative> for WrappingRing<$t> {
            fn identity(&self) -> Wrap<$t> {
                Wrap(1)
            }
        }
        
        impl IsIdentity<Multiplicative> for Wrap<$t> {
            fn is_identity(&self) -> bool {
                self.0 == 1
            }
        }
        
        impl Associative<Multiplicative> for WrappingRing<$t> {}
        
        impl Commutative<Multiplicative> for WrappingRing<$t> {}
       
        // Ring-like properties

        impl Distributive for WrappingRing<$t> {}
    )*);
}

impl_wrapping! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }

macro_rules! impl_poly {
    ($($t:ident)*) => ($(
        impl AddAssign<&Wrap<$t>> for Wrap<$t> {
            fn add_assign(&mut self, rhs: &Self) {
                *self = Wrap(self.0.wrapping_add(rhs.0))
            }
        }
        
        impl AddFrom<&Wrap<$t>> for Wrap<$t> {
            fn add_from(&mut self, lhs: &Self) {
                *self = Wrap(lhs.0.wrapping_add(self.0))
            }
        }
       
        impl NegAssign for Wrap<$t> {
            fn neg_assign(&mut self) {
                *self = Wrap(self.0.wrapping_neg())
            }
        }

        impl SubAssign<&Wrap<$t>> for Wrap<$t> {
            fn sub_assign(&mut self, rhs: &Self) {
                *self = Wrap(self.0.wrapping_sub(rhs.0))
            }
        }
        
        impl SubFrom<&Wrap<$t>> for Wrap<$t> {
            fn sub_from(&mut self, lhs: &Self) {
                *self = Wrap(lhs.0.wrapping_sub(self.0))
            }
        }

        impl MulAssign<&Wrap<$t>> for Wrap<$t> {
            fn mul_assign(&mut self, rhs: &Self) {
                *self = Wrap(self.0.wrapping_mul(rhs.0))
            }
        }
        
        impl MulFrom<&Wrap<$t>> for Wrap<$t> {
            fn mul_from(&mut self, lhs: &Self) {
                *self = Wrap(lhs.0.wrapping_mul(self.0))
            }
        }

        impl PolyableRing for WrappingRing<$t> {
            type InnerPolyRing = GenericPolyRing<Self>;
        }

        impl New<$t> for WrappingRing<$t> {
            fn new(&self, src: $t) -> Wrap<$t> {
                Wrap(src)
            }
        }       

        impl fmt::Display for Wrap<$t> {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    )*)
}

impl_poly! { usize u8 u16 u32 u64 u128 isize i8 i16 i32 i64 i128 }


fn main() {
    let zn = WrappingRing::<i8>::init();

    let znx = PolyRing::init(&zn, "x");
    let f = znx.new([1, 0, 1]);
    let g = znx.new([3, 2, 1]);
    println!("f = {}", &f);
    println!("g = {}", &g);
    println!("f - g = {}", f-g);

    let x = zn.new(-20);
    println!("x = {}", -20);

    let y = zn.new(25);
    println!("y = {}", 25);
    
    let z = x.op(Multiplicative, &y);
    println!("x*y mod 2^8 = {}", z.0);
}
