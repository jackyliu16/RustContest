use std::ptr::NonNull;

/// 阶数。
///
/// 用于侵入式行序号到指针的转换。
pub(crate) struct Order(pub(crate) usize);

impl Order {
    /// 创建一个 Order 实例
    #[inline]
    pub(crate) const fn new(order: usize) -> Self {
        Self(order)
    }

    /// 索引 -> 指针
    /// 使用阶数和索引计算出指针的地址
    #[inline]
    pub(crate) unsafe fn idx_to_ptr<T>(&self, idx: usize) -> NonNull<T> {
        NonNull::new_unchecked((idx << self.0) as *mut _)
    }

    /// 指针 -> 索引
    /// 通过右移操作将地址转化回原始索引
    #[inline]
    pub(crate) fn ptr_to_idx<T>(&self, ptr: NonNull<T>) -> usize {
        (ptr.as_ptr() as usize) >> self.0
    }
}
