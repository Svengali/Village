
//! Renders an animated sprite by loading all animation frames from a single image (a sprite sheet)
//! into a texture atlas, and changing the displayed image periodically.

use bevy::app::AppExit;
use bevy::input::mouse::{MouseButtonInput, MouseWheel, MouseScrollUnit, MouseMotion};
use bevy::render::{RenderStage, RenderApp};
//use bevy::log::LogSettings;
use bevy::{
    prelude::*, 
};
use bevy_egui::{egui, EguiContext, EguiPlugin};

/* Async tests
use futures_lite::future;
use rand::Rng;
use std::time::{Duration, Instant};
use rand::RngCore;
use bevy::tasks::{AsyncComputeTaskPool, Task};
*/

mod fixed;

mod tile;
use bevy_sprite::SpriteSystem;
use tile::map::Map;


/*
pub mod prelude {
    #[doc(hidden)]
    pub use crate::{
        bundle::{TileBundle}
    };
}
*/

/*
// Enum that will be used as a global state for the game
#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Invalid,
    Splash,
    Menu,
    Game,
}
*/

fn main() {
    let mut app = App::new();
 
    //ComputeTaskPool::init(TaskPool::default);


    //app.insert_resource(ImageSettings::default_nearest()); // prevents blurry sprites
    app.add_plugins(DefaultPlugins);
    app.add_plugin(EguiPlugin);
    app.add_system(keyboard_input_system);
    app.add_startup_system(setup);
    //app.add_system(animate_sprite);
    app.add_system(camera_movement);
    app.add_system(mouse_button_events);
    app.add_system(mouse_move_event);
    app.add_system(scroll_events);
    app.add_system(once_visibility_propagate);
    //app.add_system(test_transform);
    //app.add_system(my_cursor_system);

    app.add_system(ui);

    app.add_system(bevy::window::close_on_esc);

    /* Async test
    app.add_startup_system(setup_env);
    app.add_startup_system(add_assets);
    app.add_startup_system(spawn_tasks);
    app.add_system(handle_tasks);
    */



    if let Ok(render_app) = app.get_sub_app_mut(RenderApp) {
        render_app
        .add_system_to_stage(
        RenderStage::Extract,
        fixed::extract_sprites.label(SpriteSystem::ExtractSprites));
    };








    app.run();
}

/*
#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

fn animate_sprite(
    time: Res<Time>,
    texture_atlases: Res<Assets<TextureAtlas>>,
    mut query: Query<(
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
        &Handle<TextureAtlas>,
    )>,
) {
    for (mut timer, mut sprite, texture_atlas_handle) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            let texture_atlas = texture_atlases.get(texture_atlas_handle).unwrap();
            sprite.index = (sprite.index + 1) % texture_atlas.textures.len();
        }
    }
}
*/

static mut S_HAS_DONE_VISIBLITY: bool = false;

fn once_visibility_propagate(
    mut root_query: Query<
        (
            Option<&Children>,
            &Visibility,
            &mut ComputedVisibility,
            Entity,
        ),
        Without<Parent>,
    >,
    _visibility_query: Query<(&Visibility, &mut ComputedVisibility, &Parent)>,
    _children_query: Query<&Children, (With<Parent>, With<Visibility>, With<ComputedVisibility>)>,
) {

    unsafe {
        if S_HAS_DONE_VISIBLITY { return };
    }

    for (_children, visibility, mut computed_visibility, _entity) in root_query.iter_mut() {
        // reset "view" visibility here ... if this entity should be drawn a future system should set this to true
        computed_visibility.reset(visibility.is_visible);
    }

    unsafe {
        S_HAS_DONE_VISIBLITY = true;
    }
}


fn ui(
    mut context: ResMut<EguiContext>,
) {
    egui::Window::new("Hello").show(context.ctx_mut(), |ui| {
        ui.label("world");
    });
}



#[derive(Component)]
struct MainCamera;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    commands.spawn(Camera2dBundle::default())
        .insert(MainCamera);



    /*
    let texture_handle = asset_server.load("textures/world/base_out_atlas.png");
    let texture_atlas = TextureAtlas::from_grid(texture_handle, Vec2::new(32.0, 32.0), 32, 32);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: texture_atlas_handle,
            transform: Transform::from_scale(Vec3::splat(2.0)),
            ..default()
        })
        .insert(AnimationTimer(Timer::from_seconds(1.0, true)));
    // */

    //*
    Map::new(
        commands,
        asset_server,
        texture_atlases,
    );
    // */
}

