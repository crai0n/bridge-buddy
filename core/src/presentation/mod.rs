use crate::primitives::game_event::GameEvent;

pub trait PresentEvent {
    fn present_event(&self, event: GameEvent);
}
