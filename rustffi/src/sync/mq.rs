use crate::ffi;
// use core::ffi::CStr;

pub struct MessageQueue<T> {
    raw: ffi::rt_mq_t,
    _phantom: core::marker::PhantomData<T>,
}

unsafe impl<T: Copy + Send> Send for MessageQueue<T> {}
unsafe impl<T: Copy + Send> Sync for MessageQueue<T> {}

impl<T: Copy + Send> MessageQueue<T> {
    pub fn new(name: &str, cap: usize) -> Option<Self> {
        let mq = unsafe {
            ffi::rt_mq_create(
                name.as_ptr() as _,
                core::mem::size_of::<T>() as _,
                cap as _,
                ffi::RT_IPC_FLAG_FIFO as _,
            )
        };

        if mq.is_null() {
            None
        } else {
            Some(Self {
                raw: mq,
                _phantom: core::marker::PhantomData,
            })
        }
    }

    pub fn send(&self, msg: T) -> bool {
        let ret = unsafe {
            ffi::rt_mq_send(
                self.raw,
                &msg as *const T as _,
                core::mem::size_of::<T>() as _,
            )
        };

        ret == 0
    }

    pub fn blocking_send(&self, msg: T, timeout: u32) -> bool {
        let ret = unsafe {
            ffi::rt_mq_send_wait(
                self.raw,
                &msg as *const T as _,
                core::mem::size_of::<T>() as _,
                timeout as _,
            )
        };

        ret == 0
    }

    pub fn send_urgent(&self, msg: T) -> bool {
        let ret = unsafe {
            ffi::rt_mq_urgent(
                self.raw,
                &msg as *const T as _,
                core::mem::size_of::<T>() as _,
            )
        };

        ret == 0
    }

    pub fn recv(&self) -> Option<T> {
        let mut msg = core::mem::MaybeUninit::<T>::uninit();
        let ret = unsafe {
            ffi::rt_mq_recv(
                self.raw,
                msg.as_mut_ptr() as _,
                core::mem::size_of::<T>() as _,
                ffi::RT_WAITING_FOREVER as _,
            )
        };

        if ret > 0 {
            Some(unsafe { msg.assume_init() })
        } else {
            None
        }
    }
}

impl<T> Drop for MessageQueue<T> {
    fn drop(&mut self) {
        unsafe {
            ffi::rt_mq_delete(self.raw);
        }
    }
}
