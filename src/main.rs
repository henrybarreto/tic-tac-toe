use bevy::prelude::*;

pub enum Status {
    Playing,
    Won(Mark),
    Draw
}

pub struct MarkImages {
    pub x: Handle<Image>,
    pub o: Handle<Image>,
}

pub enum Turn {
    X,
    O,
}

#[derive(Component, PartialEq, Debug, Clone, Copy)]
pub enum Mark {
    X,
    O,
    Empty,
}

#[derive(Component, Clone, Debug)]
pub struct Position {
    number: usize,
}

#[derive(Component)]
pub struct Cell {
    pub value: Mark,
    pub position: Position,
}

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
                value: Mark::Empty,
                position: Position { number: 1 },
            },
            cell2: Cell {
                value: Mark::Empty,
                position: Position { number: 2 },
            },
            cell3: Cell {
                value: Mark::Empty,
                position: Position { number: 3 },
            },
            cell4: Cell {
                value: Mark::Empty,
                position: Position { number: 4 },
            },
            cell5: Cell {
                value: Mark::Empty,
                position: Position { number: 5 },
            },
            cell6: Cell {
                value: Mark::Empty,
                position: Position { number: 6 },
            },
            cell7: Cell {
                value: Mark::Empty,
                position: Position { number: 7 },
            },
            cell8: Cell {
                value: Mark::Empty,
                position: Position { number: 8 },
            },
            cell9: Cell {
                value: Mark::Empty,
                position: Position { number: 9 },
            },
        }
    }

    fn is_full(&self) -> bool {
        self.cell1.value != Mark::Empty &&
            self.cell2.value != Mark::Empty &&
            self.cell3.value != Mark::Empty &&
            self.cell4.value != Mark::Empty &&
            self.cell5.value != Mark::Empty &&
            self.cell6.value != Mark::Empty &&
            self.cell7.value != Mark::Empty &&
            self.cell8.value != Mark::Empty &&
            self.cell9.value != Mark::Empty
    }

    pub fn winner(&self) -> Mark {
        let mut winner = Mark::Empty;

        if self.cell1.value == self.cell2.value && self.cell2.value == self.cell3.value {
            winner = self.cell1.value;
        } else if self.cell4.value == self.cell5.value && self.cell5.value == self.cell6.value {
            winner = self.cell4.value;
        } else if self.cell7.value == self.cell8.value && self.cell8.value == self.cell9.value {
            winner = self.cell7.value;
        } else if self.cell1.value == self.cell4.value && self.cell4.value == self.cell7.value {
            winner = self.cell1.value;
        } else if self.cell2.value == self.cell5.value && self.cell5.value == self.cell8.value {
            winner = self.cell2.value;
        } else if self.cell3.value == self.cell6.value && self.cell6.value == self.cell9.value {
            winner = self.cell3.value;
        } else if self.cell1.value == self.cell5.value && self.cell5.value == self.cell9.value {
            winner = self.cell1.value;
        } else if self.cell3.value == self.cell5.value && self.cell5.value == self.cell7.value {
            winner = self.cell3.value;
        }

        winner
    }
}

pub struct DrawEvent {
    pub mark: Mark,
    pub position: Position,
}

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn_bundle(Camera2dBundle::default());
    commands.spawn_bundle(SpriteBundle {
        texture: assets.load("background.png"),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands
        .spawn_bundle(SpriteBundle {
            texture: assets.load("board.png"),
            transform: Transform::from_xyz(0.0, 0.0, 1.0),
            ..default()
        })
        .insert(Board {
            cell1: Cell {
                value: Mark::Empty,
                position: Position { number: 0 },
            },
            cell2: Cell {
                value: Mark::Empty,
                position: Position { number: 1 },
            },
            cell3: Cell {
                value: Mark::Empty,
                position: Position { number: 2 },
            },
            cell4: Cell {
                value: Mark::Empty,
                position: Position { number: 3 },
            },
            cell5: Cell {
                value: Mark::Empty,
                position: Position { number: 4 },
            },
            cell6: Cell {
                value: Mark::Empty,
                position: Position { number: 5 },
            },
            cell7: Cell {
                value: Mark::Empty,
                position: Position { number: 6 },
            },
            cell8: Cell {
                value: Mark::Empty,
                position: Position { number: 7 },
            },
            cell9: Cell {
                value: Mark::Empty,
                position: Position { number: 8 },
            },
        });
    commands.insert_resource(Turn::X);
    commands.insert_resource(MarkImages {
        x: assets.load("x.png"),
        o: assets.load("o.png"),
    });
    commands.insert_resource(Status::Playing);
}

fn input(
    mut draw_event: EventWriter<DrawEvent>,
    mut turn: ResMut<Turn>,
    keyboard: Res<Input<KeyCode>>,
    status: Res<Status>,
    mut board_query: Query<&mut Board>,
) {
    if let Status::Playing = *status {
        for key in keyboard.get_just_pressed() {
            let mut board = board_query.single_mut();
            match key {
                KeyCode::Numpad1 => {
                    if let Mark::Empty = board.cell1.value {
                        board.cell1.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 1: {:?}", board.cell1.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell1.value,
                            position: Position { number: 0 },
                        });
                    }
                }
                KeyCode::Numpad2 => {
                    if let Mark::Empty = board.cell2.value {
                        board.cell2.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 2: {:?}", board.cell2.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell2.value,
                            position: Position { number: 1 },
                        });
                    }
                }
                KeyCode::Numpad3 => {
                    if let Mark::Empty = board.cell3.value {
                        board.cell3.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 3: {:?}", board.cell3.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell3.value,
                            position: Position { number: 2 },
                        });
                    }
                }
                KeyCode::Numpad4 => {
                    if let Mark::Empty = board.cell4.value {
                        board.cell4.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 4: {:?}", board.cell4.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell4.value,
                            position: Position { number: 3 },
                        });
                    }
                }
                KeyCode::Numpad5 => {
                    if let Mark::Empty = board.cell5.value {
                        board.cell5.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 5: {:?}", board.cell5.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell5.value,
                            position: Position { number: 4 },
                        });
                    }
                }
                KeyCode::Numpad6 => {
                    if let Mark::Empty = board.cell6.value {
                        board.cell6.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 6: {:?}", board.cell6.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell6.value,
                            position: Position { number: 5 },
                        });
                    }
                }
                KeyCode::Numpad7 => {
                    if let Mark::Empty = board.cell7.value {
                        board.cell7.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 7: {:?}", board.cell7.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell7.value,
                            position: Position { number: 6 },
                        });
                    }
                }
                KeyCode::Numpad8 => {
                    if let Mark::Empty = board.cell8.value {
                        board.cell8.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 8: {:?}", board.cell8.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell8.value,
                            position: Position { number: 7 },
                        });
                    }
                }
                KeyCode::Numpad9 => {
                    if let Mark::Empty = board.cell9.value {
                        board.cell9.value = match *turn {
                            Turn::X => Mark::X,
                            Turn::O => Mark::O,
                        };

                        *turn = match *turn {
                            Turn::X => Turn::O,
                            Turn::O => Turn::X,
                        };

                        info!("Cell 9: {:?}", board.cell9.value);
                        draw_event.send(DrawEvent {
                            mark: board.cell9.value,
                            position: Position { number: 8 },
                        });
                    }
                }
                _ => {}
            }
        }
    }
}

