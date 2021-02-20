#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

/// The easing function type.
///
/// An easing function takes an input number [t] in range 0..1, inclusive, and
/// returns a non-negative value. In addition, the function must return 1 for
/// [t] = 1.

use std::f64::consts::PI;

pub type EasingFunction =  fn(t: f64) -> f64;

fn linear(t: f64) -> f64 {
    t
}

fn ease_in_quad(t: f64) -> f64 {
    t * t
}

fn ease_out_quad(t: f64) -> f64 {
    t * (2_f64 - t)
}

fn ease_in_out_quad(t: f64) -> f64 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t;
    }
    t = t - 1.0;
    0.5 * (1.0 - t * (t - 2.0))
}

fn ease_in_cubic(t: f64) -> f64 {
    t * t * t
}

fn ease_out_cubic(t: f64) -> f64 {
    let t = t - 1.0;
    t * t * t + 1.0
}

fn ease_in_out_cubic(t: f64) -> f64 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t;
    }
    t = t - 2.0;
    0.5 * (t * t * t + 2.0)
}

fn ease_in_quart(t: f64) -> f64 {
    t * t * t * t
}

fn ease_out_quart(t: f64) -> f64 {
    let t = t - 1.0;
    1.0 - t * t * t * t
}

fn ease_in_out_quart(t: f64) -> f64 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t;
    }
    t = t - 2.0;
    0.5 * (2.0 - t * t * t * t)
}

fn ease_in_quint(t: f64) -> f64 {
    t * t * t * t * t
}

fn ease_out_quint(t: f64) -> f64 {
    let t = t - 1.0;
    t * t * t * t * t + 1.0
}

fn ease_in_out_quint(t: f64) -> f64 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * t * t * t * t * t;
    }

    t = t - 2.0;
    0.5 * (t * t * t * t * t + 2.0)
}

fn ease_in_sine(t: f64) -> f64 {
    1.0 - (t * PI / 2.0).cos()
}

fn ease_out_sine(t: f64) -> f64 {
    (t * PI / 2.0).sin()
}

fn ease_in_out_sine(t: f64) -> f64 {
    0.5 * (1.0 - (PI * t).cos())
}

fn ease_in_expo(t: f64) -> f64 {
    if t == 0.0 {
        return 1.0;
    }

    f64::powf(2.0, 10.0 * (t - 1.0))
}

fn ease_out_expo(t: f64) -> f64 {
    if t == 1.0 {
        return 1.0;
    }

    1.0 - f64::powf(2.0, -10.0 * t)
}

fn ease_in_out_expo(t: f64) -> f64 {
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
        return 1.0;
    }
    let t = t * 2.0;
    if t < 1.0 {
        return 1.0 / 2.0 * f64::powf(2.0, 10.0 * (t - 1.0));
    }

    0.5 * (-f64::powf(2.0, -10.0 * --t) + 2.0)
}

fn ease_in_circ(t: f64) -> f64 {
    if t >= 1.0 {
        return t;
    }

    1.0 - (1.0 - t * t).sqrt()
}

fn ease_out_circ(t: f64) -> f64 {
    let t = t - 1.0;
    (1.0 - t * t).sqrt()
}

fn ease_in_out_circ(t: f64) -> f64 {
    let mut t = t * 2.0;
    if t < 1.0 {
        return -0.5 * ((1.0 - t * t).sqrt() - 1.0);
    }
    t = t - 2.0;
    0.5 * ((1.0 - t * t).sqrt() + 1.0)
}

