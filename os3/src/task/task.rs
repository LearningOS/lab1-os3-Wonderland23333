//! Types related to task management

use super::TaskContext;
use crate::config::MAX_SYSCALL_NUM;

#[derive(Clone, Copy)]
// take control block structure
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,
    // add whatever you need in Lab1
    pub task_begin_time: usize,
    pub syscall_times: [u32; MAX_SYSCALL_NUM],
}

#[derive(Copy, Clone, PartialEq)]
// Task status: UnInit, Ready, Running, Exited
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}