use ::std::{
    any::{Any, TypeId},
    collections::HashMap,
};

use crate::ui::Id;

/// Internal State maintains any widget state that needs to persist between
/// view rebuilds.
pub struct InternalState {
    widget_states: HashMap<Id, Box<dyn Any>>,
}

impl InternalState {
    /// Create a new, empty, internal state.
    pub fn new() -> Self {
        Self {
            widget_states: HashMap::new(),
        }
    }

    /// Get the state for a Widget's id.
    /// If no state exists, a default instance will be created and inserted.
    pub fn get_state<S>(&mut self, id: &Id) -> &S
    where
        S: Default + 'static,
    {
        self.check_missing::<S>(id);

        // the unwraps are safe due to the checks up above
        self.widget_states.get(id).unwrap().downcast_ref().unwrap()
    }

    /// Get a mutable referenc to the state ofra Widget's id.
    /// If not state exists, a default instance will be created and inserted.
    pub fn get_state_mut<S>(&mut self, id: &Id) -> &mut S
    where
        S: Default + 'static,
    {
        self.check_missing::<S>(id);

        // the unwraps are safe due to the checks up above
        self.widget_states
            .get_mut(id)
            .unwrap()
            .downcast_mut()
            .unwrap()
    }

    /// Check if the state map includes any state for the given ID.
    /// If the state is missing, a default instance is inserted.
    /// If the state is present but has the wrong type, an error is logged and
    /// the state is overwritten with a default instance.
    fn check_missing<S>(&mut self, id: &Id)
    where
        S: Default + 'static,
    {
        let needs_insert = if let Some(state) = self.widget_states.get(id) {
            let wrong_type = state.as_ref().type_id() != TypeId::of::<S>();
            if wrong_type {
                log::error!(
                    "Unable to downcast existing Widget state for {:?}! \
                    Are your UI IDs unique? Expected {:?} but found {:?}",
                    id,
                    TypeId::of::<S>(),
                    state.type_id(),
                );
            }
            // The state was found, but if the types don't match then the
            // correct type needs to be inserted.
            wrong_type
        } else {
            // couldn't find a state for this id, so it'll need to be inserted
            true
        };

        if needs_insert {
            log::trace!("Creating new state for {:?}", id);
            self.widget_states.insert(*id, Box::new(S::default()));
        }
    }
}
