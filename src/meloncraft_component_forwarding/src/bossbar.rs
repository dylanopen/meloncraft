use bevy::ecs::change_detection::DetectChanges as _;
use bevy::ecs::component::Component;
use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::{Commands, Query};
use bevy::ecs::world::Ref;
use meloncraft_bossbar::active::ActiveBossbars;
use meloncraft_bossbar::color::BossbarColor;
use meloncraft_bossbar::division::BossbarDivision;
use meloncraft_bossbar::flags::{BossbarCreatesFog, BossbarDarkensSky, BossbarIsDragon};
use meloncraft_bossbar::marker::BossbarMarker;
use meloncraft_bossbar::title::BossbarTitle;
use meloncraft_entity::Uuid;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_packets::{BossEventAction, ClientboundBossEvent};

#[derive(Component, Debug, Clone)]
pub struct PreviousActiveBossbars(pub ActiveBossbars);

#[expect(clippy::type_complexity, reason = "Simplest way to do a query")]
pub fn send_bossbar_on_change_active(
    mut commands: Commands,
    active_bossbars_q: Query<
        (Entity, &ActiveBossbars, Option<&mut PreviousActiveBossbars>),
        Changed<ActiveBossbars>,
    >,
    bossbar_q: Query<
        (
            &Uuid,
            &BossbarTitle,
            &CurrentHealth,
            &BossbarColor,
            &BossbarDivision,
            &BossbarDarkensSky,
            &BossbarIsDragon,
            &BossbarCreatesFog,
        ),
        With<BossbarMarker>,
    >,
    mut boss_event_pw: MessageWriter<ClientboundBossEvent>,
) {
    for (player_entity, active_bossbars, previous_active_bossbars) in active_bossbars_q {
        for bossbar_entity in active_bossbars.0.clone() {
            if let Some(previous_active_bossbars) = &previous_active_bossbars
                && previous_active_bossbars.0.0.contains(&bossbar_entity)
            {
                continue; // don't add it if they already had the bossbar before
            }
            let (uuid, title, health, color, division, darkens_sky, is_dragon, creates_fog) =
                bossbar_q.get(bossbar_entity).unwrap();
            boss_event_pw.write(ClientboundBossEvent {
                client: player_entity,
                uuid: uuid.clone(),
                action: meloncraft_packets::BossEventAction::Add {
                    title: title.clone(),
                    health: *health,
                    color: color.clone(),
                    division: division.clone(),
                    darkens_sky: darkens_sky.clone(),
                    is_dragon: is_dragon.clone(),
                    creates_fog: creates_fog.clone(),
                },
            });
        }
        if let Some(mut previous_active_bossbars) = previous_active_bossbars {
            for previous_active_bossbar in previous_active_bossbars.0.0.clone() {
                if !active_bossbars.0.contains(&previous_active_bossbar) {
                    let uuid = bossbar_q.get(previous_active_bossbar).unwrap().0;
                    boss_event_pw.write(ClientboundBossEvent {
                        client: player_entity,
                        uuid: uuid.clone(),
                        action: BossEventAction::Remove,
                    });
                }
            }
            previous_active_bossbars.0 = active_bossbars.clone();
        } else {
            commands
                .entity(player_entity)
                .insert(PreviousActiveBossbars(active_bossbars.clone()));
        }
    }
}

#[expect(clippy::type_complexity, reason = "Simplest way to do a query")]
pub fn send_bossbar_update_on_change_health(
    bossbar_q: Query<
        (
            Entity,
            &Uuid,
            &CurrentHealth,
            &BossbarMarker,
            Ref<BossbarMarker>,
        ),
        Changed<CurrentHealth>,
    >,
    active_bossbars_q: Query<(Entity, &ActiveBossbars)>,
    mut boss_event_pw: MessageWriter<ClientboundBossEvent>,
) {
    for (bossbar_entity, uuid, health, _, _) in bossbar_q.iter().filter(|r| return !r.4.is_added())
    {
        for (player_entity, active_bossbars) in active_bossbars_q {
            if !active_bossbars.0.contains(&bossbar_entity) {
                continue;
            }
            boss_event_pw.write(ClientboundBossEvent {
                client: player_entity,
                uuid: uuid.clone(),
                action: BossEventAction::UpdateHealth {
                    new_health: *health,
                },
            });
        }
    }
}

#[expect(clippy::type_complexity, reason = "Simplest way to do a query")]
pub fn send_bossbar_update_on_change_title(
    bossbar_q: Query<
        (
            Entity,
            &Uuid,
            &BossbarTitle,
            &BossbarMarker,
            Ref<BossbarMarker>,
        ),
        Changed<BossbarTitle>,
    >,
    active_bossbars_q: Query<(Entity, &ActiveBossbars)>,
    mut boss_event_pw: MessageWriter<ClientboundBossEvent>,
) {
    for (bossbar_entity, uuid, title, _, _) in bossbar_q.iter().filter(|r| return !r.4.is_added()) {
        for (player_entity, active_bossbars) in active_bossbars_q {
            if !active_bossbars.0.contains(&bossbar_entity) {
                continue;
            }
            boss_event_pw.write(ClientboundBossEvent {
                client: player_entity,
                uuid: uuid.clone(),
                action: BossEventAction::UpdateTitle {
                    new_title: title.clone(),
                },
            });
        }
    }
}

#[expect(clippy::type_complexity, reason = "Simplest way to do a query")]
pub fn send_bossbar_update_on_change_style(
    bossbar_q: Query<
        (
            Entity,
            &Uuid,
            &BossbarColor,
            &BossbarDivision,
            &BossbarMarker,
            Ref<BossbarMarker>,
        ),
        Changed<BossbarDivision>,
    >,
    active_bossbars_q: Query<(Entity, &ActiveBossbars)>,
    mut boss_event_pw: MessageWriter<ClientboundBossEvent>,
) {
    for (bossbar_entity, uuid, color, division, _, _) in
        bossbar_q.iter().filter(|r| return !r.5.is_added())
    {
        for (player_entity, active_bossbars) in active_bossbars_q {
            if !active_bossbars.0.contains(&bossbar_entity) {
                continue;
            }
            boss_event_pw.write(ClientboundBossEvent {
                client: player_entity,
                uuid: uuid.clone(),
                action: BossEventAction::UpdateStyle {
                    new_color: color.clone(),
                    new_division: division.clone(),
                },
            });
        }
    }
}
