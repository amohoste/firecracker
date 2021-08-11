use std::fs::File;
use std::path::Path;
use snapshot::Snapshot;
use vmm::persist::MicrovmState;
use vmm::version_map::VERSION_MAP;

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