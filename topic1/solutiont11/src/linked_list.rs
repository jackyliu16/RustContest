// From BuddyAllocator
use core::{fmt, ptr::NonNull};
use crate::order::Order;
use crate::node::Node;

/// 侵入式链表 Allocation Free List
pub struct LinkedNodeList {
    free_list: Node,
    order: Order,
}

/// 必须实现 [`Send`] 才能加锁。
unsafe impl Send for LinkedNodeList {}

impl fmt::Debug for LinkedNodeList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[")?;
        let mut cursor = &self.free_list;
        while let Some(next) = cursor.next {
            write!(f, "{:#x}, ", self.order.ptr_to_idx(next))?;
            cursor = unsafe { next.as_ref() };
        }
        write!(f, "]")
    }
}

impl LinkedNodeList {
    const INTRUSIVE_META_SIZE: usize = core::mem::size_of::<Node>();

    pub const EMPTY: Self = Self {
        free_list: Node { next: None },
        order: Order::new(0),
    };

    #[inline]
    fn init(&mut self, order: usize, _base: usize) {
        self.order = Order::new(order);
    }

    /// 从当前肯用空闲列表中获取某个满足 align_order 的内存块的 idx
    fn take_any(&mut self, align_order: usize) -> Option<usize> {
        self.free_list
            .take_any()
            .map(|ptr| unsafe { self.order.ptr_to_idx(ptr) })
    }

    /// 插入某个特定 idx 内存块
    /// 如果 node 和 他的 buddy 同时存在, 则更新并且返回上一个大小级别的 阶数
    pub(crate) fn put(&mut self, idx: usize) -> Option<usize> {
        let node = unsafe { self.order.idx_to_ptr(idx) };
        let buddy = unsafe { self.order.idx_to_ptr(idx ^ 1) };

        if !self.free_list.insert(node, buddy) {
            None
        } else {
            // 如果目标节点不存在, 则返回
            Some(idx >> 1)
        }
    }
}