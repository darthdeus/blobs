// {
//     let spacing = 0.3;
//     let num = 10;
//     let w = num;
//     let h = num;
//
//     let offset = -vec2(w as f32 * spacing, h as f32 * spacing) / 2.0;
//
//     let grid = grids::Grid::filled_with(w, h, |x, y| {
//         let idx = sim.balls.insert(TestObject {
//             position: Vec2::ZERO,
//             color: YELLOW,
//         });
//
//         let rbd_handle = spawn_rbd_entity(
//             &mut sim.physics,
//             idx,
//             RigidBodyDesc {
//                 position: vec2(x as f32 * spacing, y as f32 * spacing) + offset,
//                 radius: 0.1,
//                 // gravity_mod: 0.0,
//                 ..Default::default()
//             },
//         );
//
//         (idx, rbd_handle)
//     });
//
//     for (x, y, (_, rbd_handle_a)) in grid.iter() {
//         if x < grid.width() - 1 {
//             let rbd_handle_b = grid[(x + 1, y)].1;
//             sim.physics
//                 .create_fixed_joint(*rbd_handle_a, rbd_handle_b, Vec2::ZERO, Vec2::ZERO);
//         }
//         if y < grid.width() - 1 {
//             let rbd_handle_b = grid[(x, y + 1)].1;
//             sim.physics
//                 .create_fixed_joint(*rbd_handle_a, rbd_handle_b, Vec2::ZERO, Vec2::ZERO);
//         }
//     }
//
//     let a = sim.balls.insert(TestObject {
//         position: Vec2::ZERO,
//         color: ORANGE,
//     });
//
//     let cloth_pin = spawn_rbd_entity(
//         &mut sim.physics,
//         a,
//         RigidBodyDesc {
//             position: vec2(0.0, 3.5),
//             body_type: RigidBodyType::Static,
//             collision_groups: groups(0, 0),
//             // gravity_mod: 0.0,
//             ..Default::default()
//         },
//     );
//
//     let grid_anchor = grid[(5, 0)].1;
//
//     // let spring = blobs.springs.insert(Spring {
//     //     rigid_body_a: grid_anchor,
//     //     rigid_body_b: cloth_pin,
//     //     rest_length: 1.0,
//     //     stiffness: 3000.0,
//     //     damping: 50.0,
//     // });
//
//     sim.physics.create_fixed_joint_with_distance(
//         grid_anchor,
//         cloth_pin,
//         Vec2::ZERO,
//         Vec2::ZERO,
//         0.1,
//     );
// }
