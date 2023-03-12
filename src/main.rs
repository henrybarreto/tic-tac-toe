use bevy::prelude::*;

#[derive(Resource)]
pub struct GameAssets {
    pub logo: Handle<Image>,
    pub copy: Handle<Image>,
    pub start: Handle<Image>,
    pub background: Handle<Image>,
    pub x: Handle<Image>,
    pub o: Handle<Image>,
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub enum GameStatus {
    Menu,
    Playing,
}

pub enum GameStatusEvent {
    Menu,
    Playing,
}

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct Sprite;

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.insert_resource(GameAssets {
        logo: assets.load("sprites/logo.png"),
        copy: assets.load("sprites/copyleft.png"),
        start: assets.load("sprites/start.png"),
        background: assets.load("sprites/background.png"),
        x: assets.load("sprites/x.png"),
        o: assets.load("sprites/o.png"),
    });
}

fn menu_setup(mut commands: Commands, game_assets: Res<GameAssets>) {
    commands
        .spawn(SpriteBundle {
            texture: game_assets.background.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Sprite);
    commands
        .spawn(SpriteBundle {
            texture: game_assets.logo.clone(),
            transform: Transform::from_xyz(0.0, 200.0, 1.0),
            ..default()
        })
        .insert(Sprite);
    commands
        .spawn(SpriteBundle {
            texture: game_assets.copy.clone(),
            transform: Transform::from_xyz(0.0, -200.0, 1.0),
            ..default()
        })
        .insert(Sprite);
    commands
        .spawn(SpriteBundle {
            texture: game_assets.start.clone(),
            transform: Transform::from_xyz(0.0, -100.0, 1.0),
            ..default()
        })
        .insert(PlayButton)
        .insert(Sprite);
}

fn menu_button(
    mouse: Res<Input<MouseButton>>,
    mut event: EventWriter<GameStatusEvent>,
    windows: Res<Windows>,
    button_query: Query<(&PlayButton, &Transform)>,
    camera_query: Query<(&Camera, &GlobalTransform)>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(position) = window.cursor_position() {
        let (camera, global_transform) = camera_query.single();
        let (_, transform) = button_query.single();
        let ray = camera.viewport_to_world(global_transform, position);
        if let Some(ray) = ray {
            if ray.origin.x > transform.translation.x - 100.0
                && ray.origin.x < transform.translation.x + 100.0
            {
                if ray.origin.y > transform.translation.y - 50.0
                    && ray.origin.y < transform.translation.y + 50.0
                {
                    if mouse.just_pressed(MouseButton::Left) {
                        event.send(GameStatusEvent::Playing);
                    }
                }
            }
        }
    }
}

fn menu_input(mut event: EventWriter<GameStatusEvent>, keyboard: Res<Input<KeyCode>>) {
    if keyboard.just_pressed(KeyCode::Space) {
        event.send(GameStatusEvent::Playing);
    }
}

fn menu_cleanup(
    mut commands: Commands,
    mut event: EventReader<GameStatusEvent>,
    mut query: Query<Entity, With<Sprite>>,
) {
    for _ in event.iter() {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
            println!("Despawning entity: {:?}", entity);
        }
    }
}

#[derive(Resource, Debug, Clone, Eq, PartialEq, Hash)]
pub enum PlayingStatus {
    Playing,
    Won(Mark),
    Draw,
}

#[derive(Resource, Debug)]
pub enum Turn {
    X,
    O,
}

impl Turn {
    fn next(&mut self) {
        *self = match self {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        }
    }
}

#[derive(Component, Eq, PartialEq, Debug, Clone, Copy, Hash)]
pub enum Mark {
    X,
    O,
    Empty,
}

impl Mark {
    fn set_from_turn(&mut self, turn: &Turn) {
        *self = match turn {
            Turn::X => Mark::X,
            Turn::O => Mark::O,
        }
    }
}

#[derive(Component, Debug, Clone, Copy)]
pub struct Position {
    number: usize,
}

#[derive(Component)]
pub struct Cell {
    pub mark: Mark,
    pub position: Position,
}