fn draw(mut commands: Commands, mut status: ResMut<Status>, mut draw_events: EventReader<DrawEvent>, mark_images: Res<MarkImages>) {
    for event in draw_events.iter() {
                let texture = match event.mark {
                    Mark::X => mark_images.x.clone(),
                    Mark::O => mark_images.o.clone(),
                    _ => panic!("Invalid mark"),
                };

                // TODO: Remove this position variable from that function.
                let positions = vec![
                    Vec3::new(-170.0, -170.0, 0.0),
                    Vec3::new(0.0, -170.0, 0.0),
                    Vec3::new(170.0, -170.0, 0.0),
                    Vec3::new(-170.0, 0.0, 0.0),
                    Vec3::new(0.0, 0.0, 0.0),
                    Vec3::new(170.0, 0.0, 0.0),
                    Vec3::new(-170.0, 170.0, 0.0),
                    Vec3::new(0.0, 170.0, 0.0),
                    Vec3::new(170.0, 170.0, 0.0),
                ];

                match event.position.number {
                    0 => {
                        info!("Drawing X at position 0");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[0].x, positions[0].y, 3.0),
                            ..Default::default()
                        });
                    }
                    1 => {
                        info!("Drawing X at position 1");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[1].x, positions[1].y, 3.0),
                            ..Default::default()
                        });
                    }
                    2 => {
                        info!("Drawing X at position 2");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[2].x, positions[2].y, 3.0),
                            ..Default::default()
                        });
                    }
                    3 => {
                        info!("Drawing X at position 3");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[3].x, positions[3].y, 3.0),
                            ..Default::default()
                        });
                    }
                    4 => {
                        info!("Drawing X at position 4");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[4].x, positions[4].y, 3.0),
                            ..Default::default()
                        });
                    }
                    5 => {
                        info!("Drawing X at position 5");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[5].x, positions[5].y, 3.0),
                            ..Default::default()
                        });
                    }
                    6 => {
                        info!("Drawing X at position 6");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[6].x, positions[6].y, 3.0),
                            ..Default::default()
                        });
                    }
                    7 => {
                        info!("Drawing X at position 7");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[7].x, positions[7].y, 3.0),
                            ..Default::default()
                        });
                    }
                    8 => {
                        info!("Drawing X at position 8");
                        commands.spawn_bundle(SpriteBundle {
                            texture,
                            transform: Transform::from_xyz(positions[8].x, positions[8].y, 3.0),
                            ..Default::default()
                        });
                    }
                    _ => {}
                }
            }
}

fn won(mut status: ResMut<Status>, mut board_query: Query<&Board>) {
    match *status {
        Status::Playing => {
            for board in board_query.iter() {
                match board.winner() {
                    Mark::X => {
                        info!("X won");

                        *status = Status::Won(Mark::X);
                    },
                    Mark::O => {
                        info!("O won");

                        *status = Status::Won(Mark::O);
                    },
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
        .insert_resource(WindowDescriptor {
            title: "Tic Tac Toe".to_string(),
            width: 600.0,
            height: 600.0,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_event::<DrawEvent>()
        .add_startup_system(setup)
        .add_system(input)
        .add_system(draw)
        .add_system(won)
        .run();
}
