use crate::spec::base::apple::{opts, Arch, TargetAbi};
use crate::spec::{Target, TargetOptions};

pub(crate) fn target() -> Target {
    let arch = Arch::Armv7k;
    Target {
        llvm_target: "armv7k-apple-watchos".into(),
        metadata: crate::spec::TargetMetadata {
            description: Some("Armv7-A Apple WatchOS".into()),
            tier: Some(3),
            host_tools: Some(false),
            std: Some(true),
        },
        pointer_width: 32,
        data_layout: "e-m:o-p:32:32-Fi8-i64:64-a:0:32-n32-S128".into(),
        arch: arch.target_arch(),
        options: TargetOptions {
            features: "+v7,+vfp4,+neon".into(),
            max_atomic_width: Some(64),
            dynamic_linking: false,
            position_independent_executables: true,
            ..opts("watchos", arch, TargetAbi::Normal)
        },
    }
}
