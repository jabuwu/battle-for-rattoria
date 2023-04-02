use std::{
    fmt::{self, Debug},
    hash::{Hash, Hasher},
    marker::PhantomData,
};

use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct SpawnSet;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub struct UpdateSet;

#[derive(Copy, SystemSet)]
pub enum EventSet<T: Send + Sync + 'static> {
    Sender,
    #[doc(hidden)]
    #[system_set(ignore_field)]
    _Data(PhantomData<T>),
}

impl<T: Send + Sync + 'static> Clone for EventSet<T> {
    fn clone(&self) -> Self {
        Self::Sender
    }
}

impl<T: Send + Sync + 'static> Debug for EventSet<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Sender => {
                f.write_str("Sender")?;
            }
            Self::_Data(..) => unreachable!(),
        }
        Ok(())
    }
}

impl<T: Send + Sync + 'static> Hash for EventSet<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self {
            Self::Sender => {
                state.write_u32(0);
            }
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> PartialEq for EventSet<T> {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Self::Sender => match other {
                Self::Sender => true,
                Self::_Data(..) => unreachable!(),
            },
            Self::_Data(..) => unreachable!(),
        }
    }
}

impl<T: Send + Sync + 'static> Eq for EventSet<T> {}

pub struct SetsPlugin;

impl Plugin for SetsPlugin {
    fn build(&self, app: &mut App) {
        let schedule = app.get_schedule_mut(CoreSchedule::FixedUpdate).unwrap();
        schedule.configure_set(UpdateSet.after(SpawnSet));
    }
}
