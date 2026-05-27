use bevy::ecs::entity::Entity;
use bevy::ecs::message::MessageWriter;
use bevy::ecs::query::{Changed, With};
use bevy::ecs::system::Query;
use meloncraft_bossbar::active::ActiveBossbars;
use meloncraft_bossbar::color::BossbarColor;
use meloncraft_bossbar::division::BossbarDivision;
use meloncraft_bossbar::flags::{BossbarCreatesFog, BossbarDarkensSky, BossbarIsDragon};
use meloncraft_bossbar::marker::BossbarMarker;
use meloncraft_bossbar::title::BossbarTitle;
use meloncraft_entity::Uuid;
use meloncraft_entity::health::current::CurrentHealth;
use meloncraft_packets::ClientboundBossEvent;

#[expect(clippy::type_complexity, reason = "Simplest way to do a query")]
pub fn send_bossbar(
    active_bossbars_q: Query<(Entity, &ActiveBossbars), Changed<ActiveBossbars>>,
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
    for (player_entity, active_bossbars) in active_bossbars_q {
        for bossbar_entity in active_bossbars.0.clone() {
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
    }
}
