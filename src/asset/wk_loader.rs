
use bevy_asset::{Asset, AssetServer};
use bevy_ecs::prelude::*;
use bevy_reflect::Reflect;

//---------------------------------------------------------------

#[derive(Asset, Reflect)]
pub struct AssetIntoHandle;

//---------------------------------------------------------------

pub struct LoadAsset;
impl LoadAsset
{

    /// Uses for load asset
    pub fn load_asset(
        asset_server: Res<AssetServer>,
    )
    {
        let _load_asset_handle: bevy_asset::Handle<AssetIntoHandle> = asset_server.load("textures/claire.png");

        println!("Asset requested!");
    }

}