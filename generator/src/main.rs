use anyhow::Result;
use generate::generate_sheet;
use ironworks::{
	excel::{Excel, FfxivSqpackResource},
	sqpack::{FfxivFsResource, SqPack},
};
use ironworks_schema::saint_coinach::Provider;

mod generate;

fn main() -> Result<()> {
	saint_coinach()?;

	Ok(())
}

// TODO: Seperate file and all that jazz.
fn saint_coinach() -> Result<()> {
	let provider = Provider::new()?;

	let sheet_name = "CustomTalk";

	// TODO: fetch updates to the provider to make sure it's fresh
	// TODO: arg for version?
	let version = provider.version("HEAD")?;
	let schema = version.sheet(sheet_name)?;

	// TODO: probably need args for this stuff too
	// TODO: this might be shareable across providers
	let sqpack = SqPack::new(FfxivFsResource::search().unwrap());
	let excel = Excel::new(FfxivSqpackResource::new(&sqpack));
	let sheet = excel.sheet(sheet_name)?;

	println!("cols: {:#?}", sheet.columns());

	generate_sheet(sheet_name, schema);

	Ok(())
}
