#![allow(dead_code)]
use crate::color::Color;

pub trait CanvasGradientInterface {

}

pub trait CanvasPatternInterface {

}

pub struct CanvasStyle<G, P> where
    G: CanvasGradientInterface,
    P: CanvasPatternInterface
{
    color: Color,
    gradient: G,
    pattern: P
}
