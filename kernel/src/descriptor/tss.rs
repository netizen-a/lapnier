use core::cell::SyncUnsafeCell;

#[repr(C, packed)]
pub struct TaskStateSegment {
    _reserved0: u32,
    pub privilege_stack_table: [u64; 3],
    _reserved1: u64,
    pub interrupt_stack_table: [u64; 7],
    _reserved2: u64,
    _reserved3: u16,
    pub iomap_base: u16,
}

pub static TSS: SyncUnsafeCell<TaskStateSegment> = SyncUnsafeCell::new(TaskStateSegment{
    privilege_stack_table: [0; 3],
    interrupt_stack_table: [0; 7],
    iomap_base: 0,
    _reserved0: 0,
    _reserved1: 0,
    _reserved2: 0,
    _reserved3: 0,
});
