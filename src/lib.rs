#![allow(dead_code)]

struct Zero;
struct Suc<T> {
    _phantom: std::marker::PhantomData<T>,
}

trait Number {}
impl Number for Zero {}
impl<T: Number> Number for Suc<T> {}

trait NormalForm {
    type NF: Number;
}

impl NormalForm for Zero {
    type NF = Zero;
}

impl<T: NormalForm> NormalForm for Suc<T> {
    type NF = Suc<T::NF>;
}

trait SameNormalForm {}
impl<A: NormalForm, B: NormalForm> SameNormalForm for (A, B) where (A::NF, B::NF): SameNumber {}

trait SameNumber {}
impl<T> SameNumber for (T, T) where T: Number {}

type One = Suc<Zero>;
type Two = Suc<One>;
type Three = Suc<Two>;
type Four = Suc<Three>;
type Five = Suc<Four>;
type Six = Suc<Five>;
type Seven = Suc<Six>;
type Eight = Suc<Seven>;
type Nine = Suc<Eight>;
type Ten = Suc<Nine>;

struct Same<A, B>
where
    A: NormalForm,
    B: NormalForm,
    (A, B): SameNormalForm,
{
    _phantom: std::marker::PhantomData<(A, B)>,
}

struct Add<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<B: NormalForm> NormalForm for Add<Zero, B> {
    type NF = B::NF;
}

impl<A: NormalForm, B: NormalForm> NormalForm for Add<Suc<A>, B>
where
    Add<A, B>: NormalForm,
{
    type NF = <Suc<Add<A, B>> as NormalForm>::NF;
}

struct Mul<A, B> {
    _phantom: std::marker::PhantomData<(A, B)>,
}

impl<B: NormalForm> NormalForm for Mul<Zero, B> {
    type NF = Zero;
}

impl<A: NormalForm, B: NormalForm> NormalForm for Mul<Suc<A>, B>
where
    Add<B, Mul<A, B>>: NormalForm,
{
    type NF = <Add<B, Mul<A, B>> as NormalForm>::NF;
}

struct Fac<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl NormalForm for Fac<Zero> {
    type NF = Suc<Zero>;
}

impl<T: NormalForm> NormalForm for Fac<Suc<T>>
where
    Fac<T>: NormalForm,
    Mul<Suc<T>, <Fac<T> as NormalForm>::NF>: NormalForm,
{
    type NF = <Mul<Suc<T>, <Fac<T> as NormalForm>::NF> as NormalForm>::NF;
}

fn main() {
    let _: Same<Zero, Zero>; // OK
    let _: Same<One, One>;
    // 1 + 1 == 2
    let _: Same<Add<One, One>, Two>;
    let _: Same<Mul<Two, Two>, Four>;
    // factorial of 4 is 24
    let _: Same<Fac<Four>, Add<Four, Mul<Two, Ten>>>;
    // and factorial of 4 is not 23
    let _: Same<Fac<Four>, Add<Three, Mul<Two, Ten>>>;
}
