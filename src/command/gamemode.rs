use valence::{GameMode, app::{Plugin, Update}, client::{Client, Username}, command::{AddCommand, handler::CommandResultEvent, parsers::{EntitySelector, entity_selector::EntitySelectors}}, command_macros::Command, entity::Position, message::SendMessage, prelude::{Entity, EventReader, Query}};
use tracing::info;

pub struct GamemodeCommandPlugin;

impl Plugin for GamemodeCommandPlugin {
    fn build(&self, app: &mut valence::app::App) {
        app.add_command::<GamemodeCommand>();
        app.add_systems(Update, handle_gamemode_command);
    }
}

#[derive(Debug, Clone, Command)]
#[paths("gamemode", "gm")]
#[scopes("meloncraft.command.gamemode")]
pub enum GamemodeCommand {
    #[paths("survival {target?}")]
    Survival { target: Option<EntitySelector> },
    #[paths("creative {target?}")]
    Creative { target: Option<EntitySelector> },
    #[paths("adventure {target?}")]
    Adventure { target: Option<EntitySelector> },
    #[paths("spectator {target?}")]
    Spectator { target: Option<EntitySelector> },
}

fn handle_gamemode_command(
    mut events: EventReader<CommandResultEvent<GamemodeCommand>>,
    mut clients: Query<(&mut Client, &mut GameMode, &Username, Entity)>,
    positions: Query<&Position>,
) {
    for event in events.read() {
        let (gamemode_type, entity_selector) = match &event.result {
            GamemodeCommand::Survival { target } => (GameMode::Survival, target),
            GamemodeCommand::Creative { target } => (GameMode::Creative, target),
            GamemodeCommand::Adventure { target } => (GameMode::Adventure, target),
            GamemodeCommand::Spectator { target } => (GameMode::Spectator, target),
        };
        match entity_selector {
            None => {
                let mut game_mode = clients.get_mut(event.executor).unwrap().1;
                set_gamemode(gamemode_type, &mut game_mode);
            },
            Some(selector) => match selector {
                EntitySelector::SimpleSelector(selector) => match selector {
                    EntitySelectors::SinglePlayer(player_name) => {
                        let target = clients
                            .iter_mut()
                            .find(|(.., username, _)| username.0 == *player_name)
                            .map(|(.., target)| target);
                        match target {
                            None => {
                                let client = &mut clients.get_mut(event.executor).unwrap().0;
                                client.send_chat_message(format!("Player '{}' not found.", player_name));
                            },
                            Some(target_entity) => {
                                let mut game_mode = clients.get_mut(target_entity).unwrap().1;
                                set_gamemode(gamemode_type, &mut game_mode);
                            }
                        }

                    },
                    EntitySelectors::AllPlayers => {
                        for (_, mut game_mode, _, _) in &mut clients {
                            set_gamemode(gamemode_type, &mut game_mode);
                        }
                    },
                    EntitySelectors::AllEntities => {
                        let client = &mut clients.get_mut(event.executor).unwrap().0;
                        client.send_chat_message("You can only change the gamemode of players.");
                    },
                    EntitySelectors::SelfPlayer => {
                        let mut game_mode = clients.get_mut(event.executor).unwrap().1;
                        set_gamemode(gamemode_type, &mut game_mode);
                    },
                    EntitySelectors::NearestPlayer => {
                        info!("NearestPlayer selector is not implemented yet.");
                    },
                    EntitySelectors::RandomPlayer => {
                        info!("RandomPlayer selector is not implemented yet.");
                    },
                },
                EntitySelector::ComplexSelector(_, _) => {
                    info!("Complex selectors are not implemented yet.");
                }
            }
        }
    }
}

fn set_gamemode(
    gamemode_type: GameMode,
    gamemode_component: &mut GameMode,
) {
    *gamemode_component = gamemode_type;
}

