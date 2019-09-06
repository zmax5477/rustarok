use crate::components::char::CharAttributeModifierCollector;
use crate::components::controller::{CharEntityId, WorldCoords};
use crate::components::status::status::{
    ApplyStatusComponentPayload, ApplyStatusInAreaComponent, Status, StatusStackingResult,
    StatusType, StatusUpdateResult,
};
use crate::components::{ApplyForceComponent, AreaAttackComponent, AttackType, StrEffectComponent};
use crate::systems::atk_calc::AttackOutcome;
use crate::systems::render::render_command::RenderCommandCollectorComponent;
use crate::systems::render_sys::RenderDesktopClientSystem;
use crate::systems::SystemVariables;
use crate::{ElapsedTime, StrEffectType};
use nalgebra::Isometry2;
use specs::LazyUpdate;

#[derive(Clone)]
pub struct FireBombStatus {
    pub caster_entity_id: CharEntityId,
    pub started: ElapsedTime,
    pub until: ElapsedTime,
}

impl Status for FireBombStatus {
    fn dupl(&self) -> Box<dyn Status> {
        Box::new(self.clone())
    }

    fn can_target_move(&self) -> bool {
        true
    }

    fn typ(&self) -> StatusType {
        StatusType::Harmful
    }

    fn can_target_cast(&self) -> bool {
        true
    }

    fn get_render_color(&self, _now: ElapsedTime) -> [u8; 4] {
        [255, 255, 255, 255]
    }

    fn get_render_size(&self) -> f32 {
        1.0
    }

    fn calc_attribs(&self, _modifiers: &mut CharAttributeModifierCollector) {}

    fn update(
        &mut self,
        self_char_id: CharEntityId,
        char_pos: &WorldCoords,
        system_vars: &mut SystemVariables,
        entities: &specs::Entities,
        updater: &mut specs::Write<LazyUpdate>,
    ) -> StatusUpdateResult {
        if self.until.is_earlier_than(system_vars.time) {
            let area_shape = Box::new(ncollide2d::shape::Ball::new(2.0));
            let area_isom = Isometry2::new(*char_pos, 0.0);
            system_vars.area_attacks.push(AreaAttackComponent {
                area_shape: area_shape.clone(),
                area_isom: area_isom.clone(),
                source_entity_id: self.caster_entity_id,
                typ: AttackType::SpellDamage(200),
            });
            system_vars
                .apply_area_statuses
                .push(ApplyStatusInAreaComponent {
                    source_entity_id: self.caster_entity_id,
                    status: ApplyStatusComponentPayload::from_secondary(Box::new(FireBombStatus {
                        caster_entity_id: self.caster_entity_id,
                        started: system_vars.time,
                        until: system_vars.time.add_seconds(2.0),
                    })),
                    area_shape: area_shape.clone(),
                    area_isom: area_isom.clone(),
                    except: Some(self_char_id),
                });
            let effect_comp = StrEffectComponent {
                effect_type: StrEffectType::FirePillarBomb,
                pos: *char_pos,
                start_time: system_vars.time.add_seconds(-0.5),
                die_at: system_vars.time.add_seconds(1.0),
            };
            updater.insert(entities.create(), effect_comp);

            StatusUpdateResult::RemoveIt
        } else {
            StatusUpdateResult::KeepIt
        }
    }

    fn affect_incoming_damage(&mut self, outcome: AttackOutcome) -> AttackOutcome {
        outcome
    }

    fn allow_push(&mut self, _push: &ApplyForceComponent) -> bool {
        true
    }

    fn render(
        &self,
        char_pos: &WorldCoords,
        system_vars: &SystemVariables,
        render_commands: &mut RenderCommandCollectorComponent,
    ) {
        RenderDesktopClientSystem::render_str(
            StrEffectType::FireWall,
            self.started,
            char_pos,
            system_vars,
            render_commands,
        );
    }

    fn get_status_completion_percent(&self, now: ElapsedTime) -> Option<(ElapsedTime, f32)> {
        Some((self.until, now.percentage_between(self.started, self.until)))
    }

    fn stack(&mut self, _other: Box<dyn Status>) -> StatusStackingResult {
        StatusStackingResult::AddTheNewStatus
    }
}
