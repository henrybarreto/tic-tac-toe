use bevy::prelude::*;

#[derive(Resource)]
pub struct Marks {
    pub x: Handle<Image>,
    pub o: Handle<Image>,
}

#[derive(Resource)]
pub enum Status {
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

#[derive(Component, PartialEq, Debug, Clone, Copy)]
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

pub struct DrawCellEvent {
    pub mark: Mark,
    pub position: Position,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());
    commands.spawn(SpriteBundle {
        texture: assets.load("background.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn((
        Board::new(),
        SpriteBundle {
            texture: assets.load("board.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        },
    ));

    commands.insert_resource(Status::Playing);
    commands.insert_resource(Turn::X);
    commands.insert_resource(Marks {
        x: assets.load("x.png"),
        o: assets.load("o.png"),
    });
}

fn input(
    mut draw_cell_event: EventWriter<DrawCellEvent>,
    mut turn: ResMut<Turn>,
    keyboard: Res<Input<KeyCode>>,
    status: Res<Status>,
    mut query: Query<&mut Board>,
) {
    if let Status::Playing = *status {
        for key in keyboard.get_just_pressed() {
            let mut board = query.single_mut();
            match key {
                KeyCode::Numpad1 => {
                    if let Mark::Empty = board.cell1.mark {
                        board.cell1.mark.set_from_turn(&turn);

                        info!("Cell 0: {:?}", turn);
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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
                        draw_cell_event.send(DrawCellEvent {
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

fn draw(
    mut commands: Commands,
    mut draw_events: EventReader<DrawCellEvent>,
    mark_images: Res<Marks>,
) {
    for event in draw_events.iter() {
        info!("Drawing on at position {:#?}", event.position.number);
        commands.spawn(SpriteBundle {
            texture: match event.mark {
                Mark::X => mark_images.x.clone(),
                Mark::O => mark_images.o.clone(),
                _ => panic!("Invalid mark"),
            },
            transform: Transform::from_xyz(
                BOARD_POSITIONS[event.position.number].x,
                BOARD_POSITIONS[event.position.number].y,
                3.0,
            ),
            ..Default::default()
        });
    }
}

fn won(mut status: ResMut<Status>, query: Query<&Board>) {
    match *status {
        Status::Playing => {
            for board in query.iter() {
                match board.winner() {
                    Mark::X => {
                        info!("X won");

                        *status = Status::Won(Mark::X);
                    }
                    Mark::O => {
                        info!("O won");

                        *status = Status::Won(Mark::O);
                    }
                    Mark::Empty => {
                        if board.is_full() {
                            info!("Draw");

                            *status = Status::Draw;
                        }
                    }
                }
            }
        }
        Status::Won(_) => {}
        Status::Draw => {}
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
                        width: 600.0,
                        height: 600.0,
                        ..default()
                    },
                    ..default()
                }),
        )
        .add_event::<DrawCellEvent>()
        .add_startup_system(setup)
        .add_system(input)
        .add_system(draw)
        .add_system(won)
        .run();
}