/*
fn my_cursor_system(
    // need to get window dimensions
    wnds: Res<Windows>,
    // query to get camera transform
    q_camera: Query<(&Camera, &GlobalTransform), With<MainCamera>>,
    mut egui_context: EguiContext,
    time: Res<Time>,
) {
    // get the camera info and transform
    // assuming there is exactly one main camera entity, so query::single() is OK
    let (camera, camera_transform) = q_camera.single();

    // get the window that the camera is displaying to (or the primary window)
    let wnd = if let RenderTarget::Window(id) = camera.target {
        wnds.get(id).unwrap()
    } else {
        wnds.get_primary().unwrap()
    };

    // check if the cursor is inside the window and get its position
    if let Some(screen_pos) = wnd.cursor_position() {
        // get the size of the window
        let window_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);

        let world_pos = fun_name(screen_pos, window_size, camera_transform, camera);

        egui::Window::new("my_cursor_system")
            .anchor(Align2::LEFT_TOP, bevy_egui::egui::Vec2::new(10.0, 10.0))
            .show(egui_context.ctx_mut(), |ui| {

            //let world_pos_str = ;

            let delta = time.delta_seconds();

            ui.label( format!("{world_pos} {delta}") );
        });

        //eprintln!("World coords: {}/{}", world_pos.x, world_pos.y);
    }
}
*/

/*
fn fun_name(screen_pos: Vec2, window_size: Vec2, camera_transform: &GlobalTransform, camera: &Camera) -> Vec2 {
    // convert screen position [0..resolution] to ndc [-1..1] (gpu coordinates)
    let ndc = (screen_pos / window_size) * 2.0 - Vec2::ONE;
    // matrix for undoing the projection and camera transform
    let ndc_to_world = camera_transform.compute_matrix() * camera.projection_matrix().inverse();
    // use it to convert ndc to world-space coordinates
    let world_pos = ndc_to_world.project_point3(ndc.extend(-1.0));
    // reduce it to a 2D value
    let world_pos: Vec2 = world_pos.truncate();

    world_pos
}
*/


fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>, 
    mut exit: EventWriter<AppExit>,

) {

    if keyboard_input.pressed(KeyCode::Escape) {
        exit.send(AppExit);
    }

    if keyboard_input.just_pressed(KeyCode::W) {
    }

    if keyboard_input.just_released(KeyCode::S) {
    }


    if keyboard_input.just_pressed(KeyCode::A) {
    }

    if keyboard_input.just_released(KeyCode::D) {
    }

}

/*
fn test_transform(
    _time: Res<Time>,
    _keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&TextureAtlasSprite, &Transform, Changed<Transform>)>,
) {
    let mut v = 0.0;

    for (_atlas, transform, _changed) in query.iter() {


        v += transform.translation.x;


    }
}
*/

fn camera_movement(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        let mut direction = Vec3::ZERO;
        let scale = transform.scale.x;

        let mut change = false;
        
        if keyboard_input.pressed(KeyCode::A) {
            direction -= Vec3::new(1.0, 0.0, 0.0);
            change = true;
        }

        if keyboard_input.pressed(KeyCode::D) {
            direction += Vec3::new(1.0, 0.0, 0.0);
            change = true;
        }

        if keyboard_input.pressed(KeyCode::W) {
            direction += Vec3::new(0.0, 1.0, 0.0);
            change = true;
        }

        if keyboard_input.pressed(KeyCode::S) {
            direction -= Vec3::new(0.0, 1.0, 0.0);
            change = true;
        }

        if keyboard_input.pressed(KeyCode::Z) {
            let scale = scale + 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
            change = true;
        }

        if keyboard_input.pressed(KeyCode::X) && scale > 1.1 {
            let scale = scale - 0.1;
            transform.scale = Vec3::new(scale, scale, scale);
            change = true;
        }

        if change {
            transform.translation += time.delta_seconds() * direction * 1000.0;
        }

    }
}

fn mouse_move_event( 
    mut mouse: EventReader<MouseMotion>,
) {
    for _ev in mouse.iter() {
        //eprintln!("Mouse Delta: {}/{}", ev.delta.x, ev.delta.y);
    }
}

fn mouse_button_events(
    mut mousebtn_evr: EventReader<MouseButtonInput>,
) {
    use bevy::input::ButtonState;

    for ev in mousebtn_evr.iter() {
        match ev.state {
            ButtonState::Pressed => {
                //println!("Mouse button press: {:?}", ev.button);
            }
            ButtonState::Released => {
                //println!("Mouse button release: {:?}", ev.button);
            }
        }
    }
}