// NOTICE: positions based on `board.png` sprite.
pub const BOARD_POSITIONS: [Vec2; 9] = [
    Vec2::new(-170.0, -170.0),
    Vec2::new(0.0, -170.0),
    Vec2::new(170.0, -170.0),
    Vec2::new(-170.0, 0.0),
    Vec2::new(0.0, 0.0),
    Vec2::new(170.0, 0.0),
    Vec2::new(-170.0, 170.0),
    Vec2::new(0.0, 170.0),
    Vec2::new(170.0, 170.0),
];

#[derive(Component)]
pub struct Board {
    pub cell1: Cell,
    pub cell2: Cell,
    pub cell3: Cell,
    pub cell4: Cell,
    pub cell5: Cell,
    pub cell6: Cell,
    pub cell7: Cell,
    pub cell8: Cell,
    pub cell9: Cell,
}

impl Board {
    pub fn new() -> Self {
        Board {
            cell1: Cell {
                mark: Mark::Empty,
                position: Position { number: 1 },
            },
            cell2: Cell {
                mark: Mark::Empty,
                position: Position { number: 2 },
            },
            cell3: Cell {
                mark: Mark::Empty,
                position: Position { number: 3 },
            },
            cell4: Cell {
                mark: Mark::Empty,
                position: Position { number: 4 },
            },
            cell5: Cell {
                mark: Mark::Empty,
                position: Position { number: 5 },
            },
            cell6: Cell {
                mark: Mark::Empty,
                position: Position { number: 6 },
            },
            cell7: Cell {
                mark: Mark::Empty,
                position: Position { number: 7 },
            },
            cell8: Cell {
                mark: Mark::Empty,
                position: Position { number: 8 },
            },
            cell9: Cell {
                mark: Mark::Empty,
                position: Position { number: 9 },
            },
        }
    }

    fn is_full(&self) -> bool {
        self.cell1.mark != Mark::Empty
            && self.cell2.mark != Mark::Empty
            && self.cell3.mark != Mark::Empty
            && self.cell4.mark != Mark::Empty
            && self.cell5.mark != Mark::Empty
            && self.cell6.mark != Mark::Empty
            && self.cell7.mark != Mark::Empty
            && self.cell8.mark != Mark::Empty
            && self.cell9.mark != Mark::Empty
    }

    pub fn winner(&self) -> Mark {
        let mut winner = Mark::Empty;

        if self.cell1.mark == self.cell2.mark && self.cell2.mark == self.cell3.mark {
            winner = self.cell1.mark;
        } else if self.cell4.mark == self.cell5.mark && self.cell5.mark == self.cell6.mark {
            winner = self.cell4.mark;
        } else if self.cell7.mark == self.cell8.mark && self.cell8.mark == self.cell9.mark {
            winner = self.cell7.mark;
        } else if self.cell1.mark == self.cell4.mark && self.cell4.mark == self.cell7.mark {
            winner = self.cell1.mark;
        } else if self.cell2.mark == self.cell5.mark && self.cell5.mark == self.cell8.mark {
            winner = self.cell2.mark;
        } else if self.cell3.mark == self.cell6.mark && self.cell6.mark == self.cell9.mark {
            winner = self.cell3.mark;
        } else if self.cell1.mark == self.cell5.mark && self.cell5.mark == self.cell9.mark {
            winner = self.cell1.mark;
        } else if self.cell3.mark == self.cell5.mark && self.cell5.mark == self.cell7.mark {
            winner = self.cell3.mark;
        }

        winner
    }
}

#[derive(Resource)]
pub struct TTF {
    pub handler: Handle<Font>,
}

pub struct MarkEvent {
    pub mark: Mark,
    pub position: Position,
}

