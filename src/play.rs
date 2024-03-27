use crate::*;

pub trait Play {
    fn play(&self, receiver: &std::sync::mpsc::Receiver<String>, interface: &mut Interface);
}