fn scroll_events(
    mut scroll_evr: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {

    //let log_settings = LogSettings::new();
    //let handle: Handle<Level> = asset_server.load("trees.json.level");


    for (_, mut transform) in query.iter_mut() {
        for ev in scroll_evr.iter() {
            let scroll;

            match ev.unit {
                MouseScrollUnit::Line => {
                    //println!("Scroll (line units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    scroll = ev.y;

                }
                MouseScrollUnit::Pixel => {
                    //println!("Scroll (pixel units): vertical: {}, horizontal: {}", ev.y, ev.x);
                    scroll = ev.y * 10.0;
                }
            }

            let scale = scroll * 1.0;
            let old_scale = transform.scale.x;
            let new_scale = (old_scale + scale).clamp( 1.0, 20.0 );
            transform.scale = Vec3::splat( new_scale );
        }
    }
}

/*

Example of Async handling

// Number of cubes to spawn across the x, y, and z axis
const NUM_CUBES: u32 = 10;

#[derive(Resource, Deref)]
struct BoxMeshHandle(Handle<Mesh>);

#[derive(Resource, Deref)]
struct BoxMaterialHandle(Handle<StandardMaterial>);

/// Startup system which runs only once and generates our Box Mesh
/// and Box Material assets, adds them to their respective Asset
/// Resources, and stores their handles as resources so we can access
/// them later when we're ready to render our Boxes
fn add_assets(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let box_mesh_handle = meshes.add(Mesh::from(shape::Cube { size: 0.25 }));
    commands.insert_resource(BoxMeshHandle(box_mesh_handle));

    let box_material_handle = materials.add(Color::rgb(1.0, 0.2, 0.3).into());
    commands.insert_resource(BoxMaterialHandle(box_material_handle));
}

#[derive(Component)]
struct ComputeTransform(Task<Transform>);

/// This system generates tasks simulating computationally intensive
/// work that potentially spans multiple frames/ticks. A separate
/// system, `handle_tasks`, will poll the spawned tasks on subsequent
/// frames/ticks, and use the results to spawn cubes
fn spawn_tasks(mut commands: Commands) {
    let thread_pool = AsyncComputeTaskPool::get();
    for x in 0..NUM_CUBES {
        for y in 0..NUM_CUBES {
            for z in 0..NUM_CUBES {
                // Spawn new task on the AsyncComputeTaskPool
                let task = thread_pool.spawn(async move {
                    let mut rng = rand::thread_rng();
                    let start_time = Instant::now();


                    let duration = Duration::from_secs_f32(rng.gen_range(0.1..1.0));
                    //let duration = Duration::from_secs_f32(10.0);

                    while start_time.elapsed() < duration {
                        // Spinning for 'duration', simulating doing hard
                        // compute work generating translation coords!
                    }

                    // Such hard work, all done!
                    Transform::from_xyz(x as f32, y as f32, z as f32)
                });

                // Spawn new entity and add our new task as a component
                commands.spawn(ComputeTransform(task));
            }
        }
    }
}

/// This system queries for entities that have our Task<Transform> component. It polls the
/// tasks to see if they're complete. If the task is complete it takes the result, adds a
/// new [`PbrBundle`] of components to the entity using the result from the task's work, and
/// removes the task component from the entity.
fn handle_tasks(
    mut commands: Commands,
    mut transform_tasks: Query<(Entity, &mut ComputeTransform)>,
    box_mesh_handle: Res<BoxMeshHandle>,
    box_material_handle: Res<BoxMaterialHandle>,
) {
    for (entity, mut task) in &mut transform_tasks {
        if true { //task.0.is_finished() {
            let res = future::block_on(future::poll_once(&mut task.0));

            if let Some(transform) = res {
                // Add our new PbrBundle of components to our tagged entity
                commands.entity(entity).insert(PbrBundle {
                    mesh: box_mesh_handle.clone(),
                    material: box_material_handle.clone(),
                    transform,
                    ..default()
                });

                // Task is complete, so remove task component from entity
                commands.entity(entity).remove::<ComputeTransform>();
            }
        } else {

            let _x1 = 0;

        }
    }
}

/// This system is only used to setup light and camera for the environment
fn setup_env(mut commands: Commands) {
    // Used to center camera on spawned cubes
    let offset = if NUM_CUBES % 2 == 0 {
        (NUM_CUBES / 2) as f32 - 0.5
    } else {
        (NUM_CUBES / 2) as f32
    };

    // lights
    commands.spawn(PointLightBundle {
        transform: Transform::from_xyz(4.0, 12.0, 15.0),
        ..default()
    });

    /*
    // camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(offset, offset, 15.0)
            .looking_at(Vec3::new(offset, offset, 0.0), Vec3::Y),
        ..default()
    });
    */
}


*/