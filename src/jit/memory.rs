use anyhow::{Context, Result};
use std::ptr;

#[cfg(unix)]
use libc::{mmap, munmap, MAP_ANONYMOUS, MAP_PRIVATE, PROT_EXEC, PROT_READ, PROT_WRITE};

#[cfg(windows)]
use winapi::um::memoryapi::{VirtualAlloc, VirtualFree};
#[cfg(windows)]
use winapi::um::winnt::{MEM_COMMIT, MEM_RELEASE, PAGE_EXECUTE_READWRITE};

#[derive(Debug)]
pub enum MemoryError {
    AllocationFailed,
    InvalidSize,
}

/// Dynamically allocated executable memory
pub struct ExecutableMemory {
    ptr: *mut u8,
    size: usize,
}

unsafe impl Send for ExecutableMemory {}
unsafe impl Sync for ExecutableMemory {}

impl ExecutableMemory {
    /// Allocate executable memory of specified size
    pub fn allocate(size: usize) -> Result<Self> {
        if size == 0 {
            return Err(anyhow::anyhow!(MemoryError::InvalidSize));
        }

        #[cfg(unix)]
        {
            let ptr = unsafe {
                mmap(
                    ptr::null_mut(),
                    size,
                    PROT_READ | PROT_WRITE | PROT_EXEC,
                    MAP_PRIVATE | MAP_ANONYMOUS,
                    -1,
                    0,
                )
            };

            if ptr == libc::MAP_FAILED {
                return Err(anyhow::anyhow!(MemoryError::AllocationFailed));
            }

            Ok(Self {
                ptr: ptr as *mut u8,
                size,
            })
        }

        #[cfg(windows)]
        {
            let ptr = unsafe {
                VirtualAlloc(
                    ptr::null_mut(),
                    size,
                    MEM_COMMIT,
                    PAGE_EXECUTE_READWRITE,
                )
            };

            if ptr.is_null() {
                return Err(anyhow::anyhow!(MemoryError::AllocationFailed));
            }

            Ok(Self {
                ptr: ptr as *mut u8,
                size,
            })
        }
    }

    /// Return pointer to memory
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr
    }

    /// Return memory size
    pub fn size(&self) -> usize {
        self.size
    }

    /// Write data to memory
    pub unsafe fn write(&mut self, offset: usize, data: &[u8]) -> Result<()> {
        if offset + data.len() > self.size {
            return Err(anyhow::anyhow!("Write outside memory bounds"));
        }

        ptr::copy_nonoverlapping(
            data.as_ptr(),
            self.ptr.add(offset),
            data.len(),
        );

        Ok(())
    }

    /// Get a function pointer
    pub fn as_function<T>(&self) -> T {
        unsafe { std::mem::transmute(self.ptr) }
    }
}

impl Drop for ExecutableMemory {
    fn drop(&mut self) {
        #[cfg(unix)]
        {
            unsafe {
                munmap(self.ptr as *mut libc::c_void, self.size);
            }
        }

        #[cfg(windows)]
        {
            unsafe {
                VirtualFree(self.ptr as *mut winapi::ctypes::c_void, 0, MEM_RELEASE);
            }
        }
    }
}

