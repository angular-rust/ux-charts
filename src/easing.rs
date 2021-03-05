#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

/// The easing function type.
///
/// An easing function takes an input number [t] in range 0..1, inclusive, and
/// returns a non-negative amount. In addition, the function must return 1 for
/// [t] = 1.

use std::f64::consts::PI;

pub type EasingFunction =  fn(t: f64) -> f64;

fn linear(amount: f64) -> f64 {
    amount
}

fn quad_in(amount: f64) -> f64 {
    amount * amount
}

fn quad_out(amount: f64) -> f64 {
    amount * (2.0 - amount)
}

fn quad_in_out(amount: f64) -> f64 {
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return 0.5 * amount * amount;
    }
    amount = amount - 1.0;
    0.5 * (1.0 - amount * (amount - 2.0))
}

fn cubic_in(amount: f64) -> f64 {
    amount * amount * amount
}

fn cubic_out(amount: f64) -> f64 {
    let amount = amount - 1.0;
    amount * amount * amount + 1.0
}

fn cubic_in_out(amount: f64) -> f64 {
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return 0.5 * amount * amount * amount;
    }
    amount = amount - 2.0;
    0.5 * (amount * amount * amount + 2.0)
}

fn quart_in(amount: f64) -> f64 {
    amount * amount * amount * amount
}

fn quart_out(amount: f64) -> f64 {
    let amount = amount - 1.0;
    1.0 - amount * amount * amount * amount
}

fn quart_in_out(amount: f64) -> f64 {
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return 0.5 * amount * amount * amount * amount;
    }
    amount = amount - 2.0;
    0.5 * (2.0 - amount * amount * amount * amount)
}

fn quint_in(amount: f64) -> f64 {
    amount * amount * amount * amount * amount
}

fn quint_out(amount: f64) -> f64 {
    let amount = amount - 1.0;
    amount * amount * amount * amount * amount + 1.0
}

fn quint_in_out(amount: f64) -> f64 {
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return 0.5 * amount * amount * amount * amount * amount;
    }

    amount = amount - 2.0;
    0.5 * (amount * amount * amount * amount * amount + 2.0)
}

fn sine_in(amount: f64) -> f64 {
    1.0 - (amount * PI / 2.0).cos()
}

fn sine_out(amount: f64) -> f64 {
    (amount * PI / 2.0).sin()
}

fn sine_in_out(t: f64) -> f64 {
    0.5 * (1.0 - (PI * t).cos())
}

fn expo_in(amount: f64) -> f64 {
    if amount == 0.0 {
        return 1.0;
    }

    f64::powf(2.0, 10.0 * (amount - 1.0))
}

fn expo_out(amount: f64) -> f64 {
    if amount == 1.0 {
        return 1.0;
    }

    1.0 - f64::powf(2.0, -10.0 * amount)
}

fn expo_in_out(amount: f64) -> f64 {
    if amount == 0.0 {
        return 0.0;
    }
    if amount == 1.0 {
        return 1.0;
    }
    let amount = amount * 2.0;
    if amount < 1.0 {
        return 1.0 / 2.0 * f64::powf(2.0, 10.0 * (amount - 1.0));
    }

    0.5 * (-f64::powf(2.0, -10.0 * --amount) + 2.0)
}

fn circ_in(amount: f64) -> f64 {
    if amount >= 1.0 {
        return amount;
    }

    1.0 - (1.0 - amount * amount).sqrt()
}

fn circ_out(amount: f64) -> f64 {
    let amount = amount - 1.0;
    (1.0 - amount * amount).sqrt()
}

fn circ_in_out(amount: f64) -> f64 {
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return -0.5 * ((1.0 - amount * amount).sqrt() - 1.0);
    }
    amount = amount - 2.0;
    0.5 * ((1.0 - amount * amount).sqrt() + 1.0)
}

fn elastic_in(amount: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    if amount == 0.0 {
        return 0.0;
    }
    if amount == 1.0 {
        return 1.0;
    }
    if p == 0.0 {
        p = 0.3;
    }

    #[allow(unused_assignments)]
    let mut s = 1.70158;
    if a < 1.0 {
        a = 1.0;
        s = p / 4.0;
    } else {
        s = p / (2.0 * PI) * f64::asin(1.0 / a);
    }

    let amount = amount - 1.0;
    -(a * f64::powf(2.0, 10.0 * amount) * ((amount - s) * (2.0 * PI) / p).sin())
}

fn elastic_out(amount: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    
    if amount == 0.0 {
        return 0.0;
    }
    
    if amount == 1.0 {
        return 1.0;
    }
    
    if p == 0.0 {
        p = 0.3;
    }

    #[allow(unused_assignments)]
    let mut s = 1.70158;
    if a < 1.0 {
        a = 1.0;
        s = p / 4.0;
    } else {
        s = p / (2.0 * PI) * f64::asin(1.0 / a);
    }

    a * f64::powf(2.0, -10.0 * amount) * ((amount - s) * (2.0 * PI) / p).sin() + 1.0
}

