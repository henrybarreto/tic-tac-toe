use bevy::prelude::*;

pub struct Sprints {
    pub x: Handle<Image>,
    pub o: Handle<Image>,
}

pub enum Turn {
    X,
    O,
}

#[derive(Component)]
pub enum Mark {
    X,
    O,
    Empty,
}

#[derive(Component)]
pub struct Cell {
    pub value: Mark,
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
            cell1: Cell { value: Mark::Empty },
            cell2: Cell { value: Mark::Empty },
            cell3: Cell { value: Mark::Empty },
            cell4: Cell { value: Mark::Empty },
            cell5: Cell { value: Mark::Empty },
            cell6: Cell { value: Mark::Empty },
            cell7: Cell { value: Mark::Empty },
            cell8: Cell { value: Mark::Empty },
            cell9: Cell { value: Mark::Empty },
        });
    commands.insert_resource(Turn::X);
    commands.insert_resource(Sprints {
        x: assets.load("x.png"),
        o: assets.load("o.png"),
    });
}

fn input(
    mut commands: Commands,
    mut turn: ResMut<Turn>,
    sprites: Res<Sprints>,
    keyboard: Res<Input<KeyCode>>,
    assets: Res<AssetServer>,
    mut board_query: Query<(&mut Board, &Transform)>,
) {
    // map 1,2,3,4,5,6,7,8 and 9
    // to cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8 and cell9
    // if keyboard.just_pressed(KeyCode::Numpad1) {
    //     for (mut board, transform) in board_query.iter_mut() {
    //         board.cell1.value = Mark::X;
    //         commands.spawn_bundle(SpriteBundle {
    //             texture: assets.load("x.png"),
    //             transform: Transform::from_xyz(transform.translation.x - 170.0, transform.translation.y + 170.0, 3.0),
    //             ..default()
    //         });
    //     }
    // }
    for key in keyboard.get_just_pressed() {
        let (mut board, transform) = board_query.single_mut();
        match key {
            KeyCode::Numpad1 => {
                if let Mark::Empty = board.cell1.value {
                    board.cell1.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell1.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad2 => {
                if let Mark::Empty = board.cell2.value {
                    board.cell2.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell2.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad3 => {
                if let Mark::Empty = board.cell3.value {
                    board.cell3.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell3.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y + 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad4 => {
                if let Mark::Empty = board.cell4.value {
                    board.cell4.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell4.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad5 => {
                if let Mark::Empty = board.cell5.value {
                    board.cell5.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell5.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad6 => {
                if let Mark::Empty = board.cell6.value {
                    board.cell6.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell6.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad7 => {
                if let Mark::Empty = board.cell7.value {
                    board.cell7.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell7.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x - 170.0,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad8 => {
                if let Mark::Empty = board.cell8.value {
                    board.cell8.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell8.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            KeyCode::Numpad9 => {
                if let Mark::Empty = board.cell9.value {
                    board.cell9.value = match turn.as_ref() {
                        Turn::X => {
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.x.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::X
                        }
                        Turn::O => {
                            board.cell9.value = Mark::O;
                            commands.spawn_bundle(SpriteBundle {
                                texture: sprites.o.clone(),
                                transform: Transform::from_xyz(
                                    transform.translation.x + 170.0,
                                    transform.translation.y - 170.0,
                                    3.0,
                                ),
                                ..default()
                            });
                            Mark::O
                        }
                    }
                }
            }
            _ => {}
        }
        *turn = match turn.as_ref() {
            Turn::X => Turn::O,
            Turn::O => Turn::X,
        };
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
        .add_startup_system(setup)
        .add_system(input)
        .run();
}

// use std::any::Any;
// use bevy::input::mouse::MouseMotion;
// use bevy::prelude::*;
// use bevy::prelude::Vec2;
//
// #[derive(Component)]
// struct Empty(usize, usize);
//
// #[derive(Component)]
// struct X(usize, usize);
//
// #[derive(Component)]
// struct O(usize, usize);
//
// #[derive(Component)]
// struct Board;
//
// struct Background;
//
// pub struct MainPlugin;
// impl Plugin for MainPlugin {
//     fn build(&self, app: &mut App) {
//         app.insert_resource(WindowDescriptor {
//             title: "Tic Tac Toe".to_string(),
//             width: 600.0,
//             height: 600.0,
//             present_mode: bevy::window::PresentMode::AutoVsync,
//             ..Default::default()
//         });
//     }
// }
//
// fn main() {
//     App::new()
//         .add_plugin(MainPlugin)
//         .add_plugins(DefaultPlugins)
//         .add_event::<MarkEvent>()
//         .add_startup_system(setup)
//         .add_system(board)
//         .add_system(mark)
//         .run();
// }
//
// fn setup(mut commands: Commands, assets: Res<AssetServer>) {
//     let camera = Camera2dBundle::default();
//
//     commands.spawn_bundle( Camera2dBundle::default());
//     commands.spawn_bundle(SpriteBundle {
//         texture: assets.load("background.png"),
//         transform: Transform::from_xyz(0.0, 0.0, 0.0),
//         ..default()
//     });
// }
//
// const cell1: Vec2 = Vec2::new(-170.0, 170.0);
// const cell2: Vec2 = Vec2::new(0.0, 170.0);
// const cell3: Vec2 = Vec2::new(170.0, 170.0);
//
// const cell4: Vec2 = Vec2::new(-170.0, 0.0);
// const cell5: Vec2 = Vec2::new(0.0, 0.0);
// const cell6: Vec2 = Vec2::new(170.0, 0.0);
//
// const cell7: Vec2 = Vec2::new(-170.0, -170.0);
// const cell8: Vec2 = Vec2::new(0.0, -170.0);
// const cell9: Vec2 = Vec2::new(170.0, -170.0);
//
// const cells: [Vec2; 9] = [cell1, cell2, cell3, cell4, cell5, cell6, cell7, cell8, cell9];
//
// struct MarkEvent;
//
// fn board(mut commands: Commands, assets: Res<AssetServer>, mut events: EventReader<MarkEvent>) {
//     commands.spawn_bundle(SpriteBundle {
//         texture: assets.load("board.png"),
//         transform: Transform::from_xyz(0.0, 0.0, 1.0),
//         ..default()
//     });
//
//     for n in 0..9 {
//         commands.spawn_bundle(SpriteBundle {
//             texture: assets.load("empty.png"),
//             transform: Transform::from_xyz(cells[n].x, cells[n].y, 2.0),
//             visibility: Visibility {
//                 is_visible: false,
//             },
//             ..default()
//         }).insert(Empty(n , n));
//     }
// }
//
// fn mark(mut commands: Commands, assets: Res<AssetServer>, keys: Res<Input<KeyCode>>, mut events: EventWriter<MarkEvent>, query: Query<(&mut Empty)>) {
//     for mut empty in query.iter() {
//         if keys.just_pressed(KeyCode::Numpad1) {
//             if empty.0 == 0 && empty.1 == 0 {
//                 commands.spawn_bundle(SpriteBundle {
//                     texture: assets.load("x.png"),
//                     transform: Transform::from_xyz(cells[empty.0].x, cells[empty.1].y, 3.0),
//                     ..default()
//                 }).insert(X(empty.0, empty.1));
//                 //commands.despawn(empty);
//                 events.send(MarkEvent);
//             }
//             //commands.despawn(empty);
//             // commands.spawn_bundle(SpriteBundle {
//             //     texture: assets.load("x.png"),
//             //     transform: Transform::from_xyz(cells[empty.0].x, cells[empty.0].y, 3.0),
//             //     ..default()
//             // }).insert(X(empty.0, empty.1));
//             //events.send(MarkEvent);
//         }
//         // if keys.just_pressed(KeyCode::O) {
//         //     commands.spawn_bundle(SpriteBundle {
//         //         texture: assets.load("o.png"),
//         //         transform: Transform::from_xyz(cells[empty.0].x, cells[empty.0].y, 3.0),
//         //         ..default()
//         //     }).insert(O(empty.0, empty.1));
//         //     commands.despawn(empty);
//         //     events.send(MarkEvent);
//         // }
//     }
//
//     // for key in keys.get_just_pressed() {
//     //     match key {
//     //         KeyCode::Numpad1 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell1.x, cell1.y, 2.0),
//     //                 ..default()
//     //             }).insert(X(0, 0));
//     //         }
//     //         _ => {}
//     //     }
//     // }
//
//     // for key in keys.get_just_pressed() {
//     //     match key {
//     //         KeyCode::Numpad1 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell1.x, cell1.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad2 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell2.x, cell2.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad3 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell3.x, cell3.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad4 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell4.x, cell4.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad5 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell5.x, cell5.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad6 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell6.x, cell6.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad7 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell7.x, cell7.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad8 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell8.x, cell8.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         KeyCode::Numpad9 => {
//     //             commands.spawn_bundle(SpriteBundle {
//     //                 texture: assets.load("x.png"),
//     //                 transform: Transform::from_xyz(cell9.x, cell9.y, 2.0),
//     //                 ..default()
//     //             }).insert(Sign::Empty);
//     //         }
//     //         _ => {}
//     //     }
//     // }
// }
