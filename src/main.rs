//! An Asteroids-ish example game to show off ggez.
//! The idea is that this game is simple but still
//! non-trivial enough to be interesting.

mod ent;
mod com;
mod res;

mod state;

//use ent::*;


use ggez::{conf, ContextBuilder};
use ggez::event::{self};
use ggez::{GameResult};

//use serde::{Deserialize, Serialize};

use std::env;
use std::path;

/// **********************************************************************
/// Finally our main function!  Which merely sets up a config and calls
/// `ggez::event::run()` with our `EventHandler` type.
/// **********************************************************************

pub fn main() -> GameResult {
    // We add the CARGO_MANIFEST_DIR/resources to the resource paths
    // so that ggez will look in our cargo project directory for files.

    /*
    let vars = env::vars();

    for var in vars {
        println!( "{} = {}", var.0, var.1 );
    }
    // */

    let cargo_dir = env::var("CARGO_MANIFEST_DIR");

    println!("{}", cargo_dir.clone().unwrap_or_default());

    let resource_dir = if let Ok(manifest_dir) = cargo_dir {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./run/resources")
    };

    let cb = ContextBuilder::new("village", "marcsh")
        .window_setup(conf::WindowSetup::default().title("Village!"))
        .window_mode(conf::WindowMode::default().dimensions(2000.0, 1200.0))
        .add_resource_path(resource_dir);

    let (mut ctx, events_loop) = cb.build()?;

    let state = state::MainState::new(&mut ctx)?;

    /*

    //Verify assumptions about references

    let mut ent = ent::Entity::new();

    let _id = std::any::TypeId::of::<ent::Entity>();


    let mut r = com::Renderable {
        ..Default::default()
    };

    r.test = 10;

    println!("Test {}", r.test);

    ent.com.add( r );

    let e_mut = ent.com.get_mut::<com::Renderable>();

    let r_def = &mut com::Renderable::default();

    let r_new = e_mut.unwrap_or( r_def );

    println!("Test {}", r_new.test);

    r_new.test = 20;

    println!("Test {}", r_new.test);

    // */

    //state.world.add( ent );

    event::run(ctx, events_loop, state)

}
