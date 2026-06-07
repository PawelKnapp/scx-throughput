#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

char LICENSE[] SEC("license") = "GPL";

SEC("struct_ops/select_cpu")
s32 BPF_PROG(select_cpu, struct task_struct *p, s32 prev_cpu, u64 wake_flags) {
    // To jest miejsce, gdzie planista decyduje, na którym rdzeniu proces ma pracować.
    // Na razie zwracamy 0, czyli domyślny wybór jądra.
    return 0;
}