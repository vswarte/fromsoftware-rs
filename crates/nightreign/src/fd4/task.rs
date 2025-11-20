use super::FD4Time;

#[repr(C)]
#[derive(Debug)]
pub struct FD4TaskData {
    pub delta_time: FD4Time,
    pub task_group_id: u32,
    pub seed: i32,
}
