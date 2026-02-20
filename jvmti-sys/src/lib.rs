#![doc(html_root_url = "https://docs.rs/jvmti-sys/0.4.1")]
#![allow(non_snake_case, non_camel_case_types)]
#![warn(rust_2018_idioms, missing_debug_implementations)]
#![no_std]

use core::ffi::c_char;
use core::ffi::c_int;
use core::ffi::c_uchar;
use core::ffi::c_void;

use jni_sys::jboolean;
use jni_sys::jchar;
use jni_sys::jclass;
use jni_sys::jdouble;
use jni_sys::jfieldID;
use jni_sys::jfloat;
use jni_sys::jint;
use jni_sys::jlong;
use jni_sys::jmethodID;
use jni_sys::jobject;
use jni_sys::jvalue;
use jni_sys::JNIEnv;
use jni_sys::JNINativeInterface_;
use jni_sys::JavaVM;
use jni_sys_macros::jni_to_union;

pub type enum_t = c_int;

pub const JVMTI_VERSION_1: enum_t = 0x30010000;
pub const JVMTI_VERSION_1_0: enum_t = 0x30010000;
pub const JVMTI_VERSION_1_1: enum_t = 0x30010100;
pub const JVMTI_VERSION_1_2: enum_t = 0x30010200;
pub const JVMTI_VERSION_9: enum_t = 0x30090000;
pub const JVMTI_VERSION_11: enum_t = 0x300B0000;
pub const JVMTI_VERSION_19: enum_t = 0x30130000;
pub const JVMTI_VERSION_21: enum_t = 0x30150000;
pub const JVMTI_VERSION: enum_t = 0x30190000;

pub type Agent_OnLoad =
    unsafe extern "C" fn(vm: *mut JavaVM, options: *mut c_char, reserved: *mut c_void) -> jint;
pub type Agent_OnAttach =
    unsafe extern "C" fn(vm: *mut JavaVM, options: *mut c_char, reserved: *mut c_void) -> jint;
pub type Agent_OnUnload = unsafe extern "C" fn(vm: *mut JavaVM);

pub type jvmtiEnv = *const jvmtiInterface_1_;
pub type jthread = jobject;
pub type jthreadGroup = jobject;
pub type jlocation = jlong;
#[derive(Debug)]
pub enum _jrawMonitorID {}
pub type jrawMonitorID = *mut _jrawMonitorID;
pub type jniNativeInterface = JNINativeInterface_;

pub const JVMTI_THREAD_STATE_ALIVE: enum_t = 0x0001;
pub const JVMTI_THREAD_STATE_TERMINATED: enum_t = 0x0002;
pub const JVMTI_THREAD_STATE_RUNNABLE: enum_t = 0x0004;
pub const JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER: enum_t = 0x0400;
pub const JVMTI_THREAD_STATE_WAITING: enum_t = 0x0080;
pub const JVMTI_THREAD_STATE_WAITING_INDEFINITELY: enum_t = 0x0010;
pub const JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT: enum_t = 0x0020;
pub const JVMTI_THREAD_STATE_SLEEPING: enum_t = 0x0040;
pub const JVMTI_THREAD_STATE_IN_OBJECT_WAIT: enum_t = 0x0100;
pub const JVMTI_THREAD_STATE_PARKED: enum_t = 0x0200;
pub const JVMTI_THREAD_STATE_SUSPENDED: enum_t = 0x100000;
pub const JVMTI_THREAD_STATE_INTERRUPTED: enum_t = 0x200000;
pub const JVMTI_THREAD_STATE_IN_NATIVE: enum_t = 0x400000;
pub const JVMTI_THREAD_STATE_VENDOR_1: enum_t = 0x10000000;
pub const JVMTI_THREAD_STATE_VENDOR_2: enum_t = 0x20000000;
pub const JVMTI_THREAD_STATE_VENDOR_3: enum_t = 0x40000000;

pub const JVMTI_JAVA_LANG_THREAD_STATE_MASK: enum_t = JVMTI_THREAD_STATE_TERMINATED
    | JVMTI_THREAD_STATE_ALIVE
    | JVMTI_THREAD_STATE_RUNNABLE
    | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER
    | JVMTI_THREAD_STATE_WAITING
    | JVMTI_THREAD_STATE_WAITING_INDEFINITELY
    | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;
pub const JVMTI_JAVA_LANG_THREAD_STATE_NEW: enum_t = 0;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TERMINATED: enum_t = JVMTI_THREAD_STATE_TERMINATED;
pub const JVMTI_JAVA_LANG_THREAD_STATE_RUNNABLE: enum_t =
    JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_RUNNABLE;
pub const JVMTI_JAVA_LANG_THREAD_STATE_BLOCKED: enum_t =
    JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_BLOCKED_ON_MONITOR_ENTER;
pub const JVMTI_JAVA_LANG_THREAD_STATE_WAITING: enum_t =
    JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_INDEFINITELY;
pub const JVMTI_JAVA_LANG_THREAD_STATE_TIMED_WAITING: enum_t =
    JVMTI_THREAD_STATE_ALIVE | JVMTI_THREAD_STATE_WAITING | JVMTI_THREAD_STATE_WAITING_WITH_TIMEOUT;

pub const JVMTI_THREAD_MIN_PRIORITY: enum_t = 1;
pub const JVMTI_THREAD_NORM_PRIORITY: enum_t = 5;
pub const JVMTI_THREAD_MAX_PRIORITY: enum_t = 10;

pub const JVMTI_HEAP_FILTER_TAGGED: enum_t = 0x4;
pub const JVMTI_HEAP_FILTER_UNTAGGED: enum_t = 0x8;
pub const JVMTI_HEAP_FILTER_CLASS_TAGGED: enum_t = 0x10;
pub const JVMTI_HEAP_FILTER_CLASS_UNTAGGED: enum_t = 0x20;

