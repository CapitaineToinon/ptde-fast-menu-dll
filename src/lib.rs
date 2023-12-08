use memory_rs::internal::injections::*;
use memory_rs::internal::process_info::ProcessInfo;

// function wrapper to be called by DllMain
pub unsafe extern "system" fn wrapper(_lib: *mut std::ffi::c_void) -> u32 {
    match patch() {
        Ok(_) => println!("Everything is OK"),
        Err(e) => println!("Error: {}", e),
    };

    0
}

fn patch() -> Result<(), Box<dyn std::error::Error>> {
    let proc_inf = ProcessInfo::new(Some("DARKSOULS.exe"))?;

    let mut force_slot_0 = Injection::new(
        proc_inf.region.start_address + 0xAEDF0E,
        vec![
            0xB8, 0x0, 0x0, 0x0, 0x0, // mov EAX,0
        ],
    );

    let mut remove_slot_count_call_1 = Injection::new(
        proc_inf.region.start_address + 0x877E9F,
        vec![
            0xB8, 0x5, 0x0, 0x0, 0x0, // mov EAX,1
        ],
    );

    let mut remove_slot_count_call_2 = Injection::new(
        proc_inf.region.start_address + 0xAEE55F,
        vec![
            0xB8, 0x5, 0x0, 0x0, 0x0, // mov EAX,1
        ],
    );

    let mut remove_slot_count_call_3 = Injection::new(
        proc_inf.region.start_address + 0x878450,
        vec![
            0xB8, 0x5, 0x0, 0x0, 0x0, // mov EAX,1
        ],
    );

    let mut remove_slot_count_call_4 = Injection::new(
        proc_inf.region.start_address + 0xAECA3E,
        vec![
            0xB8, 0x5, 0x0, 0x0, 0x0, // mov EAX,1
        ],
    );

    let mut remove_slot_count_call_5 = Injection::new(
        proc_inf.region.start_address + 0xAED466,
        vec![
            0xB8, 0x5, 0x0, 0x0, 0x0, // mov EAX,1
        ],
    );

    force_slot_0.inject();
    remove_slot_count_call_1.inject();
    remove_slot_count_call_2.inject();
    remove_slot_count_call_3.inject();
    remove_slot_count_call_4.inject();
    remove_slot_count_call_5.inject();

    loop {}
}

// This will generate the DllMain required to create a DLL in Windows.
memory_rs::main_dll!(wrapper);
