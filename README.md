## inertia-generic

**inertia-generic** provides implementations of generic algebraic structures like 
polynomial rings, matrix spaces, and fraction fields which can be recursively 
defined. It is part of [inertia](https://github.com/wjyoumans/inertia) 
but is designed to be usable as a standalone crate (alongside 
[inertia-algebra](https://github.com/wjyoumans/inertia-algebra), which provides
traits for algebraic structures).

<!--
TODO:
 * improve equality of Parents. use both ptr::Eq and ops::Eq. (Should polynomial 
   rings be equal if they have equal base rings but different variables? Inertia will
   likely have trait for isomorphism, decide what inertia_generic should do)
 * account for sign in Display impl for GenericPoly to avoid "x + -1" etc.
 * either require Debug for Parents in inertia_algebra and derive it for generics, or 
   conditionally implement it when possible.
 -->