pub const JVMTI_VISIT_OBJECTS: enum_t = 0x100;
pub const JVMTI_VISIT_ABORT: enum_t = 0x8000;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiHeapReferenceKind {
    JVMTI_HEAP_REFERENCE_CLASS = 1,
    JVMTI_HEAP_REFERENCE_FIELD = 2,
    JVMTI_HEAP_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_HEAP_REFERENCE_CLASS_LOADER = 4,
    JVMTI_HEAP_REFERENCE_SIGNERS = 5,
    JVMTI_HEAP_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_HEAP_REFERENCE_INTERFACE = 7,
    JVMTI_HEAP_REFERENCE_STATIC_FIELD = 8,
    JVMTI_HEAP_REFERENCE_CONSTANT_POOL = 9,
    JVMTI_HEAP_REFERENCE_SUPERCLASS = 10,
    JVMTI_HEAP_REFERENCE_JNI_GLOBAL = 21,
    JVMTI_HEAP_REFERENCE_SYSTEM_CLASS = 22,
    JVMTI_HEAP_REFERENCE_MONITOR = 23,
    JVMTI_HEAP_REFERENCE_STACK_LOCAL = 24,
    JVMTI_HEAP_REFERENCE_JNI_LOCAL = 25,
    JVMTI_HEAP_REFERENCE_THREAD = 26,
    JVMTI_HEAP_REFERENCE_OTHER = 27,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiPrimitiveType {
    JVMTI_PRIMITIVE_TYPE_BOOLEAN = 90,
    JVMTI_PRIMITIVE_TYPE_BYTE = 66,
    JVMTI_PRIMITIVE_TYPE_CHAR = 67,
    JVMTI_PRIMITIVE_TYPE_SHORT = 83,
    JVMTI_PRIMITIVE_TYPE_INT = 73,
    JVMTI_PRIMITIVE_TYPE_LONG = 74,
    JVMTI_PRIMITIVE_TYPE_FLOAT = 70,
    JVMTI_PRIMITIVE_TYPE_DOUBLE = 68,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiHeapObjectFilter {
    JVMTI_HEAP_OBJECT_TAGGED = 1,
    JVMTI_HEAP_OBJECT_UNTAGGED = 2,
    JVMTI_HEAP_OBJECT_EITHER = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiHeapRootKind {
    JVMTI_HEAP_ROOT_JNI_GLOBAL = 1,
    JVMTI_HEAP_ROOT_SYSTEM_CLASS = 2,
    JVMTI_HEAP_ROOT_MONITOR = 3,
    JVMTI_HEAP_ROOT_STACK_LOCAL = 4,
    JVMTI_HEAP_ROOT_JNI_LOCAL = 5,
    JVMTI_HEAP_ROOT_THREAD = 6,
    JVMTI_HEAP_ROOT_OTHER = 7,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiObjectReferenceKind {
    JVMTI_REFERENCE_CLASS = 1,
    JVMTI_REFERENCE_FIELD = 2,
    JVMTI_REFERENCE_ARRAY_ELEMENT = 3,
    JVMTI_REFERENCE_CLASS_LOADER = 4,
    JVMTI_REFERENCE_SIGNERS = 5,
    JVMTI_REFERENCE_PROTECTION_DOMAIN = 6,
    JVMTI_REFERENCE_INTERFACE = 7,
    JVMTI_REFERENCE_STATIC_FIELD = 8,
    JVMTI_REFERENCE_CONSTANT_POOL = 9,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiIterationControl {
    JVMTI_ITERATION_CONTINUE = 1,
    JVMTI_ITERATION_IGNORE = 2,
    JVMTI_ITERATION_ABORT = 0,
}

pub const JVMTI_CLASS_STATUS_VERIFIED: enum_t = 1;
pub const JVMTI_CLASS_STATUS_PREPARED: enum_t = 2;
pub const JVMTI_CLASS_STATUS_INITIALIZED: enum_t = 4;
pub const JVMTI_CLASS_STATUS_ERROR: enum_t = 8;
pub const JVMTI_CLASS_STATUS_ARRAY: enum_t = 16;
pub const JVMTI_CLASS_STATUS_PRIMITIVE: enum_t = 32;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiEventMode {
    JVMTI_ENABLE = 1,
    JVMTI_DISABLE = 0,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiParamTypes {
    JVMTI_TYPE_JBYTE = 101,
    JVMTI_TYPE_JCHAR = 102,
    JVMTI_TYPE_JSHORT = 103,
    JVMTI_TYPE_JINT = 104,
    JVMTI_TYPE_JLONG = 105,
    JVMTI_TYPE_JFLOAT = 106,
    JVMTI_TYPE_JDOUBLE = 107,
    JVMTI_TYPE_JBOOLEAN = 108,
    JVMTI_TYPE_JOBJECT = 109,
    JVMTI_TYPE_JTHREAD = 110,
    JVMTI_TYPE_JCLASS = 111,
    JVMTI_TYPE_JVALUE = 112,
    JVMTI_TYPE_JFIELDID = 113,
    JVMTI_TYPE_JMETHODID = 114,
    JVMTI_TYPE_CCHAR = 115,
    JVMTI_TYPE_CVOID = 116,
    JVMTI_TYPE_JNIENV = 117,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiParamKind {
    JVMTI_KIND_IN = 91,
    JVMTI_KIND_IN_PTR = 92,
    JVMTI_KIND_IN_BUF = 93,
    JVMTI_KIND_ALLOC_BUF = 94,
    JVMTI_KIND_ALLOC_ALLOC_BUF = 95,
    JVMTI_KIND_OUT = 96,
    JVMTI_KIND_OUT_BUF = 97,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiTimerKind {
    JVMTI_TIMER_USER_CPU = 30,
    JVMTI_TIMER_TOTAL_CPU = 31,
    JVMTI_TIMER_ELAPSED = 32,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiPhase {
    JVMTI_PHASE_ONLOAD = 1,
    JVMTI_PHASE_PRIMORDIAL = 2,
    JVMTI_PHASE_START = 6,
    JVMTI_PHASE_LIVE = 4,
    JVMTI_PHASE_DEAD = 8,
}

pub const JVMTI_VERSION_INTERFACE_JNI: enum_t = 0x00000000;
pub const JVMTI_VERSION_INTERFACE_JVMTI: enum_t = 0x30000000;

pub const JVMTI_VERSION_MASK_INTERFACE_TYPE: enum_t = 0x70000000;
pub const JVMTI_VERSION_MASK_MAJOR: enum_t = 0x0FFF0000;
pub const JVMTI_VERSION_MASK_MINOR: enum_t = 0x0000FF00;
pub const JVMTI_VERSION_MASK_MICRO: enum_t = 0x000000FF;

pub const JVMTI_VERSION_SHIFT_MAJOR: enum_t = 16;
pub const JVMTI_VERSION_SHIFT_MINOR: enum_t = 8;
pub const JVMTI_VERSION_SHIFT_MICRO: enum_t = 0;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiVerboseFlag {
    JVMTI_VERBOSE_OTHER = 0,
    JVMTI_VERBOSE_GC = 1,
    JVMTI_VERBOSE_CLASS = 2,
    JVMTI_VERBOSE_JNI = 4,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiJlocationFormat {
    JVMTI_JLOCATION_JVMBCI = 1,
    JVMTI_JLOCATION_MACHINEPC = 2,
    JVMTI_JLOCATION_OTHER = 0,
}

pub const JVMTI_RESOURCE_EXHAUSTED_OOM_ERROR: enum_t = 0x0001;
pub const JVMTI_RESOURCE_EXHAUSTED_JAVA_HEAP: enum_t = 0x0002;
pub const JVMTI_RESOURCE_EXHAUSTED_THREADS: enum_t = 0x0004;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiError {
    JVMTI_ERROR_NONE = 0,
    JVMTI_ERROR_INVALID_THREAD = 10,
    JVMTI_ERROR_INVALID_THREAD_GROUP = 11,
    JVMTI_ERROR_INVALID_PRIORITY = 12,
    JVMTI_ERROR_THREAD_NOT_SUSPENDED = 13,
    JVMTI_ERROR_THREAD_SUSPENDED = 14,
    JVMTI_ERROR_THREAD_NOT_ALIVE = 15,
    JVMTI_ERROR_INVALID_OBJECT = 20,
    JVMTI_ERROR_INVALID_CLASS = 21,
    JVMTI_ERROR_CLASS_NOT_PREPARED = 22,
    JVMTI_ERROR_INVALID_METHODID = 23,
    JVMTI_ERROR_INVALID_LOCATION = 24,
    JVMTI_ERROR_INVALID_FIELDID = 25,
    JVMTI_ERROR_INVALID_MODULE = 26,
    JVMTI_ERROR_NO_MORE_FRAMES = 31,
    JVMTI_ERROR_OPAQUE_FRAME = 32,
    JVMTI_ERROR_TYPE_MISMATCH = 34,
    JVMTI_ERROR_INVALID_SLOT = 35,
    JVMTI_ERROR_DUPLICATE = 40,
    JVMTI_ERROR_NOT_FOUND = 41,
    JVMTI_ERROR_INVALID_MONITOR = 50,
    JVMTI_ERROR_NOT_MONITOR_OWNER = 51,
    JVMTI_ERROR_INTERRUPT = 52,
    JVMTI_ERROR_INVALID_CLASS_FORMAT = 60,
    JVMTI_ERROR_CIRCULAR_CLASS_DEFINITION = 61,
    JVMTI_ERROR_FAILS_VERIFICATION = 62,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_ADDED = 63,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_SCHEMA_CHANGED = 64,
    JVMTI_ERROR_INVALID_TYPESTATE = 65,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_HIERARCHY_CHANGED = 66,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_DELETED = 67,
    JVMTI_ERROR_UNSUPPORTED_VERSION = 68,
    JVMTI_ERROR_NAMES_DONT_MATCH = 69,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_MODIFIERS_CHANGED = 70,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_METHOD_MODIFIERS_CHANGED = 71,
    JVMTI_ERROR_UNSUPPORTED_REDEFINITION_CLASS_ATTRIBUTE_CHANGED = 72,
    JVMTI_ERROR_UNSUPPORTED_OPERATION = 73,
    JVMTI_ERROR_UNMODIFIABLE_CLASS = 79,
    JVMTI_ERROR_UNMODIFIABLE_MODULE = 80,
    JVMTI_ERROR_NOT_AVAILABLE = 98,
    JVMTI_ERROR_MUST_POSSESS_CAPABILITY = 99,
    JVMTI_ERROR_NULL_POINTER = 100,
    JVMTI_ERROR_ABSENT_INFORMATION = 101,
    JVMTI_ERROR_INVALID_EVENT_TYPE = 102,
    JVMTI_ERROR_ILLEGAL_ARGUMENT = 103,
    JVMTI_ERROR_NATIVE_METHOD = 104,
    JVMTI_ERROR_CLASS_LOADER_UNSUPPORTED = 106,
    JVMTI_ERROR_OUT_OF_MEMORY = 110,
    JVMTI_ERROR_ACCESS_DENIED = 111,
    JVMTI_ERROR_WRONG_PHASE = 112,
    JVMTI_ERROR_INTERNAL = 113,
    JVMTI_ERROR_UNATTACHED_THREAD = 115,
    JVMTI_ERROR_INVALID_ENVIRONMENT = 116,
}

impl jvmtiError {
    pub const JVMTI_ERROR_MAX: Self = Self::JVMTI_ERROR_INVALID_ENVIRONMENT;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(C)]
pub enum jvmtiEvent {
    JVMTI_EVENT_VM_INIT = 50,
    JVMTI_EVENT_VM_DEATH = 51,
    JVMTI_EVENT_THREAD_START = 52,
    JVMTI_EVENT_THREAD_END = 53,
    JVMTI_EVENT_CLASS_FILE_LOAD_HOOK = 54,
    JVMTI_EVENT_CLASS_LOAD = 55,
    JVMTI_EVENT_CLASS_PREPARE = 56,
    JVMTI_EVENT_VM_START = 57,
    JVMTI_EVENT_EXCEPTION = 58,
    JVMTI_EVENT_EXCEPTION_CATCH = 59,
    JVMTI_EVENT_SINGLE_STEP = 60,
    JVMTI_EVENT_FRAME_POP = 61,
    JVMTI_EVENT_BREAKPOINT = 62,
    JVMTI_EVENT_FIELD_ACCESS = 63,
    JVMTI_EVENT_FIELD_MODIFICATION = 64,
    JVMTI_EVENT_METHOD_ENTRY = 65,
    JVMTI_EVENT_METHOD_EXIT = 66,
    JVMTI_EVENT_NATIVE_METHOD_BIND = 67,
    JVMTI_EVENT_COMPILED_METHOD_LOAD = 68,
    JVMTI_EVENT_COMPILED_METHOD_UNLOAD = 69,
    JVMTI_EVENT_DYNAMIC_CODE_GENERATED = 70,
    JVMTI_EVENT_DATA_DUMP_REQUEST = 71,
    JVMTI_EVENT_MONITOR_WAIT = 73,
    JVMTI_EVENT_MONITOR_WAITED = 74,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTER = 75,
    JVMTI_EVENT_MONITOR_CONTENDED_ENTERED = 76,
    JVMTI_EVENT_RESOURCE_EXHAUSTED = 80,
    JVMTI_EVENT_GARBAGE_COLLECTION_START = 81,
    JVMTI_EVENT_GARBAGE_COLLECTION_FINISH = 82,
    JVMTI_EVENT_OBJECT_FREE = 83,
    JVMTI_EVENT_VM_OBJECT_ALLOC = 84,
    JVMTI_EVENT_SAMPLED_OBJECT_ALLOC = 86,
    JVMTI_EVENT_VIRTUAL_THREAD_START = 87,
    JVMTI_EVENT_VIRTUAL_THREAD_END = 88,
}

impl jvmtiEvent {
    pub const JVMTI_MIN_EVENT_TYPE_VAL: Self = Self::JVMTI_EVENT_VM_INIT;
    pub const JVMTI_MAX_EVENT_TYPE_VAL: Self = Self::JVMTI_EVENT_VIRTUAL_THREAD_END;
}

pub type jvmtiStartFunction =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, arg: *mut c_void)>;

pub type jvmtiHeapIterationCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiHeapReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiHeapReferenceKind,
        reference_info: *const jvmtiHeapReferenceInfo,
        class_tag: jlong,
        referrer_class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag_ptr: *mut jlong,
        length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiPrimitiveFieldCallback = Option<
    unsafe extern "C" fn(
        kind: jvmtiHeapReferenceKind,
        info: *const jvmtiHeapReferenceInfo,
        object_class_tag: jlong,
        object_tag_ptr: *mut jlong,
        value: jvalue,
        value_type: jvmtiPrimitiveType,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiArrayPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        element_count: jint,
        element_type: jvmtiPrimitiveType,
        elements: *const c_void,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiStringPrimitiveValueCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        value: *const jchar,
        value_length: jint,
        user_data: *mut c_void,
    ) -> jint,
>;

pub type jvmtiReservedCallback = Option<unsafe extern "C" fn() -> jint>;

pub type jvmtiHeapObjectCallback = Option<
    unsafe extern "C" fn(
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiHeapRootCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiStackReferenceCallback = Option<
    unsafe extern "C" fn(
        root_kind: jvmtiHeapRootKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        thread_tag: jlong,
        depth: jint,
        method: jmethodID,
        slot: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiObjectReferenceCallback = Option<
    unsafe extern "C" fn(
        reference_kind: jvmtiObjectReferenceKind,
        class_tag: jlong,
        size: jlong,
        tag_ptr: *mut jlong,
        referrer_tag: jlong,
        referrer_index: jint,
        user_data: *mut c_void,
    ) -> jvmtiIterationControl,
>;

pub type jvmtiExtensionFunction =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...) -> jvmtiError>;

pub type jvmtiExtensionEvent = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, ...)>;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiThreadInfo {
    pub name: *mut c_char,
    pub priority: jint,
    pub is_daemon: jboolean,
    pub thread_group: jthreadGroup,
    pub context_class_loader: jobject,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiMonitorStackDepthInfo {
    pub monitor: jobject,
    pub stack_depth: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiThreadGroupInfo {
    pub parent: jthreadGroup,
    pub name: *mut c_char,
    pub max_priority: jint,
    pub is_daemon: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiFrameInfo {
    pub method: jmethodID,
    pub location: jlocation,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiStackInfo {
    pub thread: jthread,
    pub state: jint,
    pub frame_buffer: *mut jvmtiFrameInfo,
    pub frame_count: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoField {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoArray {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoConstantPool {
    pub index: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoStackLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
    pub location: jlocation,
    pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoJniLocal {
    pub thread_tag: jlong,
    pub thread_id: jlong,
    pub depth: jint,
    pub method: jmethodID,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiHeapReferenceInfoReserved {
    pub reserved1: jlong,
    pub reserved2: jlong,
    pub reserved3: jlong,
    pub reserved4: jlong,
    pub reserved5: jlong,
    pub reserved6: jlong,
    pub reserved7: jlong,
    pub reserved8: jlong,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union jvmtiHeapReferenceInfo {
    pub field: jvmtiHeapReferenceInfoField,
    pub array: jvmtiHeapReferenceInfoArray,
    pub constant_pool: jvmtiHeapReferenceInfoConstantPool,
    pub stack_local: jvmtiHeapReferenceInfoStackLocal,
    pub jni_local: jvmtiHeapReferenceInfoJniLocal,
    pub other: jvmtiHeapReferenceInfoReserved,
}

impl core::fmt::Debug for jvmtiHeapReferenceInfo {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(core::any::type_name::<Self>())
    }
}

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct jvmtiHeapCallbacks {
    pub heap_iteration_callback: jvmtiHeapIterationCallback,
    pub heap_reference_callback: jvmtiHeapReferenceCallback,
    pub primitive_field_callback: jvmtiPrimitiveFieldCallback,
    pub array_primitive_value_callback: jvmtiArrayPrimitiveValueCallback,
    pub string_primitive_value_callback: jvmtiStringPrimitiveValueCallback,
    pub reserved5: jvmtiReservedCallback,
    pub reserved6: jvmtiReservedCallback,
    pub reserved7: jvmtiReservedCallback,
    pub reserved8: jvmtiReservedCallback,
    pub reserved9: jvmtiReservedCallback,
    pub reserved10: jvmtiReservedCallback,
    pub reserved11: jvmtiReservedCallback,
    pub reserved12: jvmtiReservedCallback,
    pub reserved13: jvmtiReservedCallback,
    pub reserved14: jvmtiReservedCallback,
    pub reserved15: jvmtiReservedCallback,
}

impl jvmtiHeapCallbacks {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            heap_iteration_callback: None,
            heap_reference_callback: None,
            primitive_field_callback: None,
            array_primitive_value_callback: None,
            string_primitive_value_callback: None,
            reserved5: None,
            reserved6: None,
            reserved7: None,
            reserved8: None,
            reserved9: None,
            reserved10: None,
            reserved11: None,
            reserved12: None,
            reserved13: None,
            reserved14: None,
            reserved15: None,
        }
    }
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiClassDefinition {
    pub klass: jclass,
    pub class_byte_count: jint,
    pub class_bytes: *const c_uchar,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiMonitorUsage {
    pub owner: jthread,
    pub entry_count: jint,
    pub waiter_count: jint,
    pub waiters: *mut jthread,
    pub notify_waiter_count: jint,
    pub notify_waiters: *mut jthread,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiLineNumberEntry {
    pub start_location: jlocation,
    pub line_number: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiLocalVariableEntry {
    pub start_location: jlocation,
    pub length: jint,
    pub name: *mut c_char,
    pub signature: *mut c_char,
    pub generic_signature: *mut c_char,
    pub slot: jint,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiParamInfo {
    pub name: *mut c_char,
    pub kind: jvmtiParamKind,
    pub base_type: jvmtiParamTypes,
    pub null_ok: jboolean,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiExtensionFunctionInfo {
    pub func: jvmtiExtensionFunction,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
    pub error_count: jint,
    pub errors: *mut jvmtiError,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiExtensionEventInfo {
    pub extension_event_index: jint,
    pub id: *mut c_char,
    pub short_description: *mut c_char,
    pub param_count: jint,
    pub params: *mut jvmtiParamInfo,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiTimerInfo {
    pub max_value: jlong,
    pub may_skip_forward: jboolean,
    pub may_skip_backward: jboolean,
    pub kind: jvmtiTimerKind,
    pub reserved1: jlong,
    pub reserved2: jlong,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiAddrLocationMap {
    pub start_address: *const c_void,
    pub location: jlocation,
}

#[repr(C)]
#[repr(align(4))]
#[derive(Default, Copy, Clone)]
#[cfg_attr(systest, derive(Debug))]
pub struct jvmtiCapabilities {
    pub inner: [u8; 16],
}

// 'ctest2' choked on macro generator
#[cfg(not(systest))]
mod capabilities;

pub type jvmtiEventReserved = Option<unsafe extern "C" fn()>;

pub type jvmtiEventBreakpoint = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ),
>;

pub type jvmtiEventClassFileLoadHook = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        class_being_redefined: jclass,
        loader: jobject,
        name: *const c_char,
        protection_domain: jobject,
        class_data_len: jint,
        class_data: *const c_uchar,
        new_class_data_len: *mut jint,
        new_class_data: *mut *mut c_uchar,
    ),
>;

pub type jvmtiEventClassLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ),
>;

pub type jvmtiEventClassPrepare = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        klass: jclass,
    ),
>;

pub type jvmtiEventCompiledMethodLoad = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        method: jmethodID,
        code_size: jint,
        code_addr: *const c_void,
        map_length: jint,
        map: *const jvmtiAddrLocationMap,
        compile_info: *const c_void,
    ),
>;

pub type jvmtiEventCompiledMethodUnload = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, method: jmethodID, code_addr: *const c_void),
>;

pub type jvmtiEventDataDumpRequest = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventDynamicCodeGenerated = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        name: *const c_char,
        address: *const c_void,
        length: jint,
    ),
>;

pub type jvmtiEventException = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
        catch_method: jmethodID,
        catch_location: jlocation,
    ),
>;

pub type jvmtiEventExceptionCatch = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        exception: jobject,
    ),
>;

pub type jvmtiEventFieldAccess = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
    ),
>;

pub type jvmtiEventFieldModification = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
        field_klass: jclass,
        object: jobject,
        field: jfieldID,
        signature_type: c_char,
        new_value: jvalue,
    ),
>;

pub type jvmtiEventFramePop = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
    ),
>;

pub type jvmtiEventGarbageCollectionFinish = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventGarbageCollectionStart = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv)>;

pub type jvmtiEventMethodEntry = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
    ),
>;

pub type jvmtiEventMethodExit = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        was_popped_by_exception: jboolean,
        return_value: jvalue,
    ),
>;

pub type jvmtiEventMonitorContendedEnter = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ),
>;

pub type jvmtiEventMonitorContendedEntered = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
    ),
>;

pub type jvmtiEventMonitorWait = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timeout: jlong,
    ),
>;

pub type jvmtiEventMonitorWaited = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        timed_out: jboolean,
    ),
>;

pub type jvmtiEventNativeMethodBind = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        address: *mut c_void,
        new_address_ptr: *mut *mut c_void,
    ),
>;

pub type jvmtiEventObjectFree = Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, tag: jlong)>;

pub type jvmtiEventResourceExhausted = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        flags: jint,
        reserved: *const c_void,
        description: *const c_char,
    ),
>;

pub type jvmtiEventSampledObjectAlloc = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    ),
>;

pub type jvmtiEventSingleStep = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        method: jmethodID,
        location: jlocation,
    ),
>;

pub type jvmtiEventThreadEnd =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread)>;

