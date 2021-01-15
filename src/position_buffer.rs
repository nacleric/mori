#[derive(Debug, Eq, PartialEq)]
pub struct PositionBuffer<B: Bufferable, P: Positionable> {
    buffer: B,
    position: P,
}
