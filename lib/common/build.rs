use raw_cpuid::CpuId;

fn main() {
    let cpuid = CpuId::new();

    if let Some(cpu) = cpuid.get_vendor_info() {
        eprintln!("CPU vendor: {}", cpu.as_str());
        println!("cargo:rustc-cfg=feature=\"{cpuv}\"", cpuv = if cpu.as_str() == "GenuineIntel" {
            "intel_cpu"
        } else if cpu.as_str() == "AuthenticAMD" {
            "amd_cpu"
        } else {
            "unknown_cpu"
        });
    } else {
        eprintln!("Failed to get CPU vendor info");
    }
}
