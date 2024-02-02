use rand::Rng;
use std::ffi::CStr;
use std::{env, fs};

fn main() {
    let mut args = env::args();
    args.next();

    let file = args.next().unwrap();
    let mut data = fs::read(&file).unwrap();

    let key: [u8; 16] = rand::thread_rng().gen();

    let infos;

    unsafe {
        infos = get_segment_info(data.as_mut_ptr());
    }

    println!("infos: {:?}", infos);

    encrypt(&mut data, &infos[0], &key);

    patch_stub(&mut data, &infos, key);

    fs::write("target\\release\\encrypted_clicksbotgui.exe", data).unwrap();
}

unsafe fn get_segment_info(base_address: *mut u8) -> [SegmentInfo; 2] {
    let dos_header = base_address as *mut DOSHeader;
    if (*dos_header).e_magic != 0x5A4D {
        panic!("DOS_MAGIC is invalid.");
    }

    let pe_header = base_address.add((*dos_header).e_lfanew as usize) as *mut PEHeader;

    if (*pe_header).signature != 0x4550 {
        panic!("PE_MAGIC is invalid.");
    }

    let optional_header = &(*pe_header).optional_header;

    if optional_header.magic != 0x20B {
        panic!("OPTIONAL_HEADER_MAGIC is invalid.");
    }

    let mut code_info = SegmentInfo {
        module_base: optional_header.image_base,
        file_segment_offset: 0,
        file_segment_size: 0,
        memory_segment_offset: 0,
    };

    let mut stub_info = SegmentInfo {
        module_base: optional_header.image_base,
        file_segment_offset: 0,
        file_segment_size: 0,
        memory_segment_offset: 0,
    };

    let number_of_sections = (*pe_header).file_header.number_of_sections;

    let mut section = (&(*pe_header).optional_header as *const _ as usize
        + (*pe_header).file_header.size_of_optional_header as usize)
        as *mut ImageSectionHeader;

    for _ in 0..number_of_sections {
        println!(
            "Section Name: {:?}",
            CStr::from_ptr((*section).name.as_ptr() as _)
        );

        let info;
        if libc::strcmp((*section).name.as_ptr() as _, b".code\0".as_ptr() as _) == 0 {
            info = &mut code_info;
        } else if libc::strcmp((*section).name.as_ptr() as _, b".stub\0".as_ptr() as _) == 0 {
            info = &mut stub_info;
        } else {
            section = section.add(1);
            continue;
        }

        info.file_segment_offset = (*section).pointer_to_raw_data;
        info.file_segment_size = (*section).size_of_raw_data;
        info.memory_segment_offset = (*section).virtual_address;

        section = section.add(1);
    }

    [code_info, stub_info]
}

fn encrypt(data: &mut [u8], code_info: &SegmentInfo, key: &[u8]) {
    let d =
        &mut data[code_info.file_segment_offset as usize..][..code_info.file_segment_size as usize];

    for (original, key) in d.iter_mut().zip(key.iter().cycle()) {
        *original ^= key;
    }
}

fn patch_stub(data: &mut [u8], infos: &[SegmentInfo; 2], key: [u8; 16]) {
    let d =
        &mut data[infos[1].file_segment_offset as usize..][..infos[1].file_segment_size as usize];

    let code_start = d.windows(8).position(|x| x == [0x18; 8]).unwrap();
    println!("code_start: {}", code_start);
    let code_length = d.windows(8).position(|x| x == [0x19; 8]).unwrap();
    println!("code_length: {}", code_length);
    let key_pos = d.windows(16).position(|x| x == [0x20; 16]).unwrap();
    println!("key_pos: {}", key_pos);

    d[code_start..][..8].copy_from_slice(&u64::to_le_bytes(
        infos[0].module_base + infos[0].memory_segment_offset as u64,
    ));
    d[code_length..][..8].copy_from_slice(&u64::to_le_bytes(infos[0].file_segment_size as u64));
    d[key_pos..][..16].copy_from_slice(&key);
}

#[repr(C)]
struct PEHeader {
    //change to 32 bit
    signature: u32,
    file_header: FileHeader,
    optional_header: OptionalHeader,
}

#[repr(C)]
struct FileHeader {
    machine: u16,
    number_of_sections: u16,
    time_date_stamp: u32,
    pointer_to_symbol_table: u32,
    number_of_symbols: u32,
    size_of_optional_header: u16,
    characteristics: u16,
}

#[repr(C)]
struct OptionalHeader {
    magic: u16,
    major_linker_version: u8,
    minor_linker_version: u8,
    size_of_code: u32,
    size_of_initialized_data: u32,
    size_of_uninitialized_data: u32,
    address_of_entry_point: u32,
    base_of_code: u32,
    image_base: u64,
    section_alignment: u32,
    file_alignment: u32,
    major_operating_system_version: u16,
    minor_operating_system_version: u16,
    major_image_version: u16,
    minor_image_version: u16,
    major_subsystem_version: u16,
    minor_subsystem_version: u16,
    win32version_value: u32,
    size_of_image: u32,
    size_of_headers: u32,
    check_sum: u32,
    subsystem: u16,
    dll_characteristics: u16,
    size_of_stack_reserve: u64,
    size_of_stack_commit: u64,
    size_of_heap_reserve: u64,
    size_of_heap_commit: u64,
    loader_flags: u32,
    number_of_rva_and_sizes: u32,
    data_directory: [ImageDataDirectory; 16],
}

#[repr(C)]
struct ImageDataDirectory {
    virtual_address: u32,
    size: u32,
}

#[derive(Debug)]
struct SegmentInfo {
    module_base: u64,
    file_segment_offset: u32,
    file_segment_size: u32,
    memory_segment_offset: u32,
}

#[repr(C)]
struct DOSHeader {
    e_magic: u16,
    e_cblp: u16,
    e_cp: u16,
    e_crlc: u16,
    e_cparhdr: u16,
    e_minalloc: u16,
    e_maxalloc: u16,
    e_ss: u16,
    e_sp: u16,
    e_csum: u16,
    e_ip: u16,
    e_cs: u16,
    e_lfarlc: u16,
    e_ovno: u16,
    e_res: [u16; 4],
    e_oemid: u16,
    e_oeminfo: u16,
    e_res2: [u16; 10],
    e_lfanew: u32,
}

#[repr(C)]
struct ImageSectionHeader {
    name: [u8; 8],
    misc: ImageSectionHeaderMisc,
    virtual_address: u32,
    size_of_raw_data: u32,
    pointer_to_raw_data: u32,
    pointer_to_relocations: u32,
    pointer_to_linenumbers: u32,
    number_of_relocations: u16,
    number_of_linenumbers: u16,
    characteristics: u32,
}

#[repr(C)]
union ImageSectionHeaderMisc {
    physical_address: u32,
    virtual_size: u32,
}
