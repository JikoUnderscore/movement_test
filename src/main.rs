#![allow(non_snake_case)]

use bevy_ecs::prelude::{Entity, With, Without, World};
use rand::Rng;
use crate::ENGINE::components::{AccelerationComp, MobComp, MovementComp, PlayerComp, SpriteComp};
use crate::ENGINE::core::{FpsCapDeltaTime, Point2D, Rect, Renderer, SDLErrs, Vector2D};
use crate::ENGINE::events;

mod ENGINE;

pub const WINDOW_WIDHT: i32 = 1280;
pub const WINDOW_HEIGHT: i32 = 720;


fn random_pos_xy(i: i32) -> (i32, i32) {
    let rand_x = if i % 2 == 0 {
        if rand::thread_rng().gen::<f32>() > 0.5 { -17 } else { 1280 + 17 }
    } else {
        rand::thread_rng().gen_range(-17..1280 + 17)
    };
    let rand_y = if i % 2 == 0 {
        rand::thread_rng().gen_range(-17..720 + 17)
    } else {
        if rand::thread_rng().gen::<f32>() > 0.5 { -17 } else { 720 + 17 }
    };

    return (rand_x, rand_y);
}


fn main() -> Result<(), SDLErrs> {
    let mut core = Renderer::new("movement test\0")?;
    let sprite_sheet = core.load_texture("./assets/sprites.png\0")?;
    let mut fps_ctrl = FpsCapDeltaTime::new(60);

    let mut world = World::new();

    const SPAWN_Y: i32 = WINDOW_HEIGHT / 2;
    const VEL: f32 = 190.0;
    const SPAWN_X: i32 = WINDOW_WIDHT / 2;

    let pla = world.spawn()
                   .insert(PlayerComp {})
                   .insert(SpriteComp { srs: Rect::new(16 * 3, 16 * 3, 16 * 3, 17 * 3), pos: Point2D::new(SPAWN_X, SPAWN_Y) })
                   .insert(MovementComp { position: Vector2D::new(SPAWN_X as f32, SPAWN_Y as f32) })
                   .insert(AccelerationComp { acceleration: Vector2D::def() })
                   .id();

    static MOB_LIST: [(i32, i32, i32, i32); 3] = [
        (0 * 3, 0 * 3, 16 * 3, 16 * 3),
        (16 * 3, 0 * 3, 16 * 3, 16 * 3),
        (0 * 3, 16 * 3, 16 * 3, 17 * 3)];

    let mut rng = rand::thread_rng();


    for i in 0..11 {
        let r = rng.gen_range(0..3);
        let mob_src = Rect::new(MOB_LIST[r].0, MOB_LIST[r].1, MOB_LIST[r].2, MOB_LIST[r].3);
        let (rand_x, rand_y) = random_pos_xy(i);

        let random_number = rng.gen_bool(0.5);
        let (x, y) = if random_number { (-1.0, 1.0) } else { (1.0, -1.0) };

        world.spawn()
             .insert(SpriteComp { srs: mob_src, pos: Point2D::new(rand_x, rand_y) })
             .insert(MovementComp { position: Vector2D::new(rand_x as f32, rand_y as f32) })
             .insert(AccelerationComp { acceleration: Vector2D::def() })
             .insert(MobComp { rotate_dir: Vector2D::new(x, y) });
    }

    let mut render_all = world.query::<&SpriteComp>();
    let mut pla_acceleration = world.query::<(&mut AccelerationComp, With<PlayerComp>)>();

    // let mut pla_movement_posisiton = world.query::<(&MovementComp, With<PlayerComp>)>();
    let mut update_mob_acceleration = world.query::<(&mut AccelerationComp, &MovementComp, &MobComp)>();
    let mut update_mob_self_collition = world.query::<(&mut AccelerationComp, &MobComp, Entity, &MovementComp)>();
    let mut update_mob_self_collition2 = world.query::<(&mut AccelerationComp, &mut MobComp, Entity, &mut MovementComp)>();
    let mut update_all = world.query::<(&mut AccelerationComp, &mut SpriteComp, &mut MovementComp)>();


    let keys = events::get_keyboard_state();


    let mut is_running = true;
//--------- LOOP
    while is_running {
        fps_ctrl.start();

//--------- EVENT
        {
            for (mut dir, _pla) in pla_acceleration.iter_mut(&mut world) {
                if keys.is_scancode_pressed(events::ScanCode::A) {
                    dir.acceleration.x = -VEL;
                } else if keys.is_scancode_pressed(events::ScanCode::D) {
                    dir.acceleration.x = VEL;
                }
                if keys.is_scancode_pressed(events::ScanCode::W) {
                    dir.acceleration.y = -VEL;
                } else if keys.is_scancode_pressed(events::ScanCode::S) {
                    dir.acceleration.y = VEL;
                }
            }

            while let Some(w_event) = events::poll_iter() {
                match w_event.type_() {
                    events::QUIT => {
                        is_running = false;
                    },
                    _ => {},
                }
            }
        }
//--------- UPDATE
        {


            // mob direction of movement and/or rotaion around player
            let pla_position = (*world.get::<MovementComp>(pla).unwrap()).position.clone();
            // for (mut mob_dir , mob_movement, mob) in update_mob_acceleration.iter_mut(&mut world) {
            //
            // }

            // mob other mob collision
            for (mut dir1, mob1, entt1, movement1) in unsafe { update_mob_self_collition.iter_unchecked(&world) } {
                const DIS: f32 = 40.0 * 40.0;
                const DIS2: f32 = 20.0 * 20.0;

                let mut dir_x = pla_position.x - movement1.position.x;
                let mut dir_y = pla_position.y - movement1.position.y;
                let distance_squared = dir_x * dir_x + dir_y * dir_y;

                let hyp = (dir_x * dir_x + dir_y * dir_y).sqrt();
                dir_x /= hyp;
                dir_y /= hyp;
                if DIS < distance_squared {
                    dir1.acceleration.x = dir_x * VEL;
                    dir1.acceleration.y = dir_y * VEL;
                } else {
                    dir1.acceleration.x = (dir_y * VEL) * mob1.rotate_dir.x;
                    dir1.acceleration.y = (dir_x * VEL) * mob1.rotate_dir.y;
                    // dir1.acceleration.x = -(dir_x * VEL);
                    // dir1.acceleration.y = -(dir_y * VEL);
                }


                for (mut dir2,  _mob2, entt2, mut movement2) in unsafe { update_mob_self_collition2.iter_unchecked(&world) } {
                    if entt1 == entt2 {
                        continue;
                    }
                    // dir1.acceleration.x = (x * VEL);
                    // dir1.acceleration.y = (y * VEL);


                    // dir2.acceleration.x = -(x * VEL);
                    // dir2.acceleration.y = -(y * VEL);

                    // dbg!(normalized);
                    // movement1.position.x += normalized.0 * VEL;
                    // movement1.position.y += normalized.1 * VEL;
                    // let other = Rect::new(movement2.position.x as i32, movement2.position.y as i32, 16 * 3, 16 * 3);
                    // let this = Rect::new(movement1.position.x as i32, movement1.position.y as i32, 16 * 3, 16 * 3);
                    // if this.has_intersection(&other) {
                    let mut x = movement1.position.x - movement2.position.x;
                    let mut y = movement1.position.y - movement2.position.y;
                    let dist_squered = x * x + y * y;
                    if DIS < dist_squered {
                        continue;
                    }


                    let hyp = dist_squered.sqrt();
                    x /= hyp;
                    y /= hyp;
                    let normalized = if dist_squered != 0.0 { (x / dist_squered, y / dist_squered) } else { (x, y) };

                    if dir1.acceleration.x > 0.0 || dir1.acceleration.x < 0.0 {
                        // mob2.rotate_dir.x *= mob1.rotate_dir.x;
                        // mob2.rotate_dir.x *= -1.0;

                        movement2.position.x += -(normalized.0 * VEL * 5.0);
                        // dir2.acceleration.x = -(x * VEL);
                        dir2.acceleration.x = dir1.acceleration.x;
                    }
                    if dir1.acceleration.y > 0.0 || dir1.acceleration.y < 0.0 {
                        // mob2.rotate_dir.y *= mob2.rotate_dir.x;
                        // mob2.rotate_dir.y *= -1.0;

                        movement2.position.y += -(normalized.1 * VEL * 5.0);
                        // dir2.acceleration.y = -(y * VEL);
                        dir2.acceleration.y = dir1.acceleration.y;
                    }


                    // movement2.position.x += -(normalized.0 * VEL);
                    // movement2.position.y += -(normalized.1 * VEL);
                }
            }

            // update all movement
            for (mut dir, mut sprite, mut movement) in update_all.iter_mut(&mut world) {
                if dir.acceleration.x != 0.0 && dir.acceleration.y != 0.0 {
                    dir.acceleration.x *= std::f32::consts::FRAC_1_SQRT_2;
                    dir.acceleration.y *= std::f32::consts::FRAC_1_SQRT_2;
                }

                movement.position.x += dir.acceleration.x * fps_ctrl.dt;
                movement.position.y += dir.acceleration.y * fps_ctrl.dt;

                dir.acceleration.x = 0.0;
                dir.acceleration.y = 0.0;

                sprite.pos.set_x(movement.position.x as i32);
                sprite.pos.set_y(movement.position.y as i32);
            }
        }
//--------- RENDER
        {
            // core.set_draw_color((10, 10, 30));
            core.clear();

            // render all sprites
            let mut rendr = render_all.iter(&world).collect::<Vec<_>>();
            rendr.sort_unstable_by_key(|a| a.pos.y() + a.srs.width() / 2);
            for sprite in rendr {
                core.renderer_copy(&sprite_sheet, sprite.srs, Rect::new(sprite.pos.x(), sprite.pos.y(), sprite.srs.width(), sprite.srs.height()))?;
            }
        }

        core.present();
        fps_ctrl.end();
    }
    return Ok(());
}

