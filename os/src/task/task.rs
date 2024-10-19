//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;
/// The task control block (TCB) of a task.
#[derive(Copy, Clone, Debug)]
pub struct TaskControlBlock {
    /// The task status in it's lifecycle
    pub task_status: TaskStatus,
    /// The task context
    pub task_cx: TaskContext,
    /// task first create time
    pub  create_time: usize,
     /// syscall
    pub  syscall_times: [u32; MAX_SYSCALL_NUM],
}
impl TaskControlBlock {
    /// init 
    pub fn new()->Self{
        Self{
            task_cx: TaskContext::zero_init(),
            task_status: TaskStatus::UnInit,
            create_time: 0,
            syscall_times: [0; MAX_SYSCALL_NUM],
        }
    }
/// inc_sys_call
    pub fn inc_sys_call(&mut self, id:usize){
        self.syscall_times[id]+=1;
    }

}

/// The status of a task
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum TaskStatus {
    /// uninitialized
    UnInit,
    /// ready to run
    Ready,
    /// running
    Running,
    /// exited
    Exited,
}
