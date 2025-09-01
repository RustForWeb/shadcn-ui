//! Leptos port of shadcn/ui carousel

pub mod default;
pub mod new_york;

pub use default::{
    Carousel, CarouselContent, CarouselItem, CarouselPrevious, CarouselNext,
    CarouselOrientation, CarouselApi,
};

pub use new_york::{
    Carousel as CarouselNewYork,
    CarouselContent as CarouselContentNewYork,
    CarouselItem as CarouselItemNewYork,
    CarouselPrevious as CarouselPreviousNewYork,
    CarouselNext as CarouselNextNewYork,
    CarouselOrientation as CarouselOrientationNewYork,
    CarouselApi as CarouselApiNewYork,
};

#[cfg(test)]
mod tests;