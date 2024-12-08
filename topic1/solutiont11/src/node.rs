use std::cmp::Ordering::{Equal, Greater, Less};
use std::ptr::NonNull;

#[repr(transparent)]
pub(crate) struct Node {
    pub(crate) next: Option<NonNull<Node>>,
}

impl Node {
    /// 插入结点，如果插入成功返回 `true`。
    /// 如果目标结点存在，则返回 `false`，且存在的结点也被移除。
    ///
    /// # Notice
    ///
    /// 这个函数可以尾递归的，但 Rust 并不支持优化尾递归。
    #[inline]
    pub(crate) fn insert(&mut self, mut node: NonNull<Node>, buddy: NonNull<Node>) -> bool {
        let mut cursor = self;
        loop {
            if let Some(mut next) = cursor.next {
                use core::cmp::Ordering::*;
                match next.cmp(&buddy) {
                    // 新结点更大，找下一个
                    Less => cursor = unsafe { next.as_mut() },
                    // 相等，移除这一个
                    Equal => {
                        cursor.next = unsafe { next.as_ref().next };
                        unsafe { node.as_mut() }.next = None;
                        break false;
                    }
                    // 新结点更小，插入
                    Greater => {
                        cursor.next = Some(node);
                        unsafe { node.as_mut() }.next = Some(next);
                        break true;
                    }
                }
            } else {
                // 没有下一个，插入
                cursor.next = Some(node);
                unsafe { node.as_mut() }.next = None;
                break true;
            }
        }
    }

    /// 直接在头结点插入。
    #[inline]
    pub(crate) fn insert_unordered(&mut self, mut node: NonNull<Node>) {
        unsafe { node.as_mut() }.next = core::mem::replace(&mut self.next, Some(node));
    }

    /// 直接取下头结点。
    #[inline]
    pub(crate) fn take_any(&mut self) -> Option<NonNull<Node>> {
        let root = self.next;
        self.next = root.and_then(|node| unsafe { node.as_ref().next });
        root
    }
}
