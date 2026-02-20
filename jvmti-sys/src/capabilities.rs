struct Index {
    byte: usize,
    mask: u8,
}

impl Index {
    #[inline]
    const fn position(index: usize) -> Self {
        #[cfg(target_endian = "big")]
        let bit = 7 - (index % 8);
        #[cfg(not(target_endian = "big"))]
        let bit = index % 8;

        Self {
            byte: index / 8,
            mask: 1 << bit,
        }
    }
}

impl crate::jvmtiCapabilities {
    pub const EMPTY: Self = Self { inner: [0; 16] };

    #[inline(always)]
    pub const fn get(&self, bit: usize) -> bool {
        let Index { byte, mask } = Index::position(bit);
        self.inner[byte] & mask == mask
    }

    #[inline(always)]
    pub const fn set(&mut self, bit: usize, value: bool) {
        let Index { byte, mask } = Index::position(bit);
        if value {
            self.inner[byte] |= mask;
        } else {
            self.inner[byte] &= !mask;
        }
    }
}

macro_rules! capabilities {
    ($($field:ident),+ $(,)?) => {
        paste::paste! {
            #[repr(usize)]
            enum Position {
                $($field,)+
            }

            impl crate::jvmtiCapabilities {
                $(
                pub const [<$field:upper _BIT>]: usize = Position::$field as usize;

                #[inline(always)]
                pub const fn $field(&self) -> bool {
                    self.get(Self::[<$field:upper _BIT>])
                }

                #[inline(always)]
                pub const fn [<set_ $field>](&mut self, value: bool) {
                    self.set(Self::[<$field:upper _BIT>], value);
                }
                )+
            }

            impl core::fmt::Debug for crate::jvmtiCapabilities {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.debug_struct("jvmtiCapabilities")
                        $(
                        .field(stringify!($field), &self.$field())
                        )+
                        .finish()
                }
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
