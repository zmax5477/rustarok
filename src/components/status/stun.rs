use crate::components::char::{
    ActionPlayMode, CharActionIndex, CharState, CharacterStateComponent,
    SpriteRenderDescriptorComponent,
};
use crate::components::controller::CharEntityId;
use crate::components::status::status::{
    Status, StatusNature, StatusStackingResult, StatusUpdateResult,
};
use crate::components::SoundEffectComponent;
use crate::runtime_assets::map::PhysicEngine;
use crate::systems::render::render_command::RenderCommandCollector;
use crate::systems::render_sys::render_action;
use crate::systems::SystemVariables;
use crate::ElapsedTime;
use specs::{Entities, LazyUpdate};

#[derive(Clone)]
pub struct StunStatus {
    pub caster_entity_id: CharEntityId,
    pub started: ElapsedTime,
    pub until: ElapsedTime,
}

impl StunStatus {
    pub fn new(caster_entity_id: CharEntityId, now: ElapsedTime, duration: f32) -> StunStatus {
        StunStatus {
            caster_entity_id,
            started: now,
            until: now.add_seconds(duration),
        }
    }
}

impl Status for StunStatus {
    fn dupl(&self) -> Box<dyn Status + Send> {
        Box::new(self.clone())
    }

    fn on_apply(
        &mut self,
        self_entity_id: CharEntityId,
        target_char: &mut CharacterStateComponent,
        entities: &Entities,
        updater: &mut specs::Write<LazyUpdate>,
        system_vars: &SystemVariables,
    ) {
        target_char.set_state(CharState::StandBy, target_char.dir());
        let entity = entities.create();
        updater.insert(
            entity,
            SoundEffectComponent {
                target_entity_id: self_entity_id,
                sound_id: system_vars.assets.sounds.stun,
                pos: target_char.pos(),
                start_time: system_vars.time,
            },
        );
    }

    fn can_target_move(&self) -> bool {
        false
    }

    fn can_target_be_controlled(&self) -> bool {
        true
    }

    fn can_target_cast(&self) -> bool {
        false
    }

    fn update(
        &mut self,
        self_char_id: CharEntityId,
        _char_state: &CharacterStateComponent,
        _physics_world: &mut PhysicEngine,
        system_vars: &mut SystemVariables,
        _entities: &specs::Entities,
        _updater: &mut specs::Write<LazyUpdate>,
    ) -> StatusUpdateResult {
        if self.until.has_already_passed(system_vars.time) {
            StatusUpdateResult::RemoveIt
        } else {
            StatusUpdateResult::KeepIt
        }
    }

    fn render(
        &self,
        char_state: &CharacterStateComponent,
        system_vars: &SystemVariables,
        render_commands: &mut RenderCommandCollector,
    ) {
        let anim = SpriteRenderDescriptorComponent {
            action_index: CharActionIndex::Idle as usize,
            animation_started: self.started,
            animation_ends_at: ElapsedTime(0.0),
            forced_duration: None,
            direction: 0,
            fps_multiplier: 1.0,
        };
        render_action(
            system_vars.time,
            &anim,
            &system_vars.assets.sprites.stun,
            &char_state.pos(),
            [0, -100],
            false,
            1.0,
            ActionPlayMode::Repeat,
            &[255, 255, 255, 255],
            render_commands,
        );
    }

    fn get_status_completion_percent(&self, now: ElapsedTime) -> Option<(ElapsedTime, f32)> {
        Some((self.until, now.percentage_between(self.started, self.until)))
    }

    fn stack(&self, _other: &Box<dyn Status>) -> StatusStackingResult {
        StatusStackingResult::Replace
    }

    fn typ(&self) -> StatusNature {
        StatusNature::Harmful
    }
}