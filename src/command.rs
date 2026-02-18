use crate::state;

pub enum StateTransition {
    NoTransition,
    PushState(Box<dyn state::State>),
    SwitchState(Box<dyn state::State>),
    PopState,
    Quit,
}