pub fn zero(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    t0 - i * ((t0 - tn) / n)
}

pub fn um(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    t0 * (tn / t0).powf(i / n)
}

pub fn dois(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    let a: f64 = ((t0 - tn) * (n + 1.0)) / n;
    let b: f64 = t0 - a;

    return (a / (i + 1.0)) + b;
}

pub fn tres(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    let a: f64 = (t0 - tn).ln() / n.ln();

    return t0 - i.powf(a);
}

pub fn cinco(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    return ((t0 - tn) / (1.0 + (3.0 * (i - (n / 2.0))).exp())) + tn;
}

pub fn seis(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    return ((1.0 / 2.0) * (t0 - tn)) * (1.0 - (((10.0 * i) / n) - 5.0).tanh()) + tn;
}

pub fn sete(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    return ((t0 - tn) / ((10.0 * i) / n).cosh()) + tn;
}

pub fn oito(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    let a: f64 = (1.0 / n) * (t0 / tn).ln();

    return t0 * (-a * i).exp();
}

pub fn nove(i: f64, t0: f64, tn: f64, n: f64) -> f64 {
    let a: f64 = (1.0 / n.powi(2)) * (t0 / tn).ln();

    return t0 * (-a * i.powi(2)).exp();
}
