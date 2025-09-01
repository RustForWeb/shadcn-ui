//! Leptos port of shadcn/ui slider

pub mod default;
pub mod new_york;

pub use default::{
    Slider, RangeSlider, SliderRoot, SliderVariant, SliderSize
};
pub use new_york::{
    Slider as SliderNewYork, RangeSlider as RangeSliderNewYork, 
    SliderRoot as SliderRootNewYork, SliderVariant as SliderVariantNewYork,
    SliderSize as SliderSizeNewYork
};

#[cfg(test)]
mod tests;
