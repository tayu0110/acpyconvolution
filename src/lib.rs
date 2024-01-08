mod convolution;
mod modint;
mod ntt;

pub use crate::convolution::convolution;
pub use crate::modint::*;
use pyo3::prelude::*;

#[pyfunction]
#[pyo3(name = "convolution")]
fn pyconvolution(a: Vec<u32>, b: Vec<u32>, modulo: KnownModulo) -> Vec<u32> {
    match modulo {
        KnownModulo::Mod167772161 => convolution::<Mod167772161>(a, b),
        KnownModulo::Mod377487361 => convolution::<Mod377487361>(a, b),
        KnownModulo::Mod469762049 => convolution::<Mod469762049>(a, b),
        KnownModulo::Mod595591169 => convolution::<Mod595591169>(a, b),
        KnownModulo::Mod645922817 => convolution::<Mod645922817>(a, b),
        KnownModulo::Mod754974721 => convolution::<Mod754974721>(a, b),
        KnownModulo::Mod880803841 => convolution::<Mod880803841>(a, b),
        KnownModulo::Mod897581057 => convolution::<Mod897581057>(a, b),
        KnownModulo::Mod998244353 => convolution::<Mod998244353>(a, b),
        KnownModulo::Mod1107296257 => convolution::<Mod1107296257>(a, b),
        KnownModulo::Mod1224736769 => convolution::<Mod1224736769>(a, b),
        KnownModulo::Mod1300234241 => convolution::<Mod1300234241>(a, b),
        KnownModulo::Mod1484783617 => convolution::<Mod1484783617>(a, b),
        KnownModulo::Mod1711276033 => convolution::<Mod1711276033>(a, b),
        KnownModulo::Mod1811939329 => convolution::<Mod1811939329>(a, b),
        KnownModulo::Mod2013265921 => convolution::<Mod2013265921>(a, b),
        KnownModulo::Mod2088763393 => convolution::<Mod2088763393>(a, b),
        KnownModulo::Mod2113929217 => convolution::<Mod2113929217>(a, b),
        KnownModulo::Mod2130706433 => convolution::<Mod2130706433>(a, b),
        KnownModulo::Mod2281701377 => convolution::<Mod2281701377>(a, b),
        KnownModulo::Mod2483027969 => convolution::<Mod2483027969>(a, b),
        KnownModulo::Mod2533359617 => convolution::<Mod2533359617>(a, b),
        KnownModulo::Mod2634022913 => convolution::<Mod2634022913>(a, b),
        KnownModulo::Mod2717908993 => convolution::<Mod2717908993>(a, b),
        KnownModulo::Mod2868903937 => convolution::<Mod2868903937>(a, b),
        KnownModulo::Mod2885681153 => convolution::<Mod2885681153>(a, b),
        KnownModulo::Mod3221225473 => convolution::<Mod3221225473>(a, b),
        KnownModulo::Mod3238002689 => convolution::<Mod3238002689>(a, b),
        KnownModulo::Mod3489660929 => convolution::<Mod3489660929>(a, b),
        KnownModulo::Mod3892314113 => convolution::<Mod3892314113>(a, b),
        KnownModulo::Mod3942645761 => convolution::<Mod3942645761>(a, b),
        KnownModulo::Mod4076863489 => convolution::<Mod4076863489>(a, b),
        KnownModulo::Mod4194304001 => convolution::<Mod4194304001>(a, b),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn acpyconvolution(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(pyconvolution, m)?)?;
    m.add("M167772161", M167772161)?;
    m.add("M377487361", M377487361)?;
    m.add("M469762049", M469762049)?;
    m.add("M595591169", M595591169)?;
    m.add("M645922817", M645922817)?;
    m.add("M754974721", M754974721)?;
    m.add("M880803841", M880803841)?;
    m.add("M897581057", M897581057)?;
    m.add("M998244353", M998244353)?;
    m.add("M1107296257", M1107296257)?;
    m.add("M1224736769", M1224736769)?;
    m.add("M1300234241", M1300234241)?;
    m.add("M1484783617", M1484783617)?;
    m.add("M1711276033", M1711276033)?;
    m.add("M1811939329", M1811939329)?;
    m.add("M2013265921", M2013265921)?;
    m.add("M2088763393", M2088763393)?;
    m.add("M2113929217", M2113929217)?;
    m.add("M2130706433", M2130706433)?;
    m.add("M2281701377", M2281701377)?;
    m.add("M2483027969", M2483027969)?;
    m.add("M2533359617", M2533359617)?;
    m.add("M2634022913", M2634022913)?;
    m.add("M2717908993", M2717908993)?;
    m.add("M2868903937", M2868903937)?;
    m.add("M2885681153", M2885681153)?;
    m.add("M3221225473", M3221225473)?;
    m.add("M3238002689", M3238002689)?;
    m.add("M3489660929", M3489660929)?;
    m.add("M3892314113", M3892314113)?;
    m.add("M3942645761", M3942645761)?;
    m.add("M4076863489", M4076863489)?;
    m.add("M4194304001", M4194304001)?;
    Ok(())
}
