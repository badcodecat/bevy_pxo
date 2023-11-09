use bevy::prelude::*;
use bevy::asset::{AssetLoader, LoadedAsset};

pub struct PXOAssetLoader;

impl AssetLoader for PXOAssetLoader
{
	type Asset = ();
	type Settings = ();
	type Error = pxo::PxoError;

	fn load<'a>
	(
		&'a self,
		reader: &'a mut bevy::asset::io::Reader,
		settings: &'a Self::Settings,
		load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>>
	{
		todo!()
	}

	fn extensions(&self) -> &[&str]
	{
		&["pxo"]
	}
}


pub struct PXOPlugin;

impl Plugin for PXOPlugin
{
	fn build(&self, app: &mut App)
	{
		app
			.register_asset_loader(PXOAssetLoader)
			;
	}
}
