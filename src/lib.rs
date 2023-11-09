use bevy::prelude::*;
use bevy::asset::{AssetLoader,  AsyncReadExt};

pub struct PXOAssetLoader;

impl AssetLoader for PXOAssetLoader
{
	type Asset = Image;
	type Settings = ();
	type Error = pxo::PxoError;

	fn load<'a>
	(
		&'a self,
		reader: &'a mut bevy::asset::io::Reader,
		_settings: &'a Self::Settings,
		_load_context: &'a mut bevy::asset::LoadContext,
	) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>>
	{
		Box::pin
		(
			async move
			{
				let mut buffer = Vec::new();
				reader.read_to_end(&mut buffer).await?;
				let reader = std::io::Cursor::new(buffer);
				let pxo = pxo::Pxo::load(reader)?;
				let sprite = pxo::Sprite::from(pxo, pxo::SpriteOptions::default())?;
				let first_frame = sprite.images[0].clone();
				let image = Image::from_dynamic(image::DynamicImage::from(first_frame), true);
				Ok(image)
			}
		)
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