fn ease_in_elastic(t: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    if t == 0.0 {
        return 0.0;
    }
    if t == 1.0 {
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

    let t = t - 1.0;
    -(a * f64::powf(2.0, 10.0 * t) * ((t - s) * (2.0 * PI) / p).sin())
}

fn ease_out_elastic(t: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    
    if t == 0.0 {
        return 0.0;
    }
    
    if t == 1.0 {
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

    a * f64::powf(2.0, -10.0 * t) * ((t - s) * (2.0 * PI) / p).sin() + 1.0
}

fn ease_in_out_elastic(t: f64) -> f64 {
    let mut p = 0.0;
    let mut a = 1.0;
    
    if t == 0.0 {
        return 0.0;
    }
    
    if t == 1.0 {
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

    let t = 2.0 * t - 1.0;
    if t < 0.0 {
        return -0.5 * (a * f64::powf(2.0, 10.0 * t) * ((t - s) * (2.0 * PI) / p).sin());
    }

    a * f64::powf(2.0, -10.0 * t) * ((t - s) * (2.0 * PI) / p).sin() * 0.5 + 1.0
}

fn ease_in_back(t: f64) -> f64 {
    let s = 1.70158;
    t * t * ((s + 1.0) * t - s)
}

fn ease_out_back(t: f64) -> f64 {
    let s = 1.70158;
    let t = t - 1.0;
    t * t * ((s + 1.0) * t + s) + 1.0
}

fn ease_in_out_back(t: f64) -> f64 {
    let s = 1.70158 * 1.525;
    let mut t = t * 2.0;
    if t < 1.0 {
        return 0.5 * (t * t * ((s + 1.0) * t - s));
    }
    t = t - 2.0;
    0.5 * (t * t * ((s + 1.0) * t + s) + 2.0)
}

fn ease_in_bounce(t: f64) -> f64 {
    1.0 - ease_out_bounce(1.0 - t)
}

fn ease_out_bounce(t: f64) -> f64 {
    let mut t = t;
    if t < 1.0 / 2.75 {
        return 7.5625 * t * t;
    } else if t < 2.0 / 2.75 {
        t -= 1.5 / 2.75;
        return 7.5625 * t * t + 0.75;
    } else if t < 2.5 / 2.75 {
        t -= 2.25 / 2.75;
        return 7.5625 * t * t + 0.9375;
    } else {
        t -= 2.625 / 2.75;
        return 7.5625 * t * t + 0.984375;
    }
}

fn ease_in_out_bounce(t: f64) -> f64 {
    if t < 0.5 {
        return ease_in_bounce(t * 2.0) * 0.5;
    }

    ease_out_bounce(t * 2.0 - 1.0) * 0.5 + 1.0 * 0.5
}

/// Returns the easing function with the given [name].
///
/// [name] can be an [EasingFunction] or a [String] specifying the name of one
/// of the easing functions defined above.
fn get_easing_function(name: &str) -> EasingFunction {
    // if (name is EasingFunction) return name;
    // switch (name) {
    //   case "linear":
    //     return linear;
    //   case "easeInQuad":
    //     return easeInQuad;
    //   case "easeOutQuad":
    //     return easeOutQuad;
    //   case "easeInOutQuad":
    //     return easeInOutQuad;
    //   case "easeInCubic":
    //     return easeInCubic;
    //   case "easeOutCubic":
    //     return easeOutCubic;
    //   case "easeInOutCubic":
    //     return easeInOutCubic;
    //   case "easeInQuart":
    //     return easeInQuart;
    //   case "easeOutQuart":
    //     return easeOutQuart;
    //   case "easeInOutQuart":
    //     return easeInOutQuart;
    //   case "easeInQuint":
    //     return easeInQuint;
    //   case "easeOutQuint":
    //     return easeOutQuint;
    //   case "easeInOutQuint":
    //     return easeInOutQuint;
    //   case "easeInSine":
    //     return easeInSine;
    //   case "easeOutSine":
    //     return easeOutSine;
    //   case "easeInOutSine":
    //     return easeInOutSine;
    //   case "easeInExpo":
    //     return easeInExpo;
    //   case "easeOutExpo":
    //     return easeOutExpo;
    //   case "easeInOutExpo":
    //     return easeInOutExpo;
    //   case "easeInCirc":
    //     return easeInCirc;
    //   case "easeOutCirc":
    //     return easeOutCirc;
    //   case "easeInOutCirc":
    //     return easeInOutCirc;
    //   case "easeInElastic":
    //     return easeInElastic;
    //   case "easeOutElastic":
    //     return easeOutElastic;
    //   case "easeInOutElastic":
    //     return easeInOutElastic;
    //   case "easeInBack":
    //     return easeInBack;
    //   case "easeOutBack":
    //     return easeOutBack;
    //   case "easeInOutBack":
    //     return easeInOutBack;
    //   case "easeInBounce":
    //     return easeInBack;
    //   case "easeOutBounce":
    //     return easeOutBounce;
    //   case "easeInOutBounce":
    //     return easeInOutBounce;
    //   default:
    //     throw ArgumentError.value(name, "name");
    // }
    unimplemented!()
}
