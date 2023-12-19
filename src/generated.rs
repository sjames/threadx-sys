/* automatically generated by rust-bindgen 0.69.1 */

pub type CHAR = ::core::ffi::c_char;
pub type UCHAR = ::core::ffi::c_uchar;
pub type UINT = ::core::ffi::c_uint;
pub type ULONG = ::core::ffi::c_ulong;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_TIMER_INTERNAL_STRUCT {
    pub tx_timer_internal_remaining_ticks: ULONG,
    pub tx_timer_internal_re_initialize_ticks: ULONG,
    pub tx_timer_internal_timeout_function: ::core::option::Option<unsafe extern "C" fn(id: ULONG)>,
    pub tx_timer_internal_timeout_param: ULONG,
    pub tx_timer_internal_active_next: *mut TX_TIMER_INTERNAL_STRUCT,
    pub tx_timer_internal_active_previous: *mut TX_TIMER_INTERNAL_STRUCT,
    pub tx_timer_internal_list_head: *mut *mut TX_TIMER_INTERNAL_STRUCT,
}
pub type TX_TIMER_INTERNAL = TX_TIMER_INTERNAL_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_TIMER_STRUCT {
    pub tx_timer_id: ULONG,
    pub tx_timer_name: *mut CHAR,
    pub tx_timer_internal: TX_TIMER_INTERNAL,
    pub tx_timer_created_next: *mut TX_TIMER_STRUCT,
    pub tx_timer_created_previous: *mut TX_TIMER_STRUCT,
}
pub type TX_TIMER = TX_TIMER_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_THREAD_STRUCT {
    pub tx_thread_id: ULONG,
    pub tx_thread_run_count: ULONG,
    pub tx_thread_stack_ptr: *mut ::core::ffi::c_void,
    pub tx_thread_stack_start: *mut ::core::ffi::c_void,
    pub tx_thread_stack_end: *mut ::core::ffi::c_void,
    pub tx_thread_stack_size: ULONG,
    pub tx_thread_time_slice: ULONG,
    pub tx_thread_new_time_slice: ULONG,
    pub tx_thread_ready_next: *mut TX_THREAD_STRUCT,
    pub tx_thread_ready_previous: *mut TX_THREAD_STRUCT,
    pub tx_thread_name: *mut CHAR,
    pub tx_thread_priority: UINT,
    pub tx_thread_state: UINT,
    pub tx_thread_delayed_suspend: UINT,
    pub tx_thread_suspending: UINT,
    pub tx_thread_preempt_threshold: UINT,
    pub tx_thread_schedule_hook:
        ::core::option::Option<unsafe extern "C" fn(thread_ptr: *mut TX_THREAD_STRUCT, id: ULONG)>,
    pub tx_thread_entry: ::core::option::Option<unsafe extern "C" fn(id: ULONG)>,
    pub tx_thread_entry_parameter: ULONG,
    pub tx_thread_timer: TX_TIMER_INTERNAL,
    pub tx_thread_suspend_cleanup: ::core::option::Option<
        unsafe extern "C" fn(thread_ptr: *mut TX_THREAD_STRUCT, suspension_sequence: ULONG),
    >,
    pub tx_thread_suspend_control_block: *mut ::core::ffi::c_void,
    pub tx_thread_suspended_next: *mut TX_THREAD_STRUCT,
    pub tx_thread_suspended_previous: *mut TX_THREAD_STRUCT,
    pub tx_thread_suspend_info: ULONG,
    pub tx_thread_additional_suspend_info: *mut ::core::ffi::c_void,
    pub tx_thread_suspend_option: UINT,
    pub tx_thread_suspend_status: UINT,
    pub tx_thread_created_next: *mut TX_THREAD_STRUCT,
    pub tx_thread_created_previous: *mut TX_THREAD_STRUCT,
    pub tx_thread_filex_ptr: *mut ::core::ffi::c_void,
    pub tx_thread_user_priority: UINT,
    pub tx_thread_user_preempt_threshold: UINT,
    pub tx_thread_inherit_priority: UINT,
    pub tx_thread_owned_mutex_count: UINT,
    pub tx_thread_owned_mutex_list: *mut TX_MUTEX_STRUCT,
    pub tx_thread_stack_highest_ptr: *mut ::core::ffi::c_void,
    pub tx_thread_entry_exit_notify: ::core::option::Option<
        unsafe extern "C" fn(thread_ptr: *mut TX_THREAD_STRUCT, type_: UINT),
    >,
    pub tx_thread_suspension_sequence: ULONG,
}
pub type TX_THREAD = TX_THREAD_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_BLOCK_POOL_STRUCT {
    pub tx_block_pool_id: ULONG,
    pub tx_block_pool_name: *mut CHAR,
    pub tx_block_pool_available: UINT,
    pub tx_block_pool_total: UINT,
    pub tx_block_pool_available_list: *mut UCHAR,
    pub tx_block_pool_start: *mut UCHAR,
    pub tx_block_pool_size: ULONG,
    pub tx_block_pool_block_size: UINT,
    pub tx_block_pool_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_block_pool_suspended_count: UINT,
    pub tx_block_pool_created_next: *mut TX_BLOCK_POOL_STRUCT,
    pub tx_block_pool_created_previous: *mut TX_BLOCK_POOL_STRUCT,
}
pub type TX_BLOCK_POOL = TX_BLOCK_POOL_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_BYTE_POOL_STRUCT {
    pub tx_byte_pool_id: ULONG,
    pub tx_byte_pool_name: *mut CHAR,
    pub tx_byte_pool_available: ULONG,
    pub tx_byte_pool_fragments: UINT,
    pub tx_byte_pool_list: *mut UCHAR,
    pub tx_byte_pool_search: *mut UCHAR,
    pub tx_byte_pool_start: *mut UCHAR,
    pub tx_byte_pool_size: ULONG,
    pub tx_byte_pool_owner: *mut TX_THREAD_STRUCT,
    pub tx_byte_pool_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_byte_pool_suspended_count: UINT,
    pub tx_byte_pool_created_next: *mut TX_BYTE_POOL_STRUCT,
    pub tx_byte_pool_created_previous: *mut TX_BYTE_POOL_STRUCT,
}
pub type TX_BYTE_POOL = TX_BYTE_POOL_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_EVENT_FLAGS_GROUP_STRUCT {
    pub tx_event_flags_group_id: ULONG,
    pub tx_event_flags_group_name: *mut CHAR,
    pub tx_event_flags_group_current: ULONG,
    pub tx_event_flags_group_reset_search: UINT,
    pub tx_event_flags_group_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_event_flags_group_suspended_count: UINT,
    pub tx_event_flags_group_created_next: *mut TX_EVENT_FLAGS_GROUP_STRUCT,
    pub tx_event_flags_group_created_previous: *mut TX_EVENT_FLAGS_GROUP_STRUCT,
    pub tx_event_flags_group_delayed_clear: ULONG,
    pub tx_event_flags_group_set_notify:
        ::core::option::Option<unsafe extern "C" fn(group_ptr: *mut TX_EVENT_FLAGS_GROUP_STRUCT)>,
}
pub type TX_EVENT_FLAGS_GROUP = TX_EVENT_FLAGS_GROUP_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_MUTEX_STRUCT {
    pub tx_mutex_id: ULONG,
    pub tx_mutex_name: *mut CHAR,
    pub tx_mutex_ownership_count: UINT,
    pub tx_mutex_owner: *mut TX_THREAD,
    pub tx_mutex_inherit: UINT,
    pub tx_mutex_original_priority: UINT,
    pub tx_mutex_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_mutex_suspended_count: UINT,
    pub tx_mutex_created_next: *mut TX_MUTEX_STRUCT,
    pub tx_mutex_created_previous: *mut TX_MUTEX_STRUCT,
    pub tx_mutex_highest_priority_waiting: UINT,
    pub tx_mutex_owned_next: *mut TX_MUTEX_STRUCT,
    pub tx_mutex_owned_previous: *mut TX_MUTEX_STRUCT,
}
pub type TX_MUTEX = TX_MUTEX_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_QUEUE_STRUCT {
    pub tx_queue_id: ULONG,
    pub tx_queue_name: *mut CHAR,
    pub tx_queue_message_size: UINT,
    pub tx_queue_capacity: UINT,
    pub tx_queue_enqueued: UINT,
    pub tx_queue_available_storage: UINT,
    pub tx_queue_start: *mut ULONG,
    pub tx_queue_end: *mut ULONG,
    pub tx_queue_read: *mut ULONG,
    pub tx_queue_write: *mut ULONG,
    pub tx_queue_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_queue_suspended_count: UINT,
    pub tx_queue_created_next: *mut TX_QUEUE_STRUCT,
    pub tx_queue_created_previous: *mut TX_QUEUE_STRUCT,
    pub tx_queue_send_notify:
        ::core::option::Option<unsafe extern "C" fn(queue_ptr: *mut TX_QUEUE_STRUCT)>,
}
pub type TX_QUEUE = TX_QUEUE_STRUCT;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct TX_SEMAPHORE_STRUCT {
    pub tx_semaphore_id: ULONG,
    pub tx_semaphore_name: *mut CHAR,
    pub tx_semaphore_count: ULONG,
    pub tx_semaphore_suspension_list: *mut TX_THREAD_STRUCT,
    pub tx_semaphore_suspended_count: UINT,
    pub tx_semaphore_created_next: *mut TX_SEMAPHORE_STRUCT,
    pub tx_semaphore_created_previous: *mut TX_SEMAPHORE_STRUCT,
    pub tx_semaphore_put_notify:
        ::core::option::Option<unsafe extern "C" fn(semaphore_ptr: *mut TX_SEMAPHORE_STRUCT)>,
}
pub type TX_SEMAPHORE = TX_SEMAPHORE_STRUCT;
extern "C" {
    pub fn _tx_block_allocate(
        pool_ptr: *mut TX_BLOCK_POOL,
        block_ptr: *mut *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_create(
        pool_ptr: *mut TX_BLOCK_POOL,
        name_ptr: *mut CHAR,
        block_size: ULONG,
        pool_start: *mut ::core::ffi::c_void,
        pool_size: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_delete(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_info_get(
        pool_ptr: *mut TX_BLOCK_POOL,
        name: *mut *mut CHAR,
        available_blocks: *mut ULONG,
        total_blocks: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BLOCK_POOL,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_performance_info_get(
        pool_ptr: *mut TX_BLOCK_POOL,
        allocates: *mut ULONG,
        releases: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_performance_system_info_get(
        allocates: *mut ULONG,
        releases: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_block_pool_prioritize(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;
}
extern "C" {
    pub fn _tx_block_release(block_ptr: *mut ::core::ffi::c_void) -> UINT;
}
extern "C" {
    pub fn _txe_block_allocate(
        pool_ptr: *mut TX_BLOCK_POOL,
        block_ptr: *mut *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_block_pool_create(
        pool_ptr: *mut TX_BLOCK_POOL,
        name_ptr: *mut CHAR,
        block_size: ULONG,
        pool_start: *mut ::core::ffi::c_void,
        pool_size: ULONG,
        pool_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_block_pool_delete(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;
}
extern "C" {
    pub fn _txe_block_pool_info_get(
        pool_ptr: *mut TX_BLOCK_POOL,
        name: *mut *mut CHAR,
        available_blocks: *mut ULONG,
        total_blocks: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BLOCK_POOL,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_block_pool_prioritize(pool_ptr: *mut TX_BLOCK_POOL) -> UINT;
}
extern "C" {
    pub fn _txe_block_release(block_ptr: *mut ::core::ffi::c_void) -> UINT;
}
extern "C" {
    pub fn _tx_byte_allocate(
        pool_ptr: *mut TX_BYTE_POOL,
        memory_ptr: *mut *mut ::core::ffi::c_void,
        memory_size: ULONG,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_create(
        pool_ptr: *mut TX_BYTE_POOL,
        name_ptr: *mut CHAR,
        pool_start: *mut ::core::ffi::c_void,
        pool_size: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_delete(pool_ptr: *mut TX_BYTE_POOL) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_info_get(
        pool_ptr: *mut TX_BYTE_POOL,
        name: *mut *mut CHAR,
        available_bytes: *mut ULONG,
        fragments: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BYTE_POOL,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_performance_info_get(
        pool_ptr: *mut TX_BYTE_POOL,
        allocates: *mut ULONG,
        releases: *mut ULONG,
        fragments_searched: *mut ULONG,
        merges: *mut ULONG,
        splits: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_performance_system_info_get(
        allocates: *mut ULONG,
        releases: *mut ULONG,
        fragments_searched: *mut ULONG,
        merges: *mut ULONG,
        splits: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_byte_pool_prioritize(pool_ptr: *mut TX_BYTE_POOL) -> UINT;
}
extern "C" {
    pub fn _tx_byte_release(memory_ptr: *mut ::core::ffi::c_void) -> UINT;
}
extern "C" {
    pub fn _txe_byte_allocate(
        pool_ptr: *mut TX_BYTE_POOL,
        memory_ptr: *mut *mut ::core::ffi::c_void,
        memory_size: ULONG,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_byte_pool_create(
        pool_ptr: *mut TX_BYTE_POOL,
        name_ptr: *mut CHAR,
        pool_start: *mut ::core::ffi::c_void,
        pool_size: ULONG,
        pool_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_byte_pool_delete(pool_ptr: *mut TX_BYTE_POOL) -> UINT;
}
extern "C" {
    pub fn _txe_byte_pool_info_get(
        pool_ptr: *mut TX_BYTE_POOL,
        name: *mut *mut CHAR,
        available_bytes: *mut ULONG,
        fragments: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_pool: *mut *mut TX_BYTE_POOL,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_byte_pool_prioritize(pool_ptr: *mut TX_BYTE_POOL) -> UINT;
}
extern "C" {
    pub fn _txe_byte_release(memory_ptr: *mut ::core::ffi::c_void) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_create(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        name_ptr: *mut CHAR,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_delete(group_ptr: *mut TX_EVENT_FLAGS_GROUP) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        requested_flags: ULONG,
        get_option: UINT,
        actual_flags_ptr: *mut ULONG,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_info_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        name: *mut *mut CHAR,
        current_flags: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_group: *mut *mut TX_EVENT_FLAGS_GROUP,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_performance_info_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        sets: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_performance_system_info_get(
        sets: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_set(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        flags_to_set: ULONG,
        set_option: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_event_flags_set_notify(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        events_set_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_group_ptr: *mut TX_EVENT_FLAGS_GROUP),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_create(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        name_ptr: *mut CHAR,
        event_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_delete(group_ptr: *mut TX_EVENT_FLAGS_GROUP) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        requested_flags: ULONG,
        get_option: UINT,
        actual_flags_ptr: *mut ULONG,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_info_get(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        name: *mut *mut CHAR,
        current_flags: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_group: *mut *mut TX_EVENT_FLAGS_GROUP,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_set(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        flags_to_set: ULONG,
        set_option: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_event_flags_set_notify(
        group_ptr: *mut TX_EVENT_FLAGS_GROUP,
        events_set_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_group_ptr: *mut TX_EVENT_FLAGS_GROUP),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_initialize_kernel_enter();
}
extern "C" {
    pub fn _tx_mutex_create(mutex_ptr: *mut TX_MUTEX, name_ptr: *mut CHAR, inherit: UINT) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_delete(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_get(mutex_ptr: *mut TX_MUTEX, wait_option: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_info_get(
        mutex_ptr: *mut TX_MUTEX,
        name: *mut *mut CHAR,
        count: *mut ULONG,
        owner: *mut *mut TX_THREAD,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_mutex: *mut *mut TX_MUTEX,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_performance_info_get(
        mutex_ptr: *mut TX_MUTEX,
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
        inversions: *mut ULONG,
        inheritances: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_performance_system_info_get(
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
        inversions: *mut ULONG,
        inheritances: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_prioritize(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _tx_mutex_put(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_create(
        mutex_ptr: *mut TX_MUTEX,
        name_ptr: *mut CHAR,
        inherit: UINT,
        mutex_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_delete(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_get(mutex_ptr: *mut TX_MUTEX, wait_option: ULONG) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_info_get(
        mutex_ptr: *mut TX_MUTEX,
        name: *mut *mut CHAR,
        count: *mut ULONG,
        owner: *mut *mut TX_THREAD,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_mutex: *mut *mut TX_MUTEX,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_prioritize(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _txe_mutex_put(mutex_ptr: *mut TX_MUTEX) -> UINT;
}
extern "C" {
    pub fn _tx_queue_create(
        queue_ptr: *mut TX_QUEUE,
        name_ptr: *mut CHAR,
        message_size: UINT,
        queue_start: *mut ::core::ffi::c_void,
        queue_size: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_delete(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _tx_queue_flush(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _tx_queue_info_get(
        queue_ptr: *mut TX_QUEUE,
        name: *mut *mut CHAR,
        enqueued: *mut ULONG,
        available_storage: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_queue: *mut *mut TX_QUEUE,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_performance_info_get(
        queue_ptr: *mut TX_QUEUE,
        messages_sent: *mut ULONG,
        messages_received: *mut ULONG,
        empty_suspensions: *mut ULONG,
        full_suspensions: *mut ULONG,
        full_errors: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_performance_system_info_get(
        messages_sent: *mut ULONG,
        messages_received: *mut ULONG,
        empty_suspensions: *mut ULONG,
        full_suspensions: *mut ULONG,
        full_errors: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_prioritize(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _tx_queue_receive(
        queue_ptr: *mut TX_QUEUE,
        destination_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_send_notify(
        queue_ptr: *mut TX_QUEUE,
        queue_send_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_queue_ptr: *mut TX_QUEUE),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_queue_front_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_create(
        queue_ptr: *mut TX_QUEUE,
        name_ptr: *mut CHAR,
        message_size: UINT,
        queue_start: *mut ::core::ffi::c_void,
        queue_size: ULONG,
        queue_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_delete(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _txe_queue_flush(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _txe_queue_info_get(
        queue_ptr: *mut TX_QUEUE,
        name: *mut *mut CHAR,
        enqueued: *mut ULONG,
        available_storage: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_queue: *mut *mut TX_QUEUE,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_prioritize(queue_ptr: *mut TX_QUEUE) -> UINT;
}
extern "C" {
    pub fn _txe_queue_receive(
        queue_ptr: *mut TX_QUEUE,
        destination_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_send_notify(
        queue_ptr: *mut TX_QUEUE,
        queue_send_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_queue_ptr: *mut TX_QUEUE),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_queue_front_send(
        queue_ptr: *mut TX_QUEUE,
        source_ptr: *mut ::core::ffi::c_void,
        wait_option: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_ceiling_put(semaphore_ptr: *mut TX_SEMAPHORE, ceiling: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_create(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name_ptr: *mut CHAR,
        initial_count: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_delete(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_get(semaphore_ptr: *mut TX_SEMAPHORE, wait_option: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_info_get(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name: *mut *mut CHAR,
        current_value: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_semaphore: *mut *mut TX_SEMAPHORE,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_performance_info_get(
        semaphore_ptr: *mut TX_SEMAPHORE,
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_performance_system_info_get(
        puts: *mut ULONG,
        gets: *mut ULONG,
        suspensions: *mut ULONG,
        timeouts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_prioritize(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_put(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _tx_semaphore_put_notify(
        semaphore_ptr: *mut TX_SEMAPHORE,
        semaphore_put_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_semaphore_ptr: *mut TX_SEMAPHORE),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_ceiling_put(semaphore_ptr: *mut TX_SEMAPHORE, ceiling: ULONG) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_create(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name_ptr: *mut CHAR,
        initial_count: ULONG,
        semaphore_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_delete(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_get(semaphore_ptr: *mut TX_SEMAPHORE, wait_option: ULONG) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_info_get(
        semaphore_ptr: *mut TX_SEMAPHORE,
        name: *mut *mut CHAR,
        current_value: *mut ULONG,
        first_suspended: *mut *mut TX_THREAD,
        suspended_count: *mut ULONG,
        next_semaphore: *mut *mut TX_SEMAPHORE,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_prioritize(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_put(semaphore_ptr: *mut TX_SEMAPHORE) -> UINT;
}
extern "C" {
    pub fn _txe_semaphore_put_notify(
        semaphore_ptr: *mut TX_SEMAPHORE,
        semaphore_put_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_semaphore_ptr: *mut TX_SEMAPHORE),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_context_save();
}
extern "C" {
    pub fn _tx_thread_context_restore();
}
extern "C" {
    pub fn _tx_thread_create(
        thread_ptr: *mut TX_THREAD,
        name_ptr: *mut CHAR,
        entry_function: ::core::option::Option<unsafe extern "C" fn(entry_input: ULONG)>,
        entry_input: ULONG,
        stack_start: *mut ::core::ffi::c_void,
        stack_size: ULONG,
        priority: UINT,
        preempt_threshold: UINT,
        time_slice: ULONG,
        auto_start: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_delete(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_thread_entry_exit_notify(
        thread_ptr: *mut TX_THREAD,
        thread_entry_exit_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_thread_ptr: *mut TX_THREAD, type_: UINT),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_identify() -> *mut TX_THREAD;
}
extern "C" {
    pub fn _tx_thread_info_get(
        thread_ptr: *mut TX_THREAD,
        name: *mut *mut CHAR,
        state: *mut UINT,
        run_count: *mut ULONG,
        priority: *mut UINT,
        preemption_threshold: *mut UINT,
        time_slice: *mut ULONG,
        next_thread: *mut *mut TX_THREAD,
        next_suspended_thread: *mut *mut TX_THREAD,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_interrupt_control(new_posture: UINT) -> UINT;
}
extern "C" {
    pub fn _tx_thread_performance_info_get(
        thread_ptr: *mut TX_THREAD,
        resumptions: *mut ULONG,
        suspensions: *mut ULONG,
        solicited_preemptions: *mut ULONG,
        interrupt_preemptions: *mut ULONG,
        priority_inversions: *mut ULONG,
        time_slices: *mut ULONG,
        relinquishes: *mut ULONG,
        timeouts: *mut ULONG,
        wait_aborts: *mut ULONG,
        last_preempted_by: *mut *mut TX_THREAD,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_performance_system_info_get(
        resumptions: *mut ULONG,
        suspensions: *mut ULONG,
        solicited_preemptions: *mut ULONG,
        interrupt_preemptions: *mut ULONG,
        priority_inversions: *mut ULONG,
        time_slices: *mut ULONG,
        relinquishes: *mut ULONG,
        timeouts: *mut ULONG,
        wait_aborts: *mut ULONG,
        non_idle_returns: *mut ULONG,
        idle_returns: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_preemption_change(
        thread_ptr: *mut TX_THREAD,
        new_threshold: UINT,
        old_threshold: *mut UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_priority_change(
        thread_ptr: *mut TX_THREAD,
        new_priority: UINT,
        old_priority: *mut UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_relinquish();
}
extern "C" {
    pub fn _tx_thread_reset(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_thread_resume(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_thread_sleep(timer_ticks: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_thread_stack_error_notify(
        stack_error_handler: ::core::option::Option<
            unsafe extern "C" fn(thread_ptr: *mut TX_THREAD),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_suspend(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_thread_terminate(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_thread_time_slice_change(
        thread_ptr: *mut TX_THREAD,
        new_time_slice: ULONG,
        old_time_slice: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_thread_wait_abort(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_create(
        thread_ptr: *mut TX_THREAD,
        name_ptr: *mut CHAR,
        entry_function: ::core::option::Option<unsafe extern "C" fn(entry_input: ULONG)>,
        entry_input: ULONG,
        stack_start: *mut ::core::ffi::c_void,
        stack_size: ULONG,
        priority: UINT,
        preempt_threshold: UINT,
        time_slice: ULONG,
        auto_start: UINT,
        thread_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_delete(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_entry_exit_notify(
        thread_ptr: *mut TX_THREAD,
        thread_entry_exit_notify: ::core::option::Option<
            unsafe extern "C" fn(notify_thread_ptr: *mut TX_THREAD, type_: UINT),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_info_get(
        thread_ptr: *mut TX_THREAD,
        name: *mut *mut CHAR,
        state: *mut UINT,
        run_count: *mut ULONG,
        priority: *mut UINT,
        preemption_threshold: *mut UINT,
        time_slice: *mut ULONG,
        next_thread: *mut *mut TX_THREAD,
        next_suspended_thread: *mut *mut TX_THREAD,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_preemption_change(
        thread_ptr: *mut TX_THREAD,
        new_threshold: UINT,
        old_threshold: *mut UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_priority_change(
        thread_ptr: *mut TX_THREAD,
        new_priority: UINT,
        old_priority: *mut UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_relinquish();
}
extern "C" {
    pub fn _txe_thread_reset(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_resume(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_suspend(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_terminate(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _txe_thread_time_slice_change(
        thread_ptr: *mut TX_THREAD,
        new_time_slice: ULONG,
        old_time_slice: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_thread_wait_abort(thread_ptr: *mut TX_THREAD) -> UINT;
}
extern "C" {
    pub fn _tx_timer_activate(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _tx_timer_change(
        timer_ptr: *mut TX_TIMER,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_timer_create(
        timer_ptr: *mut TX_TIMER,
        name_ptr: *mut CHAR,
        expiration_function: ::core::option::Option<unsafe extern "C" fn(input: ULONG)>,
        expiration_input: ULONG,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
        auto_activate: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_timer_deactivate(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _tx_timer_delete(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _tx_timer_info_get(
        timer_ptr: *mut TX_TIMER,
        name: *mut *mut CHAR,
        active: *mut UINT,
        remaining_ticks: *mut ULONG,
        reschedule_ticks: *mut ULONG,
        next_timer: *mut *mut TX_TIMER,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_timer_performance_info_get(
        timer_ptr: *mut TX_TIMER,
        activates: *mut ULONG,
        reactivates: *mut ULONG,
        deactivates: *mut ULONG,
        expirations: *mut ULONG,
        expiration_adjusts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_timer_performance_system_info_get(
        activates: *mut ULONG,
        reactivates: *mut ULONG,
        deactivates: *mut ULONG,
        expirations: *mut ULONG,
        expiration_adjusts: *mut ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_time_get() -> ULONG;
}
extern "C" {
    pub fn _tx_time_set(new_time: ULONG);
}
extern "C" {
    pub fn _txe_timer_activate(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _txe_timer_change(
        timer_ptr: *mut TX_TIMER,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_timer_create(
        timer_ptr: *mut TX_TIMER,
        name_ptr: *mut CHAR,
        expiration_function: ::core::option::Option<unsafe extern "C" fn(input: ULONG)>,
        expiration_input: ULONG,
        initial_ticks: ULONG,
        reschedule_ticks: ULONG,
        auto_activate: UINT,
        timer_control_block_size: UINT,
    ) -> UINT;
}
extern "C" {
    pub fn _txe_timer_deactivate(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _txe_timer_delete(timer_ptr: *mut TX_TIMER) -> UINT;
}
extern "C" {
    pub fn _txe_timer_info_get(
        timer_ptr: *mut TX_TIMER,
        name: *mut *mut CHAR,
        active: *mut UINT,
        remaining_ticks: *mut ULONG,
        reschedule_ticks: *mut ULONG,
        next_timer: *mut *mut TX_TIMER,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_trace_enable(
        trace_buffer_start: *mut ::core::ffi::c_void,
        trace_buffer_size: ULONG,
        registry_entries: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_trace_event_filter(event_filter_bits: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_trace_event_unfilter(event_unfilter_bits: ULONG) -> UINT;
}
extern "C" {
    pub fn _tx_trace_disable() -> UINT;
}
extern "C" {
    pub fn _tx_trace_isr_enter_insert(isr_id: ULONG);
}
extern "C" {
    pub fn _tx_trace_isr_exit_insert(isr_id: ULONG);
}
extern "C" {
    pub fn _tx_trace_buffer_full_notify(
        full_buffer_callback: ::core::option::Option<
            unsafe extern "C" fn(buffer: *mut ::core::ffi::c_void),
        >,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_trace_user_event_insert(
        event_id: ULONG,
        info_field_1: ULONG,
        info_field_2: ULONG,
        info_field_3: ULONG,
        info_field_4: ULONG,
    ) -> UINT;
}
extern "C" {
    pub fn _tx_trace_interrupt_control(new_posture: UINT) -> UINT;
}
