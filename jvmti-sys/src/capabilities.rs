macro_rules! capabilities {
    ($($field:ident),* $(,)?) => {
        paste::paste! {
            #[repr(usize)]
            enum Idx {
                $($field,)+
            }

            impl crate::jvmtiCapabilities {
                #[inline(always)]
                pub const fn new() -> Self {
                    Self { inner: [0; 16] }
                }

                $(
                pub const [<$field:upper _BIT>]: usize = Idx::$field as usize;
                const [<$field:upper _INDEX>]: usize = Self::[<$field:upper _BIT>] / 8;
                const [<$field:upper _MASK>]: u8 = {
                    let field = Self::[<$field:upper _BIT>];
                    let bit = if cfg!(target_endian = "big") {
                        7 - (field % 8)
                    } else {
                        field % 8
                    };
                    1 << bit
                };

                #[inline(always)]
                pub const fn $field(&self) -> bool {
                    let byte = Self::[<$field:upper _INDEX>];
                    let mask = Self::[<$field:upper _MASK>];
                    self.inner[byte] & mask == mask
                }

                #[inline(always)]
                pub const fn [<set_ $field>](&mut self, value: bool) {
                    let byte = Self::[<$field:upper _INDEX>];
                    let mask = Self::[<$field:upper _MASK>];
                    if value {
                        self.inner[byte] |= mask;
                    } else {
                        self.inner[byte] &= !mask;
                    }
                }
                )+
            }
        }
    }
}

capabilities! {
    can_tag_objects,
    can_generate_field_modification_events,
    can_generate_field_access_events,
    can_get_bytecodes,
    can_get_synthetic_attribute,
    can_get_owned_monitor_info,
    can_get_current_contended_monitor,
    can_get_monitor_info,
    can_pop_frame,
    can_redefine_classes,
    can_signal_thread,
    can_get_source_file_name,
    can_get_line_numbers,
    can_get_source_debug_extension,
    can_access_local_variables,
    can_maintain_original_method_order,
    can_generate_single_step_events,
    can_generate_exception_events,
    can_generate_frame_pop_events,
    can_generate_breakpoint_events,
    can_suspend,
    can_redefine_any_class,
    can_get_current_thread_cpu_time,
    can_get_thread_cpu_time,
    can_generate_method_entry_events,
    can_generate_method_exit_events,
    can_generate_all_class_hook_events,
    can_generate_compiled_method_load_events,
    can_generate_monitor_events,
    can_generate_vm_object_alloc_events,
    can_generate_native_method_bind_events,
    can_generate_garbage_collection_events,
    can_generate_object_free_events,
    can_force_early_return,
    can_get_owned_monitor_stack_depth_info,
    can_get_constant_pool,
    can_set_native_method_prefix,
    can_retransform_classes,
    can_retransform_any_class,
    can_generate_resource_exhaustion_heap_events,
    can_generate_resource_exhaustion_threads_events,
    can_generate_early_vmstart,
    can_generate_early_class_hook_events,
    can_generate_sampled_object_alloc_events,
    can_support_virtual_threads,
}
