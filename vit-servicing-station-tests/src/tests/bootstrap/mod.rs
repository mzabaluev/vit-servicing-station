use crate::common::{
    data,
    startup::{db::DbBuilder, server::ServerBootstrapper},
};
use assert_fs::TempDir;

pub mod parameters;

#[test]
pub fn bootstrap_with_random_data() -> Result<(), Box<dyn std::error::Error>> {
    let temp_dir = TempDir::new().unwrap().into_persistent();
    let snapshot = data::Generator::new().snapshot();
    let db_path = DbBuilder::new().with_snapshot(&snapshot).build(&temp_dir)?;

    let server = ServerBootstrapper::new()
        .with_db_path(db_path.to_str().unwrap())
        .start()?;

    std::thread::sleep(std::time::Duration::from_secs(1));
    assert!(server.is_up(&snapshot.token().1));
    Ok(())
}