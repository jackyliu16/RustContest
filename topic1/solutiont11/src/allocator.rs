use std::alloc::{GlobalAlloc, Layout};
use std::fmt;
use std::num::NonZeroUsize;
use std::ptr::NonNull;
use crate::linked_list::LinkedNodeList;

const MIN_ORDER: usize = 3;

pub(crate) struct MyAllocator<const N: usize> {
    free: usize,
    capacity: usize,
    /// 对应的是不同的 level
    each_level: [LinkedNodeList; N],
}

impl<const N: usize> MyAllocator<N>{
    #[inline]
    pub const fn new() -> Self {
        Self {
            free: 0,
            capacity: 0,
            each_level: [LinkedNodeList::EMPTY; N],
        }
    }

    #[inline]
    pub fn capacity(&self) -> usize {self.capacity}
    #[inline]
    pub fn free(&self) -> usize {self.free}

    /// 将一个 `ptr` 指向的长度为 `usize` 的内存块转移给分配器。
    ///
    /// # Safety
    ///
    /// 调用者需要保证：
    ///
    /// - 这个内存块没有被其他任何对象引用；
    /// - 这个内存块和已经托管的内存块不重叠。
    #[inline]
    pub unsafe fn transfer<T>(&mut self, ptr: NonNull<T>, size: usize) {
        self.capacity += size;
        self.deallocate(ptr, size)
    }

    fn deallocate<T>(&mut self, ptr: NonNull<T>, size: usize) {
        let mut ptr = ptr.as_ptr() as usize;
        let end = ptr + size;

        loop {
            // 剩余长度
            let len = nonzero(end - ptr);
            // 指针的对齐决定最大阶数
            let order_ptr = nonzero(ptr).trailing_zeros();
            // 长度向下取整也决定最大阶数
            let order_len = usize::BITS - len.leading_zeros() - 1;
            // 实际阶数是两个最大阶数中较小的那个
            let order = order_ptr.min(order_len) as usize;
            let order = order.min(N) as usize;

            let mut idx = ptr >> order;
            ptr += 1 << order;

            for layer in (order - MIN_ORDER).. {
                match self.each_level[layer].put(idx) {
                    Some(parent) => idx = parent,
                    None => break,
                }
            }
        }
    }

    fn allocate<T>(&mut self, layout: Layout) -> Option<(NonNull<T>, usize)> {
        #[inline]
        const fn allocated<T, U>(ptr: *mut T, size: usize) -> (NonNull<U>, usize) {
            (unsafe { NonNull::new_unchecked(ptr) }.cast(), size)
        }

        todo!()
    }
}



#[inline]
const fn nonzero(val: usize) -> NonZeroUsize {
    unsafe { NonZeroUsize::new_unchecked(val) }
}

impl<const N: usize> fmt::Debug for MyAllocator<N>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "BuddyAllocator@{:#018x}", self as *const _ as usize)?;
        writeln!(f, "---------------------------------")?;
        Ok(for (i, line) in self.each_level.iter().enumerate() {
            writeln!(f, "{:>2}> {line:?}", MIN_ORDER + i)?;
        })
    }
}

