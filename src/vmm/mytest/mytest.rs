use std::fs::File;
use std::path::Path;
use snapshot::Snapshot;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::thread;
use std::time::Duration;
use utils::tempfile::TempFile;
use versionize::VersionMap;
use vmm::persist::MicrovmState;
use vmm::utilities::mock_resources::NOISY_KERNEL_IMAGE;
use vmm::utilities::test_utils::create_vmm;
use vmm::version_map::VERSION_MAP;
use vmm::vmm_config::snapshot::{CreateSnapshotParams, SnapshotType};
use vmm::{persist, FC_EXIT_CODE_OK};

fn main() {
    let snapshot_path = Path::new("snapfile");
    let mut snapshot_reader = File::open(snapshot_path).unwrap();
    let snapshot_file_metadata = std::fs::metadata(snapshot_path).unwrap();
    let snapshot_len = snapshot_file_metadata.len() as usize;
    let microvm_state: MicrovmState = Snapshot::load(
        &mut snapshot_reader,
        snapshot_len,
        VERSION_MAP.clone(),
    ).unwrap();
    println!("{:?}", microvm_state);

}