pub type jvmtiEventThreadStart =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread)>;

pub type jvmtiEventVirtualThreadEnd = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, virtual_thread: jthread),
>;

pub type jvmtiEventVirtualThreadStart = Option<
    unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, virtual_thread: jthread),
>;

pub type jvmtiEventVMDeath =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;

pub type jvmtiEventVMInit =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv, thread: jthread)>;

pub type jvmtiEventVMObjectAlloc = Option<
    unsafe extern "C" fn(
        jvmti_env: *mut jvmtiEnv,
        jni_env: *mut JNIEnv,
        thread: jthread,
        object: jobject,
        object_klass: jclass,
        size: jlong,
    ),
>;

pub type jvmtiEventVMStart =
    Option<unsafe extern "C" fn(jvmti_env: *mut jvmtiEnv, jni_env: *mut JNIEnv)>;

#[repr(C)]
#[derive(Default, Debug, Copy, Clone)]
pub struct jvmtiEventCallbacks {
    pub VMInit: jvmtiEventVMInit,
    pub VMDeath: jvmtiEventVMDeath,
    pub ThreadStart: jvmtiEventThreadStart,
    pub ThreadEnd: jvmtiEventThreadEnd,
    pub ClassFileLoadHook: jvmtiEventClassFileLoadHook,
    pub ClassLoad: jvmtiEventClassLoad,
    pub ClassPrepare: jvmtiEventClassPrepare,
    pub VMStart: jvmtiEventVMStart,
    pub Exception: jvmtiEventException,
    pub ExceptionCatch: jvmtiEventExceptionCatch,
    pub SingleStep: jvmtiEventSingleStep,
    pub FramePop: jvmtiEventFramePop,
    pub Breakpoint: jvmtiEventBreakpoint,
    pub FieldAccess: jvmtiEventFieldAccess,
    pub FieldModification: jvmtiEventFieldModification,
    pub MethodEntry: jvmtiEventMethodEntry,
    pub MethodExit: jvmtiEventMethodExit,
    pub NativeMethodBind: jvmtiEventNativeMethodBind,
    pub CompiledMethodLoad: jvmtiEventCompiledMethodLoad,
    pub CompiledMethodUnload: jvmtiEventCompiledMethodUnload,
    pub DynamicCodeGenerated: jvmtiEventDynamicCodeGenerated,
    pub DataDumpRequest: jvmtiEventDataDumpRequest,
    pub reserved72: jvmtiEventReserved,
    pub MonitorWait: jvmtiEventMonitorWait,
    pub MonitorWaited: jvmtiEventMonitorWaited,
    pub MonitorContendedEnter: jvmtiEventMonitorContendedEnter,
    pub MonitorContendedEntered: jvmtiEventMonitorContendedEntered,
    pub reserved77: jvmtiEventReserved,
    pub reserved78: jvmtiEventReserved,
    pub reserved79: jvmtiEventReserved,
    pub ResourceExhausted: jvmtiEventResourceExhausted,
    pub GarbageCollectionStart: jvmtiEventGarbageCollectionStart,
    pub GarbageCollectionFinish: jvmtiEventGarbageCollectionFinish,
    pub ObjectFree: jvmtiEventObjectFree,
    pub VMObjectAlloc: jvmtiEventVMObjectAlloc,
    pub reserved85: jvmtiEventReserved,
    pub SampledObjectAlloc: jvmtiEventSampledObjectAlloc,
    pub VirtualThreadStart: jvmtiEventVirtualThreadStart,
    pub VirtualThreadEnd: jvmtiEventVirtualThreadEnd,
}