fn board_setup(mut commands: Commands, assets: Res<AssetServer>) {
    //commands.spawn(Camera2dBundle::default());
    commands
        .spawn(SpriteBundle {
            texture: assets.load("sprites/background.png"),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        })
        .insert(Sprite);

    commands
        .spawn((
            Board::new(),
            SpriteBundle {
                texture: assets.load("sprites/board.png"),
                transform: Transform::from_xyz(0.0, 0.0, 1.0),
                ..default()
            },
        ))
        .insert(Sprite);

    commands
        .spawn(Text2dBundle {
            text: Text::from_section(
                "X",
                TextStyle {
                    font: assets.load("fonts/FiraSans-Bold.ttf"),
                    font_size: 40.0,
                    color: Color::WHITE,
                },
            )
            .with_alignment(TextAlignment::CENTER),
            transform: Transform::from_translation(Vec3::new(0.0, 300.0, 1.0)),
            ..Default::default()
        })
        .insert(Sprite);

    commands.insert_resource(PlayingStatus::Playing);
    commands.insert_resource(Turn::X);
}

fn board_cleanup(
    mut commands: Commands,
    mut event: EventReader<GameStatusEvent>,
    mut playing_status: ResMut<PlayingStatus>,
    mut turn: ResMut<Turn>,
    mut query: Query<Entity, With<Sprite>>,
) {
    for _ in event.iter() {
        for entity in query.iter_mut() {
            commands.entity(entity).despawn();
            println!("Despawning entity: {:?}", entity);
        }
    }

    *playing_status = PlayingStatus::Playing;
    *turn = Turn::X;
}

