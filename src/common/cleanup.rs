use bevy::prelude::*;

#[derive(Component)]
pub struct Persistent;

pub fn cleanup_non_persistent_entities(
    mut commands: Commands,
    entity_query: Query<Entity, (Without<Parent>, Without<Persistent>, Without<Window>)>,
) {
    for entity in entity_query.iter() {
        if let Some(entity) = commands.get_entity(entity) {
            entity.despawn_recursive();
        }
    }
}