impl jvmtiEventCallbacks {
    #[inline(always)]
    pub const fn new() -> Self {
        Self {
            VMInit: None,
            VMDeath: None,
            ThreadStart: None,
            ThreadEnd: None,
            ClassFileLoadHook: None,
            ClassLoad: None,
            ClassPrepare: None,
            VMStart: None,
            Exception: None,
            ExceptionCatch: None,
            SingleStep: None,
            FramePop: None,
            Breakpoint: None,
            FieldAccess: None,
            FieldModification: None,
            MethodEntry: None,
            MethodExit: None,
            NativeMethodBind: None,
            CompiledMethodLoad: None,
            CompiledMethodUnload: None,
            DynamicCodeGenerated: None,
            DataDumpRequest: None,
            reserved72: None,
            MonitorWait: None,
            MonitorWaited: None,
            MonitorContendedEnter: None,
            MonitorContendedEntered: None,
            reserved77: None,
            reserved78: None,
            reserved79: None,
            ResourceExhausted: None,
            GarbageCollectionStart: None,
            GarbageCollectionFinish: None,
            ObjectFree: None,
            VMObjectAlloc: None,
            reserved85: None,
            SampledObjectAlloc: None,
            VirtualThreadStart: None,
            VirtualThreadEnd: None,
        }
    }
}

