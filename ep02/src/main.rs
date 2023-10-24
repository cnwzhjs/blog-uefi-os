#![no_main]
#![no_std]

use raw_cpuid::CpuId;
use uefi::{prelude::*, table::boot::{MemoryMap, MemoryType, OpenProtocolParams, OpenProtocolAttributes}, proto::console::gop::GraphicsOutput};
use uefi_services::*;

fn wait_for_any_key(system_table: &mut SystemTable<Boot>) {
    let mut events = [ system_table.stdin().wait_for_key_event().unwrap() ];
    system_table.boot_services().wait_for_event(&mut events).discard_errdata().unwrap();
    system_table.stdin().read_key().unwrap();
}

fn print_size_of_pages(page_count: usize) {
    let size = (page_count * 4096) as f64;
    if size < 1024f64 {
        print!("{:6.1} B", size);
    } else if size < 1024f64 * 1024f64 {
        print!("{:5.1} KB", size / 1024f64);
    } else if size < 1024f64 * 1024f64 * 1024f64 {
        print!("{:5.1} MB", size / (1024f64 * 1024f64));
    } else {
        print!("{:5.1} GB", size / (1024f64 * 1024f64 * 1024f64));
    }
}

fn print_pointer_section(ptr: usize, mem_map: &MemoryMap) {
    let ptr = ptr as u64;

    for desc in mem_map.entries() {
        if desc.phys_start <= ptr && ptr < desc.phys_start + desc.page_count * 4096 {
            print!(
                "{:016X} {:016X} {:12} ",
                desc.phys_start,
                desc.virt_start,
                desc.page_count,
            );
    
            print_size_of_pages(desc.page_count as usize);
    
            println!(" {:?} ({:?})", desc.ty, desc.att);
            return;
        }
    }

    println!("-- not found --");
}

#[entry]
fn main(handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();

    print_cpu_info();

    println!("Press any key to see memory map...");
    wait_for_any_key(&mut system_table);
    print_memory_info(&mut system_table);

    println!("Press any key to see display info...");
    wait_for_any_key(&mut system_table);
    print_display_info(handle, &mut system_table);

    println!("Press any key to return to UEFI...");
    wait_for_any_key(&mut system_table);

    Status::SUCCESS
}

fn print_cpu_info() {
    let cpuid = CpuId::new();
    let vendor_info = cpuid.get_vendor_info().unwrap();
    println!("Vendor: {}", vendor_info.as_str());
    let brand_info = cpuid.get_processor_brand_string().unwrap();
    println!("Processor: {}", brand_info.as_str());
    let feature_info = cpuid.get_feature_info().unwrap();
    let extended_processor_feature_info = cpuid.get_extended_processor_and_feature_identifiers().unwrap();
    let advanced_pm_info = cpuid.get_advanced_power_mgmt_info().unwrap();
    println!("Family: {:02X}h, Model: {:02X}h, Step: {:02X}h", feature_info.family_id(), feature_info.model_id(), feature_info.stepping_id());
    println!("Max logical processor ids: {}", feature_info.max_logical_processor_ids());
    println!("Features:");
    println!("    vmx: {}", feature_info.has_vmx());
    println!("    hypervisor: {}", feature_info.has_hypervisor());
    println!("    tsc: {}", feature_info.has_tsc());
    println!("    psn: {}", feature_info.has_psn());
    println!("    sysenter & sysexit: {}", feature_info.has_sysenter_sysexit());
    println!("    syscall & sysret: {}", extended_processor_feature_info.has_syscall_sysret());
    println!("    svm: {}", extended_processor_feature_info.has_svm());
    println!("    de: {}", extended_processor_feature_info.has_execute_disable());
    println!("    1g pages: {}", extended_processor_feature_info.has_1gib_pages());
    println!("    rdtscp: {}", extended_processor_feature_info.has_rdtscp());
    println!("    invariant tsc: {}", advanced_pm_info.has_invariant_tsc());
}

fn print_memory_info(system_table: &mut SystemTable<Boot>) {
    // fetch the memory layout
    let mut buf = [0u8; 16_384];
    let buf_ptr = buf.as_ptr() as usize;

    let memory_map = system_table.boot_services().memory_map(&mut buf).unwrap();

    // print the memory layout
    println!("Memory map:");
    println!("{:16} {:16} {:12} {:8} {}", "Physical Addr", "Virtual Addr", "Num Pages", "Size", "Type");

    let mut i = 0;
    let mut total_pages = 0;
    let mut usable_pages = 0;

    for descriptor in memory_map.entries() {
        total_pages += descriptor.page_count;
        if descriptor.ty == MemoryType::CONVENTIONAL {
            usable_pages += descriptor.page_count;
        }

        if i != 0 && (i % 39) == 0 {
            println!("--- MORE ---");
            wait_for_any_key(system_table);
        }

        print!(
            "{:016X} {:016X} {:12} ",
            descriptor.phys_start,
            descriptor.virt_start,
            descriptor.page_count,
        );

        print_size_of_pages(descriptor.page_count as usize);

        println!(" {:?} ({:?})", descriptor.ty, descriptor.att);

        i += 1;
    }

    println!("--- END ---");
    print!("Total: ");
    print_size_of_pages(total_pages as usize);
    print!(", Usable: ");
    print_size_of_pages(usable_pages as usize);
    println!();
    println!();

    println!("buf (stack) is located at {:016X}, section:", buf_ptr);
    print_pointer_section(buf_ptr, &memory_map);

    let heap_buf = system_table.boot_services().allocate_pool(MemoryType::LOADER_DATA, 1024).unwrap();
    let heap_buf_ptr = heap_buf as usize;
    println!("heap_buf is located at {:016X}, section:", heap_buf_ptr);
    print_pointer_section(heap_buf_ptr, &memory_map);
    system_table.boot_services().free_pool(heap_buf).unwrap();    
}

fn print_display_info(image_handle: Handle, system_table: &mut SystemTable<Boot>) {
    let boot_services = system_table.boot_services();
    let gop_handle = boot_services.get_handle_for_protocol::<GraphicsOutput>().unwrap();

    let gop = unsafe { system_table
        .boot_services()
        .open_protocol::<GraphicsOutput>(OpenProtocolParams {
            handle: gop_handle,
            agent: image_handle,
            controller: None
        }, OpenProtocolAttributes::GetProtocol
    ) }.unwrap();

    println!("Supported Modes:");
    for mode in gop.modes() {
        println!(
            "    {:4} x {:4} @ {:?}",
            mode.info().resolution().0,
            mode.info().resolution().1,
            mode.info().pixel_format()
        );
    };

    let current_mode = gop.current_mode_info();
    println!("Current Mode:");
    println!(
        "    {:4} x {:4} @ {:?}",
        current_mode.resolution().0,
        current_mode.resolution().1,
        current_mode.pixel_format()
    );
}
