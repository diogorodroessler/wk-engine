use bevy_app::{App, Update};
use bevy_asset::{AssetApp, AssetPlugin};
use bevy_tasks::IoTaskPool;

//---------------------------------------------------------------

pub mod audio;
pub mod asset;
pub mod render;

//---------------------------------------------------------------

fn main() {
    let mut app = App::new();

    app.add_plugins(
        AssetPlugin::default()
    );
    app.init_asset::<asset::wk_loader::AssetIntoHandle>();
    app.add_systems(Update, game_system);
    
    IoTaskPool::get_or_init(|| {
        bevy_tasks::TaskPool {  }
    });

    let gpu = pollster::block_on(
        init_wgpu(&window)
    );

    loop
    {
        process_events();

        app.update();

        render(&gpu);
    }
}

fn game_system(
) {
    println!("Game System initialize");
}
