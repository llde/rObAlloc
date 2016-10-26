extern crate libc;
extern crate kernel32;
extern crate winapi;

use std::sync::RwLock;
#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}


static doVerify : bool = true;
static ZeroInitialized : u32 = 0x00000008;
static NoSerialization : u32 = 0x00000001;
static NoFlag : u32  = 0x00000000;

fn GetHeapProcess() -> winapi::HANDLE{
    let mut handle;
    unsafe{handle = kernel32::GetProcessHeap();}
    return handle;
}

fn IsHeapValid(heap : winapi::HANDLE, pointer  : *const std::os::raw::c_void) -> i32{
    let mut valid;
    unsafe{valid = kernel32::HeapValidate(heap,NoFlag,pointer);}
    return valid;
}

fn Allocate(heap : winapi::HANDLE, size : usize) -> *const std::os::raw::c_void{
    let pointer;
    unsafe{ pointer = kernel32::HeapAlloc(heap, ZeroInitialized, size as u32);}
    return pointer;
}

fn Reallocate(heap : winapi::HANDLE, size : usize , pointe : *mut std::os::raw::c_void) -> *const std::os::raw::c_void{
    let pointer;
    unsafe{ pointer = kernel32::HeapReAlloc(heap, ZeroInitialized, pointe ,size as u32);}
    return pointer;
}

fn GetSizeHeap(heap : winapi::HANDLE) -> u32{
    let mut valid;
    unsafe{valid = kernel32::HeapSize(heap,NoFlag, 0 as *const std::os::raw::c_void);}
    return valid;
}

fn FreeHeap(heap : winapi::HANDLE, obj : *mut std::os::raw::c_void) -> (){
    unsafe{kernel32::HeapFree(heap, NoFlag ,obj);}
}


#[no_mangle]
pub extern "C"  fn obAlloc(sizeT : usize) -> *const std::os::raw::c_void{
    let size;
    if sizeT < 1 {
        size = 1;
    }
    else {size = sizeT;}
    let heap = GetHeapProcess();
    Allocate(heap,size)
}


#[no_mangle]
pub extern "C" fn obFree(allocObj : *mut std::os::raw::c_void) -> (){
    if(allocObj as usize == 0){return;}
    let  heap = GetHeapProcess();
    let  valid = IsHeapValid(heap,allocObj);
    if(valid != 0) {FreeHeap(heap, allocObj);}
}



#[no_mangle]
pub extern "C" fn obSize() -> usize {
    let mut heap = GetHeapProcess();
    GetSizeHeap(heap) as usize
}




#[no_mangle]
pub extern "C"  fn obRealloc(point : *mut std::os::raw::c_void, sizeT : usize) -> *const std::os::raw::c_void{
    let size;
    if sizeT < 1 {
        size = 1;
    }
    else {size = sizeT;}
    let mut heap = GetHeapProcess();
    let realloc = IsHeapValid(heap, point);
    if(realloc != 0){
        Reallocate(heap,size, point)
    }
    else{
        Allocate(heap,size)
    }
}



#[no_mangle]
pub extern "C" fn obIsInHeap(validate : *const std::os::raw::c_void) -> bool{
    let mut heap = GetHeapProcess();
    let valid = IsHeapValid(heap,validate); 
    if(valid == 0) {return false;}
    else  {return true;}
}

    