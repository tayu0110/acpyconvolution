#[cfg(target_arch = "x86")]
use std::arch::x86::__m256i;
#[cfg(target_arch = "x86_64")]
use std::arch::x86_64::__m256i;
use std::fmt::Debug;
use std::marker;
#[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
use std::mem::transmute;

use pyo3::prelude::*;

pub trait Modulo: Clone + marker::Copy + PartialEq + Eq + Debug {
    const N: u32;
    const N2: u32 = Self::N.wrapping_mul(2);
    // N * N_INV = 1 mod R
    const N_INV: u32 = {
        let mut inv = Self::N.wrapping_mul(2u32.wrapping_sub(Self::N.wrapping_mul(Self::N)));
        while Self::N.wrapping_mul(inv) != 1 {
            inv = inv.wrapping_mul(2u32.wrapping_sub(Self::N.wrapping_mul(inv)));
        }
        inv
    };
    // NN' = -1 mod R
    const N_PRIME: u32 = Self::N_INV.wrapping_neg();
    // R = 2^32 mod N
    const R: u32 = ((1u64 << 32) % Self::N as u64) as u32;
    // R2 = 2^64 mod N
    const R2: u32 = ((Self::N as u64).wrapping_neg() % Self::N as u64) as u32;
    const PRIM_ROOT: u32;
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const NX8: __m256i = unsafe { transmute([Self::N; 8]) };
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const N2X8: __m256i = unsafe { transmute([Self::N2; 8]) };
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const N_INVX8: __m256i = unsafe { transmute([Self::N_INV; 8]) };
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const N_PRIMEX8: __m256i = unsafe { transmute([Self::N_PRIME; 8]) };
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const RX8: __m256i = unsafe { transmute([Self::R; 8]) };
    #[cfg(any(target_arch = "x86_64", target_arch = "x86"))]
    const R2X8: __m256i = unsafe { transmute([Self::R2; 8]) };
}

macro_rules! impl_modulo {
    ( $({ $name:ident, $c:ident, $modulo:literal, $prim_root:literal },)* ) => {
        $(
            #[pyclass]
            #[derive(Debug, Clone, marker::Copy, PartialEq, Eq)]
            pub struct $name;
            impl Modulo for $name {
                const N: u32 = $modulo;
                const PRIM_ROOT: u32 = $prim_root;
            }

            pub const $c : KnownModulo = KnownModulo::$name;
        )*
    };
}

// https://www.mathenachia.blog/ntt-mod-list-01/
impl_modulo!(
    { Mod167772161, M167772161, 167772161, 3 },
    { Mod377487361, M377487361, 377487361, 7 },
    { Mod469762049, M469762049, 469762049, 3 },
    { Mod595591169, M595591169, 595591169, 3 },
    { Mod645922817, M645922817, 645922817, 3 },
    { Mod754974721, M754974721, 754974721, 11 },
    { Mod880803841, M880803841, 880803841, 26 },
    { Mod897581057, M897581057, 897581057, 3 },
    { Mod998244353, M998244353, 998244353, 3 },
    { Mod1107296257, M1107296257, 1107296257, 10 },
    { Mod1224736769, M1224736769, 1224736769, 3 },
    { Mod1300234241, M1300234241, 1300234241, 3 },
    { Mod1484783617, M1484783617, 1484783617, 5 },
    { Mod1711276033, M1711276033, 1711276033, 29 },
    { Mod1811939329, M1811939329, 1811939329, 13 },
    { Mod2013265921, M2013265921, 2013265921, 31 },
    { Mod2088763393, M2088763393, 2088763393, 5 },
    { Mod2113929217, M2113929217, 2113929217, 5 },
    { Mod2130706433, M2130706433, 2130706433, 3 },
    { Mod2281701377, M2281701377, 2281701377, 3 },
    { Mod2483027969, M2483027969, 2483027969, 3 },
    { Mod2533359617, M2533359617, 2533359617, 3 },
    { Mod2634022913, M2634022913, 2634022913, 3 },
    { Mod2717908993, M2717908993, 2717908993, 5 },
    { Mod2868903937, M2868903937, 2868903937, 35 },
    { Mod2885681153, M2885681153, 2885681153, 3 },
    { Mod3221225473, M3221225473, 3221225473, 5 },
    { Mod3238002689, M3238002689, 3238002689, 3 },
    { Mod3489660929, M3489660929, 3489660929, 3 },
    { Mod3892314113, M3892314113, 3892314113, 3 },
    { Mod3942645761, M3942645761, 3942645761, 3 },
    { Mod4076863489, M4076863489, 4076863489, 7 },
    { Mod4194304001, M4194304001, 4194304001, 3 },

);

#[pyclass]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum KnownModulo {
    Mod167772161,
    Mod377487361,
    Mod469762049,
    Mod595591169,
    Mod645922817,
    Mod754974721,
    Mod880803841,
    Mod897581057,
    Mod998244353,
    Mod1107296257,
    Mod1224736769,
    Mod1300234241,
    Mod1484783617,
    Mod1711276033,
    Mod1811939329,
    Mod2013265921,
    Mod2088763393,
    Mod2113929217,
    Mod2130706433,
    Mod2281701377,
    Mod2483027969,
    Mod2533359617,
    Mod2634022913,
    Mod2717908993,
    Mod2868903937,
    Mod2885681153,
    Mod3221225473,
    Mod3238002689,
    Mod3489660929,
    Mod3892314113,
    Mod3942645761,
    Mod4076863489,
    Mod4194304001,
}