#[repr(C)]
#[jni_to_union("JVMTI", "1.0")]
#[non_exhaustive]
#[derive(Debug, Copy, Clone)]
pub struct jvmtiInterface_1_ {
    #[jni_added("reserved")]
    pub reserved1: *mut c_void,
    pub SetEventNotificationMode: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        mode: jvmtiEventMode,
        event_type: jvmtiEvent,
        event_thread: jthread,
        ...
    ) -> jvmtiError,
    #[jni_added("9")]
    pub GetAllModules: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        module_count_ptr: *mut jint,
        modules_ptr: *mut *mut jobject,
    ) -> jvmtiError,
    pub GetAllThreads: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        threads_count_ptr: *mut jint,
        threads_ptr: *mut *mut jthread,
    ) -> jvmtiError,
    pub SuspendThread: unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    pub ResumeThread: unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    pub StopThread:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, exception: jobject) -> jvmtiError,
    pub InterruptThread: unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    pub GetThreadInfo: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        info_ptr: *mut jvmtiThreadInfo,
    ) -> jvmtiError,
    pub GetOwnedMonitorInfo: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        owned_monitor_count_ptr: *mut jint,
        owned_monitors_ptr: *mut *mut jobject,
    ) -> jvmtiError,
    pub GetCurrentContendedMonitor: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        monitor_ptr: *mut jobject,
    ) -> jvmtiError,
    pub RunAgentThread: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        proc_: jvmtiStartFunction,
        arg: *const c_void,
        priority: jint,
    ) -> jvmtiError,
    pub GetTopThreadGroups: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        group_count_ptr: *mut jint,
        groups_ptr: *mut *mut jthreadGroup,
    ) -> jvmtiError,
    pub GetThreadGroupInfo: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        group: jthreadGroup,
        info_ptr: *mut jvmtiThreadGroupInfo,
    ) -> jvmtiError,
    pub GetThreadGroupChildren: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        group: jthreadGroup,
        thread_count_ptr: *mut jint,
        threads_ptr: *mut *mut jthread,
        group_count_ptr: *mut jint,
        groups_ptr: *mut *mut jthreadGroup,
    ) -> jvmtiError,
    pub GetFrameCount: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        count_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetThreadState: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        thread_state_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetCurrentThread:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread_ptr: *mut jthread) -> jvmtiError,
    pub GetFrameLocation: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        method_ptr: *mut jmethodID,
        location_ptr: *mut jlocation,
    ) -> jvmtiError,
    pub NotifyFramePop:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, depth: jint) -> jvmtiError,
    pub GetLocalObject: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jobject,
    ) -> jvmtiError,
    pub GetLocalInt: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetLocalLong: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jlong,
    ) -> jvmtiError,
    pub GetLocalFloat: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jfloat,
    ) -> jvmtiError,
    pub GetLocalDouble: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value_ptr: *mut jdouble,
    ) -> jvmtiError,
    pub SetLocalObject: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jobject,
    ) -> jvmtiError,
    pub SetLocalInt: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jint,
    ) -> jvmtiError,
    pub SetLocalLong: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jlong,
    ) -> jvmtiError,
    pub SetLocalFloat: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jfloat,
    ) -> jvmtiError,
    pub SetLocalDouble: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        slot: jint,
        value: jdouble,
    ) -> jvmtiError,
    pub CreateRawMonitor: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        name: *const c_char,
        monitor_ptr: *mut jrawMonitorID,
    ) -> jvmtiError,
    pub DestroyRawMonitor:
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    pub RawMonitorEnter:
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    pub RawMonitorExit:
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    pub RawMonitorWait: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        monitor: jrawMonitorID,
        millis: jlong,
    ) -> jvmtiError,
    pub RawMonitorNotify:
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    pub RawMonitorNotifyAll:
        unsafe extern "C" fn(env: *mut jvmtiEnv, monitor: jrawMonitorID) -> jvmtiError,
    pub SetBreakpoint: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        location: jlocation,
    ) -> jvmtiError,
    pub ClearBreakpoint: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        location: jlocation,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub GetNamedModule: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        class_loader: jobject,
        package_name: *const c_char,
        module_ptr: *mut jobject,
    ) -> jvmtiError,
    pub SetFieldAccessWatch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    pub ClearFieldAccessWatch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    pub SetFieldModificationWatch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    pub ClearFieldModificationWatch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, klass: jclass, field: jfieldID) -> jvmtiError,
    #[jni_added("1.1")]
    pub IsModifiableClass: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_modifiable_class_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub Allocate: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        size: jlong,
        mem_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    pub Deallocate: unsafe extern "C" fn(env: *mut jvmtiEnv, mem: *mut c_uchar) -> jvmtiError,
    pub GetClassSignature: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetClassStatus: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        status_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetSourceFileName: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        source_name_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetClassModifiers: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetClassMethods: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        method_count_ptr: *mut jint,
        methods_ptr: *mut *mut jmethodID,
    ) -> jvmtiError,
    pub GetClassFields: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field_count_ptr: *mut jint,
        fields_ptr: *mut *mut jfieldID,
    ) -> jvmtiError,
    pub GetImplementedInterfaces: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        interface_count_ptr: *mut jint,
        interfaces_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    pub IsInterface: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_interface_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub IsArrayClass: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        is_array_class_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub GetClassLoader: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        classloader_ptr: *mut jobject,
    ) -> jvmtiError,
    pub GetObjectHashCode: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        hash_code_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetObjectMonitorUsage: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        info_ptr: *mut jvmtiMonitorUsage,
    ) -> jvmtiError,
    pub GetFieldName: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        name_ptr: *mut *mut c_char,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetFieldDeclaringClass: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        declaring_class_ptr: *mut jclass,
    ) -> jvmtiError,
    pub GetFieldModifiers: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    pub IsFieldSynthetic: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        field: jfieldID,
        is_synthetic_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub GetMethodName: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        name_ptr: *mut *mut c_char,
        signature_ptr: *mut *mut c_char,
        generic_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetMethodDeclaringClass: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        declaring_class_ptr: *mut jclass,
    ) -> jvmtiError,
    pub GetMethodModifiers: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        modifiers_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("25")]
    pub ClearAllFramePops: unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    pub GetMaxLocals: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        max_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetArgumentsSize: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        size_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetLineNumberTable: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        entry_count_ptr: *mut jint,
        table_ptr: *mut *mut jvmtiLineNumberEntry,
    ) -> jvmtiError,
    pub GetMethodLocation: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        start_location_ptr: *mut jlocation,
        end_location_ptr: *mut jlocation,
    ) -> jvmtiError,
    pub GetLocalVariableTable: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        entry_count_ptr: *mut jint,
        table_ptr: *mut *mut jvmtiLocalVariableEntry,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub SetNativeMethodPrefix:
        unsafe extern "C" fn(env: *mut jvmtiEnv, prefix: *const c_char) -> jvmtiError,
    #[jni_added("1.1")]
    pub SetNativeMethodPrefixes: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        prefix_count: jint,
        prefixes: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetBytecodes: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        bytecode_count_ptr: *mut jint,
        bytecodes_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    pub IsMethodNative: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_native_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub IsMethodSynthetic: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_synthetic_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub GetLoadedClasses: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        class_count_ptr: *mut jint,
        classes_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    pub GetClassLoaderClasses: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        initiating_loader: jobject,
        class_count_ptr: *mut jint,
        classes_ptr: *mut *mut jclass,
    ) -> jvmtiError,
    pub PopFrame: unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnObject:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jobject) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnInt:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jint) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnLong:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jlong) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnFloat:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jfloat) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnDouble:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread, value: jdouble) -> jvmtiError,
    #[jni_added("1.1")]
    pub ForceEarlyReturnVoid:
        unsafe extern "C" fn(env: *mut jvmtiEnv, thread: jthread) -> jvmtiError,
    pub RedefineClasses: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        class_count: jint,
        class_definitions: *const jvmtiClassDefinition,
    ) -> jvmtiError,
    pub GetVersionNumber:
        unsafe extern "C" fn(env: *mut jvmtiEnv, version_ptr: *mut jint) -> jvmtiError,
    pub GetCapabilities: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *mut jvmtiCapabilities,
    ) -> jvmtiError,
    pub GetSourceDebugExtension: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        source_debug_extension_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub IsMethodObsolete: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        method: jmethodID,
        is_obsolete_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub SuspendThreadList: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        request_count: jint,
        request_list: *const jthread,
        results: *mut jvmtiError,
    ) -> jvmtiError,
    pub ResumeThreadList: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        request_count: jint,
        request_list: *const jthread,
        results: *mut jvmtiError,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleReads:
        unsafe extern "C" fn(env: *mut jvmtiEnv, module: jobject, to_module: jobject) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleExports: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        pkg_name: *const c_char,
        to_module: jobject,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleOpens: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        pkg_name: *const c_char,
        to_module: jobject,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleUses:
        unsafe extern "C" fn(env: *mut jvmtiEnv, module: jobject, service: jclass) -> jvmtiError,
    #[jni_added("9")]
    pub AddModuleProvides: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        service: jclass,
        impl_class: jclass,
    ) -> jvmtiError,
    #[jni_added("9")]
    pub IsModifiableModule: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        module: jobject,
        is_modifiable_module_ptr: *mut jboolean,
    ) -> jvmtiError,
    pub GetAllStackTraces: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        max_frame_count: jint,
        stack_info_ptr: *mut *mut jvmtiStackInfo,
        thread_count_ptr: *mut jint,
    ) -> jvmtiError,
    pub GetThreadListStackTraces: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread_count: jint,
        thread_list: *const jthread,
        max_frame_count: jint,
        stack_info_ptr: *mut *mut jvmtiStackInfo,
    ) -> jvmtiError,
    pub GetThreadLocalStorage: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        data_ptr: *mut *mut c_void,
    ) -> jvmtiError,
    pub SetThreadLocalStorage: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        data: *const c_void,
    ) -> jvmtiError,
    pub GetStackTrace: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        start_depth: jint,
        max_frame_count: jint,
        frame_buffer: *mut jvmtiFrameInfo,
        count_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved105: *mut c_void,
    pub GetTag: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        tag_ptr: *mut jlong,
    ) -> jvmtiError,
    pub SetTag: unsafe extern "C" fn(env: *mut jvmtiEnv, object: jobject, tag: jlong) -> jvmtiError,
    pub ForceGarbageCollection: unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError,
    pub IterateOverObjectsReachableFromObject: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        object_reference_callback: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    pub IterateOverReachableObjects: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        heap_root_callback: jvmtiHeapRootCallback,
        stack_ref_callback: jvmtiStackReferenceCallback,
        object_ref_callback: jvmtiObjectReferenceCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    pub IterateOverHeap: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    pub IterateOverInstancesOfClass: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        object_filter: jvmtiHeapObjectFilter,
        heap_object_callback: jvmtiHeapObjectCallback,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved113: *mut c_void,
    pub GetObjectsWithTags: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        tag_count: jint,
        tags: *const jlong,
        count_ptr: *mut jint,
        object_result_ptr: *mut *mut jobject,
        tag_result_ptr: *mut *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub FollowReferences: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        heap_filter: jint,
        klass: jclass,
        initial_object: jobject,
        callbacks: *const jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub IterateThroughHeap: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        heap_filter: jint,
        klass: jclass,
        callbacks: *const jvmtiHeapCallbacks,
        user_data: *const c_void,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved117: *mut c_void,
    #[jni_added("21")]
    pub SuspendAllVirtualThreads: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        except_count: jint,
        except_list: *const jthread,
    ) -> jvmtiError,
    #[jni_added("21")]
    pub ResumeAllVirtualThreads: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        except_count: jint,
        except_list: *const jthread,
    ) -> jvmtiError,
    pub SetJNIFunctionTable: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        function_table: *const jniNativeInterface,
    ) -> jvmtiError,
    pub GetJNIFunctionTable: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        function_table: *mut *mut jniNativeInterface,
    ) -> jvmtiError,
    pub SetEventCallbacks: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        callbacks: *const jvmtiEventCallbacks,
        size_of_callbacks: jint,
    ) -> jvmtiError,
    pub GenerateEvents:
        unsafe extern "C" fn(env: *mut jvmtiEnv, event_type: jvmtiEvent) -> jvmtiError,
    pub GetExtensionFunctions: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        extension_count_ptr: *mut jint,
        extensions: *mut *mut jvmtiExtensionFunctionInfo,
    ) -> jvmtiError,
    pub GetExtensionEvents: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        extension_count_ptr: *mut jint,
        extensions: *mut *mut jvmtiExtensionEventInfo,
    ) -> jvmtiError,
    pub SetExtensionEventCallback: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        extension_event_index: jint,
        callback: jvmtiExtensionEvent,
    ) -> jvmtiError,
    pub DisposeEnvironment: unsafe extern "C" fn(env: *mut jvmtiEnv) -> jvmtiError,
    pub GetErrorName: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        error: jvmtiError,
        name_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub GetJLocationFormat: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        format_ptr: *mut jvmtiJlocationFormat,
    ) -> jvmtiError,
    pub GetSystemProperties: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        count_ptr: *mut jint,
        property_ptr: *mut *mut *mut c_char,
    ) -> jvmtiError,
    pub GetSystemProperty: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        property: *const c_char,
        value_ptr: *mut *mut c_char,
    ) -> jvmtiError,
    pub SetSystemProperty: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        property: *const c_char,
        value_ptr: *const c_char,
    ) -> jvmtiError,
    pub GetPhase:
        unsafe extern "C" fn(env: *mut jvmtiEnv, phase_ptr: *mut jvmtiPhase) -> jvmtiError,
    pub GetCurrentThreadCpuTimerInfo:
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    pub GetCurrentThreadCpuTime:
        unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    pub GetThreadCpuTimerInfo:
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    pub GetThreadCpuTime: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        nanos_ptr: *mut jlong,
    ) -> jvmtiError,
    pub GetTimerInfo:
        unsafe extern "C" fn(env: *mut jvmtiEnv, info_ptr: *mut jvmtiTimerInfo) -> jvmtiError,
    pub GetTime: unsafe extern "C" fn(env: *mut jvmtiEnv, nanos_ptr: *mut jlong) -> jvmtiError,
    pub GetPotentialCapabilities: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *mut jvmtiCapabilities,
    ) -> jvmtiError,
    #[jni_added("reserved")]
    pub reserved141: *mut c_void,
    pub AddCapabilities: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *const jvmtiCapabilities,
    ) -> jvmtiError,
    pub RelinquishCapabilities: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        capabilities_ptr: *const jvmtiCapabilities,
    ) -> jvmtiError,
    pub GetAvailableProcessors:
        unsafe extern "C" fn(env: *mut jvmtiEnv, processor_count_ptr: *mut jint) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetClassVersionNumbers: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        minor_version_ptr: *mut jint,
        major_version_ptr: *mut jint,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetConstantPool: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        klass: jclass,
        constant_pool_count_ptr: *mut jint,
        constant_pool_byte_count_ptr: *mut jint,
        constant_pool_bytes_ptr: *mut *mut c_uchar,
    ) -> jvmtiError,
    pub GetEnvironmentLocalStorage:
        unsafe extern "C" fn(env: *mut jvmtiEnv, data_ptr: *mut *mut c_void) -> jvmtiError,
    pub SetEnvironmentLocalStorage:
        unsafe extern "C" fn(env: *mut jvmtiEnv, data: *const c_void) -> jvmtiError,
    pub AddToBootstrapClassLoaderSearch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
    pub SetVerboseFlag: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        flag: jvmtiVerboseFlag,
        value: jboolean,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub AddToSystemClassLoaderSearch:
        unsafe extern "C" fn(env: *mut jvmtiEnv, segment: *const c_char) -> jvmtiError,
    #[jni_added("1.1")]
    pub RetransformClasses: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        class_count: jint,
        classes: *const jclass,
    ) -> jvmtiError,
    #[jni_added("1.1")]
    pub GetOwnedMonitorStackDepthInfo: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        monitor_info_count_ptr: *mut jint,
        monitor_info_ptr: *mut *mut jvmtiMonitorStackDepthInfo,
    ) -> jvmtiError,
    pub GetObjectSize: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        object: jobject,
        size_ptr: *mut jlong,
    ) -> jvmtiError,
    #[jni_added("1.2")]
    pub GetLocalInstance: unsafe extern "C" fn(
        env: *mut jvmtiEnv,
        thread: jthread,
        depth: jint,
        value_ptr: *mut jobject,
    ) -> jvmtiError,
    #[jni_added("11")]
    pub SetHeapSamplingInterval:
        unsafe extern "C" fn(env: *mut jvmtiEnv, sampling_interval: jint) -> jvmtiError,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct _jvmtiEnv {
    pub functions: *const jvmtiInterface_1_,
}
