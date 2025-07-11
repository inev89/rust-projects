use windows::Win32::System::Memory::{
                                     VirtualAlloc,
                                     VirtualFree,
                                     PAGE_EXECUTE_READWRITE,
                                     MEM_COMMIT,
                                     MEM_RELEASE,
                                    // MEM_RESERVE,
                                    };
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxA, MB_OK};
use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError};

fn main() {

    unsafe{
        let allocated_memory = VirtualAlloc(
                                           None,
                                           1024,
                                           MEM_COMMIT,
                                           PAGE_EXECUTE_READWRITE
                                        );

        if allocated_memory.is_null(){
            println!("Memory allocation failed.");
                                    }
        else{
            println!("Memory allocated at: {:?}", allocated_memory);

            MessageBoxA(None,
                    PCSTR(b"Hack the Planet!!\0".as_ptr()),
                    PCSTR(b"Hackers\0".as_ptr()),
                    MB_OK);

            let last_error = GetLastError();
            
            println!("MessageBoxA Error: {:?}", last_error);

            let _result = match VirtualFree(allocated_memory, 0, MEM_RELEASE){

                Ok(_result) => {
                    println!("Memory successfully freed.");
                },
                Err(_e) => {
                    println!("Failed to free memory");
                }
                };
            }
        };
}