fn input(
    mut mark_cell_event: EventWriter<MarkEvent>,
    keyboard: Res<Input<KeyCode>>,
    status: Res<PlayingStatus>,
    mut turn: ResMut<Turn>,
    mut game_status: EventWriter<GameStatusEvent>,
    mut query: Query<&mut Board>,
) {
    if keyboard.just_pressed(KeyCode::Escape) {
        game_status.send(GameStatusEvent::Menu);
    }

    if let PlayingStatus::Playing = *status {
        for key in keyboard.get_just_pressed() {
            let mut board = query.single_mut();
            match key {
                KeyCode::Numpad1 => {
                    if let Mark::Empty = board.cell1.mark {
                        board.cell1.mark.set_from_turn(&turn);

                        info!("Cell 0: {:?}", turn);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell1.mark,
                            position: Position { number: 0 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad2 => {
                    if let Mark::Empty = board.cell2.mark {
                        board.cell2.mark.set_from_turn(&turn);

                        info!("Cell 1: {:?}", board.cell2.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell2.mark,
                            position: Position { number: 1 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad3 => {
                    if let Mark::Empty = board.cell3.mark {
                        board.cell3.mark.set_from_turn(&turn);

                        info!("Cell 2: {:?}", board.cell3.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell3.mark,
                            position: Position { number: 2 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad4 => {
                    if let Mark::Empty = board.cell4.mark {
                        board.cell4.mark.set_from_turn(&turn);

                        info!("Cell 3: {:?}", board.cell4.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell4.mark,
                            position: Position { number: 3 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad5 => {
                    if let Mark::Empty = board.cell5.mark {
                        board.cell5.mark.set_from_turn(&turn);

                        info!("Cell 4: {:?}", board.cell5.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell5.mark,
                            position: Position { number: 4 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad6 => {
                    if let Mark::Empty = board.cell6.mark {
                        board.cell6.mark.set_from_turn(&turn);

                        info!("Cell 5: {:?}", board.cell6.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell6.mark,
                            position: Position { number: 5 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad7 => {
                    if let Mark::Empty = board.cell7.mark {
                        board.cell7.mark.set_from_turn(&turn);

                        info!("Cell 6: {:?}", board.cell7.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell7.mark,
                            position: Position { number: 6 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad8 => {
                    if let Mark::Empty = board.cell8.mark {
                        board.cell8.mark.set_from_turn(&turn);

                        info!("Cell 7: {:?}", board.cell8.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell8.mark,
                            position: Position { number: 7 },
                        });

                        turn.next();
                    }
                }
                KeyCode::Numpad9 => {
                    if let Mark::Empty = board.cell9.mark {
                        board.cell9.mark.set_from_turn(&turn);

                        info!("Cell 8: {:?}", board.cell9.mark);
                        mark_cell_event.send(MarkEvent {
                            mark: board.cell9.mark,
                            position: Position { number: 8 },
                        });

                        turn.next();
                    }
                }
                _ => {}
            }
        }
    }
}

fn mark(
    mut commands: Commands,
    mut draw_cell_events: EventReader<MarkEvent>,
    game_assets: Res<GameAssets>,
) {
    for event in draw_cell_events.iter() {
        info!("drawing on at position {:#?}", event.position.number);
        commands
            .spawn(SpriteBundle {
                texture: match event.mark {
                    Mark::X => game_assets.x.clone(),
                    Mark::O => game_assets.o.clone(),
                    _ => panic!("invalid mark"),
                },
                transform: Transform::from_xyz(
                    BOARD_POSITIONS[event.position.number].x,
                    BOARD_POSITIONS[event.position.number].y,
                    3.0,
                ),
                ..Default::default()
            })
            .insert(Sprite);
    }
}

fn won(mut status: ResMut<PlayingStatus>, query: Query<&Board>) {
    match *status {
        PlayingStatus::Playing => {
            let board = query.single();
            match board.winner() {
                Mark::X => {
                    info!("X won");

                    *status = PlayingStatus::Won(Mark::X);
                }
                Mark::O => {
                    info!("O won");

                    *status = PlayingStatus::Won(Mark::O);
                }
                Mark::Empty => {
                    if board.is_full() {
                        info!("Draw");

                        *status = PlayingStatus::Draw;
                    }
                }
            }
        }
        PlayingStatus::Won(_) => {}
        PlayingStatus::Draw => {}
    }
}

fn ui(
    status: Res<PlayingStatus>,
    turn: Res<Turn>,
    mut mark_event: EventReader<MarkEvent>,
    mut query: Query<&mut Text>,
) {
    match *status {
        PlayingStatus::Playing => {
            for _ in mark_event.iter() {
                info!("informing UI for {:#?}", *turn);
                let mut text = query.single_mut();
                text.sections[0].value = format!("{:?}", *turn);
            }
        }
        PlayingStatus::Won(mark) => {
            let mut text = query.single_mut();
            text.sections[0].value = format!("{:?} won", mark);
        }
        PlayingStatus::Draw => {
            let mut text = query.single_mut();
            text.sections[0].value = "Draw".to_string();
        }
    }
}

fn manager(mut state: ResMut<State<GameStatus>>, mut events: EventReader<GameStatusEvent>) {
    for event in events.iter() {
        match event {
            GameStatusEvent::Menu => {
                state.set(GameStatus::Menu).unwrap();
            }
            GameStatusEvent::Playing => {
                state.set(GameStatus::Playing).unwrap();
            }
        }
    }
}

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins
                .set(AssetPlugin {
                    watch_for_changes: true,
                    ..default()
                })
                .set(WindowPlugin {
                    window: WindowDescriptor {
                        title: "Tic Tac Toe".to_string(),
                        width: 700.0,
                        height: 700.0,
                        resizable: false,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_event::<GameStatusEvent>()
        .add_startup_system(setup)
        .add_system(manager)
        .add_state(GameStatus::Menu)
        .add_system_set(SystemSet::on_enter(GameStatus::Menu).with_system(menu_setup))
        .add_system_set(
            SystemSet::on_update(GameStatus::Menu)
                .with_system(menu_button)
                .with_system(menu_input),
        )
        .add_system_set(SystemSet::on_exit(GameStatus::Menu).with_system(menu_cleanup))
        .add_system_set(SystemSet::on_enter(GameStatus::Playing).with_system(board_setup))
        .add_event::<MarkEvent>()
        .add_system_set(
            SystemSet::on_update(GameStatus::Playing)
                .with_system(input)
                .with_system(mark)
                .with_system(won)
                .with_system(ui),
        )
        .add_system_set(SystemSet::on_exit(GameStatus::Playing).with_system(board_cleanup))
        .run();
}
