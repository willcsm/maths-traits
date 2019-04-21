
use algebra::*;
use analysis::*;

pub trait Trig: Field {
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    #[inline] fn sin_cos(self) -> (Self, Self) {(self.clone().sin(), self.cos())}

    #[inline] fn sec(self) -> Self { self.cos().inv() }
    #[inline] fn csc(self) -> Self { self.sin().inv() }
    #[inline] fn cot(self) -> Self { self.tan().inv() }

    fn sinh(self) -> Self;
    fn cosh(self) -> Self;
    fn tanh(self) -> Self;

    #[inline] fn sech(self) -> Self { self.cosh().inv() }
    #[inline] fn csch(self) -> Self { self.sinh().inv() }
    #[inline] fn coth(self) -> Self { self.tanh().inv() }

    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(y: Self, x: Self) -> Self;

    #[inline] fn asec(self) -> Self { self.inv().acos() }
    #[inline] fn acsc(self) -> Self { self.inv().asin() }
    #[inline] fn acot(self) -> Self { self.inv().atan() }
    #[inline] fn acot2(x: Self, y: Self) -> Self { Self::atan2(y, x) }

    fn asinh(self) -> Self;
    fn acosh(self) -> Self;
    fn atanh(self) -> Self;

    #[inline] fn asech(self) -> Self { self.inv().acosh() }
    #[inline] fn acsch(self) -> Self { self.inv().asinh() }
    #[inline] fn acoth(self) -> Self { self.inv().atanh() }
}

pub trait Exponential: Field {
    fn pow(self, power:Self) -> Self;
    fn exp(self) -> Self;
    #[inline] fn exp2(self) -> Self {self.pow(Self::one().mul_z(2))}
    #[inline] fn exp10(self) -> Self {
        self.pow(Self::one().mul_z(10))
    }

    fn log(self, base:Self) -> Self;
    fn ln(self) -> Self;
    #[inline] fn log2(self) -> Self {self.log(Self::one()+Self::one())}
    #[inline] fn log10(self) -> Self {
        self.log(Self::one().mul_z(10))
    }

    #[inline] fn root(self, index:Self) -> Self {self.pow(index.inv())}
    #[inline] fn sqrt(self) -> Self {self.root(Self::one().mul_z(2))}
    #[inline] fn cbrt(self) -> Self {self.root(Self::one().mul_z(3))}

    #[inline] fn ln_1p(self) -> Self {(self-Self::one()).ln()}
    #[inline] fn exp_m1(self) -> Self {self.exp()-Self::one()}
}

pub trait RealConstants: Field + Trig + Exponential {
    fn e() -> Self;
    fn ln_2() -> Self;
    fn ln_10() -> Self;
    fn log2_e() -> Self;
    fn log10_e() -> Self;
    fn log2_10() -> Self;
    fn log10_2() -> Self;

    fn pi() -> Self;
    fn frac_2_pi() -> Self;
    fn frac_2_sqrt_pi() -> Self;
    fn frac_pi_2() -> Self;
    fn frac_pi_3() -> Self;
    fn frac_pi_4() -> Self;
    fn frac_pi_6() -> Self;
    fn frac_pi_8() -> Self;

    fn sqrt_2() -> Self;
    fn frac_1_sqrt_2() -> Self;

    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
}

pub trait ComplexSubset: PartialEq + Clone + Semiring {
    type Real: Real
        + ComplexSubset<Natural = Self::Natural, Integer = Self::Integer, Real = Self::Real>;
    type Natural: Natural
        + IntegerSubset<Signed = Self::Integer, Unsigned = Self::Natural>
        + ComplexSubset<Natural = Self::Natural, Integer = Self::Integer, Real = Self::Real>;
    type Integer: Integer
        + IntegerSubset<Signed = Self::Integer, Unsigned = Self::Natural>
        + ComplexSubset<Natural = Self::Natural, Integer = Self::Integer, Real = Self::Real>;

    fn as_real(self) -> Self::Real;
    fn as_natural(self) -> Self::Natural;
    fn as_integer(self) -> Self::Integer;

    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;