fn elastic_in_out(amount: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    
    if amount == 0.0 {
        return 0.0;
    }
    
    if amount == 1.0 {
        return 1.0;
    }

    if p == 0.0 {
        p = 1.0 * (0.3 * 1.5);
    }

    #[allow(unused_assignments)]
    let mut s = 1.70158;
    if a < 1.0 {
        a = 1.0;
        s = p / 4.0;
    } else {
        s = p / (2.0 * PI) * f64::asin(1.0 / a);
    }

    let amount = 2.0 * amount - 1.0;
    if amount < 0.0 {
        return -0.5 * (a * f64::powf(2.0, 10.0 * amount) * ((amount - s) * (2.0 * PI) / p).sin());
    }

    a * f64::powf(2.0, -10.0 * amount) * ((amount - s) * (2.0 * PI) / p).sin() * 0.5 + 1.0
}

fn back_in(amount: f64) -> f64 {
    let s = 1.70158;
    amount * amount * ((s + 1.0) * amount - s)
}

fn back_out(amount: f64) -> f64 {
    let s = 1.70158;
    let amount = amount - 1.0;
    amount * amount * ((s + 1.0) * amount + s) + 1.0
}

fn back_in_out(amount: f64) -> f64 {
    let s = 1.70158 * 1.525;
    let mut amount = amount * 2.0;
    if amount < 1.0 {
        return 0.5 * (amount * amount * ((s + 1.0) * amount - s));
    }
    amount = amount - 2.0;
    0.5 * (amount * amount * ((s + 1.0) * amount + s) + 2.0)
}

fn bounce_in(amount: f64) -> f64 {
    1.0 - bounce_out(1.0 - amount)
}

fn bounce_out(t: f64) -> f64 {
    let mut amount = t;
    if amount < 1.0 / 2.75 {
        return 7.5625 * amount * amount;
    } else if amount < 2.0 / 2.75 {
        amount -= 1.5 / 2.75;
        return 7.5625 * amount * amount + 0.75;
    } else if amount < 2.5 / 2.75 {
        amount -= 2.25 / 2.75;
        return 7.5625 * amount * amount + 0.9375;
    } else {
        amount -= 2.625 / 2.75;
        return 7.5625 * amount * amount + 0.984375;
    }
}

fn bounce_in_out(amount: f64) -> f64 {
    if amount < 0.5 {
        return bounce_in(amount * 2.0) * 0.5;
    }

    bounce_out(amount * 2.0 - 1.0) * 0.5 + 1.0 * 0.5
}

pub enum Easing {
    Linear,
    QuadIn,
    QuadOut,
    QuadInOut,
    CubicIn,
    CubicOut,
    CubicInOut,
    QuartIn,
    QuartOut,
    QuartInOut,
    QuintIn,
    QuintOut,
    QuintInOut,
    SineIn,
    SineOut,
    SineInOut,
    ExpoIn,
    ExpoOut,
    ExpoInOut,
    CircIn,
    CircOut,
    CircInOut,
    ElasticIn,
    ElasticOut,
    ElasticInOut,
    BackIn,
    BackOut,
    BackInOut,
    BounceIn,
    BounceOut,
    BounceInOut,
}

/// Returns the easing function with the given [name].
///
/// [name] can be an [EasingFunction] or a [String] specifying the name of one
/// of the easing functions defined above.
pub fn get_easing(etype: Easing) -> EasingFunction {
    match etype {
      Easing::Linear => linear,
      Easing::QuadIn => quad_in,
      Easing::QuadOut => quad_out,
      Easing::QuadInOut => quad_in_out,
      Easing::CubicIn => cubic_in,
      Easing::CubicOut => cubic_out,
      Easing::CubicInOut => cubic_in_out,
      Easing::QuartIn => quart_in,
      Easing::QuartOut => quart_out,
      Easing::QuartInOut => quad_in_out,
      Easing::QuintIn => quint_in,
      Easing::QuintOut => quint_out,
      Easing::QuintInOut => quint_in_out,
      Easing::SineIn => sine_in,
      Easing::SineOut => sine_out,
      Easing::SineInOut => sine_in_out,
      Easing::ExpoIn => expo_in,
      Easing::ExpoOut => expo_out,
      Easing::ExpoInOut => expo_in_out,
      Easing::CircIn => circ_in,
      Easing::CircOut => circ_out,
      Easing::CircInOut => circ_in_out,
      Easing::ElasticIn => elastic_in,
      Easing::ElasticOut => elastic_out,
      Easing::ElasticInOut => elastic_in_out,
      Easing::BackIn => back_in,
      Easing::BackOut => back_out,
      Easing::BackInOut => back_out,
      Easing::BounceIn => bounce_in,
      Easing::BounceOut => bounce_out,
      Easing::BounceInOut => bounce_in_out,
    }
}
