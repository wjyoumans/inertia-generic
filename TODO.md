TODO:
 * Eq: improve equality of Parents. use both ptr::Eq and ops::Eq. (Should polynomial 
   rings be equal if they have equal base rings but different variables? etc.)
 * Display: account for sign in Display impl for GenericPoly to avoid "x + -1" etc.
 * Debug: either require Debug for Parents in inertia_algebra and derive it for 
 generics, or conditionally implement it when possible.
 * make sure serde still works
 * ops: Polynomial eval, exponentiation, assignment, scalar ops etc
 * conversions
 * noncommutative variants (infrastructure is there but it will take some work)