    fn trunc(self) -> Self;
    fn fract(self) -> Self;

    fn im(self) -> Self;
    fn re(self) -> Self;
    fn conj(self) -> Self;
}

// auto!(trait CastFloat = TryFrom<f32> + TryFrom<f64> + TryInto<f32> + TryInto<f64>;);

auto!{
    pub trait ComplexField =
        Field +
        ComplexSubset + /*CastFloat +*/
        RealConstants + Trig + Exponential +
        PowN + PowZ;
}

pub trait Real:
    ArchimedeanField +
    ComplexSubset<Real=Self> + /*CastFloat +*/
    RealConstants + Trig + Exponential +
    PowN + PowZ
{
    fn approx(self) -> f32;
    fn repr(f: f64) -> Self;
}

pub trait Complex: ComplexField {
    fn i() -> Self;
    fn mul_i(self) -> Self;
    fn div_i(self) -> Self;
}

macro_rules! impl_real {
    ($($f:ident:$n:ident:$z:ident)*) => {$(
        impl Trig for $f {
            #[inline(always)] fn sin(self) -> Self {$f::sin(self)}
            #[inline(always)] fn cos(self) -> Self {$f::cos(self)}
            #[inline(always)] fn tan(self) -> Self {$f::tan(self)}
            #[inline(always)] fn sin_cos(self) -> (Self,Self) {$f::sin_cos(self)}

            #[inline(always)] fn sinh(self) -> Self {$f::sinh(self)}
            #[inline(always)] fn cosh(self) -> Self {$f::cosh(self)}
            #[inline(always)] fn tanh(self) -> Self {$f::tanh(self)}

            #[inline(always)] fn asin(self) -> Self {$f::asin(self)}
            #[inline(always)] fn acos(self) -> Self {$f::acos(self)}
            #[inline(always)] fn atan(self) -> Self {$f::atan(self)}
            #[inline(always)] fn atan2(y:Self, x:Self) -> Self {$f::atan2(y,x)}

            #[inline(always)] fn asinh(self) -> Self {$f::asinh(self)}
            #[inline(always)] fn acosh(self) -> Self {$f::acosh(self)}
            #[inline(always)] fn atanh(self) -> Self {$f::atanh(self)}
        }

        impl Exponential for $f {
            #[inline(always)] fn pow(self, power:Self) -> Self {$f::powf(self,power)}
            #[inline(always)] fn exp(self) -> Self {$f::exp(self)}
            #[inline(always)] fn exp2(self) -> Self {$f::exp2(self)}
            #[inline(always)] fn exp10(self) -> Self {$f::powf(self, 10.0)}

            #[inline(always)] fn log(self, base:Self) -> Self {$f::log(self,base)}
            #[inline(always)] fn ln(self) -> Self {$f::ln(self)}
            #[inline(always)] fn log2(self) -> Self {$f::log2(self)}
            #[inline(always)] fn log10(self) -> Self {$f::log10(self)}

            #[inline(always)] fn root(self, index:Self) -> Self {self.pow(index.recip())}
            #[inline(always)] fn sqrt(self) -> Self {$f::sqrt(self)}
            #[inline(always)] fn cbrt(self) -> Self {$f::cbrt(self)}

            #[inline(always)] fn ln_1p(self) -> Self {$f::ln_1p(self)}
            #[inline(always)] fn exp_m1(self) -> Self {$f::exp_m1(self)}
        }

        impl RealConstants for $f {
            #[inline(always)] fn e() -> Self {::std::$f::consts::E}
            #[inline(always)] fn ln_2() -> Self {::std::$f::consts::LN_2}
            #[inline(always)] fn ln_10() -> Self {::std::$f::consts::LN_10}
            #[inline(always)] fn log2_e() -> Self {::std::$f::consts::LOG2_E}
            #[inline(always)] fn log10_e() -> Self {::std::$f::consts::LOG10_E}
            #[inline(always)] fn log2_10() -> Self {::std::$f::consts::LOG2_10}
            #[inline(always)] fn log10_2() -> Self {::std::$f::consts::LOG10_2}

            #[inline(always)] fn pi() -> Self {::std::$f::consts::PI}
            #[inline(always)] fn frac_2_pi() -> Self {::std::$f::consts::FRAC_2_PI}
            #[inline(always)] fn frac_2_sqrt_pi() -> Self {::std::$f::consts::FRAC_2_SQRT_PI}
            #[inline(always)] fn frac_pi_2() -> Self {::std::$f::consts::FRAC_PI_2}
            #[inline(always)] fn frac_pi_3() -> Self {::std::$f::consts::FRAC_PI_3}
            #[inline(always)] fn frac_pi_4() -> Self {::std::$f::consts::FRAC_PI_4}
            #[inline(always)] fn frac_pi_6() -> Self {::std::$f::consts::FRAC_PI_6}
            #[inline(always)] fn frac_pi_8() -> Self {::std::$f::consts::FRAC_PI_8}

            #[inline(always)] fn sqrt_2() -> Self {::std::$f::consts::SQRT_2}
            #[inline(always)] fn frac_1_sqrt_2() -> Self {::std::$f::consts::FRAC_1_SQRT_2}

            #[inline(always)] fn to_degrees(self) -> Self { $f::to_degrees(self) }
            #[inline(always)] fn to_radians(self) -> Self { $f::to_radians(self) }
        }

        impl ComplexSubset for $f {
            type Real = $f;
            type Natural = $n;
            type Integer = $z;

            #[inline(always)] fn as_real(self) -> Self::Real {self}
            #[inline(always)] fn as_natural(self) -> Self::Natural {self as $n}
            #[inline(always)] fn as_integer(self) -> Self::Integer {self as $z}

            #[inline(always)] fn floor(self) -> Self {$f::floor(self)}
            #[inline(always)] fn ceil(self) -> Self {$f::ceil(self)}
            #[inline(always)] fn round(self) -> Self {$f::round(self)}

            #[inline(always)] fn trunc(self) -> Self {$f::trunc(self)}
            #[inline(always)] fn fract(self) -> Self {$f::fract(self)}

            #[inline(always)] fn im(self) -> Self {self}
            #[inline(always)] fn re(self) -> Self {self}
            #[inline(always)] fn conj(self) -> Self {self}
        }

        impl ComplexSubset for $n {
            type Real = $f;
            type Natural = $n;
            type Integer = $z;

            #[inline(always)] fn as_real(self) -> Self::Real {self as $f}
            #[inline(always)] fn as_natural(self) -> Self::Natural {self}
            #[inline(always)] fn as_integer(self) -> Self::Integer {self as $z}

            #[inline(always)] fn floor(self) -> Self {self}
            #[inline(always)] fn ceil(self) -> Self {self}
            #[inline(always)] fn round(self) -> Self {self}

            #[inline(always)] fn trunc(self) -> Self {self}
            #[inline(always)] fn fract(self) -> Self {0}

            #[inline(always)] fn im(self) -> Self {self}
            #[inline(always)] fn re(self) -> Self {self}
            #[inline(always)] fn conj(self) -> Self {self}
        }

        impl ComplexSubset for $z {
            type Real = $f;
            type Natural = $n;
            type Integer = $z;

            #[inline(always)] fn as_real(self) -> Self::Real {self as $f}
            #[inline(always)] fn as_natural(self) -> Self::Natural {self as $n}
            #[inline(always)] fn as_integer(self) -> Self::Integer {self}

            #[inline(always)] fn floor(self) -> Self {self}
            #[inline(always)] fn ceil(self) -> Self {self}
            #[inline(always)] fn round(self) -> Self {self}

            #[inline(always)] fn trunc(self) -> Self {self}
            #[inline(always)] fn fract(self) -> Self {0}

            #[inline(always)] fn im(self) -> Self {self}
            #[inline(always)] fn re(self) -> Self {self}
            #[inline(always)] fn conj(self) -> Self {self}
        }

        impl Real for $f {
            #[inline(always)] fn approx(self) -> f32 {self as f32}
            #[inline(always)] fn repr(f: f64) -> Self {f as $f}
        }

    )*}
}

impl_real!(f32:u32:i32 f64:u64:i64);
