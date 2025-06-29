use winit::dpi::PhysicalPosition;

#[derive(Debug, Default)]
pub(in crate::app) struct State {
    pub cursor_position: PhysicalPosition<f64>,
}
