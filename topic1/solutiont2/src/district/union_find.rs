//! 并查集
//! 
//! # 实现原理:
//! 存在两个矩阵 data = Vec<data>; self.fa = Vec<usize>; 
//! - data.len() == self.fa.len(); 二者一一对应
//! - self.fa.get(idx) 代表着对应 data.get(idx) 所从属的集合类别
//! - 集合类别以另一元素的 idx 作为标记 (采用路径压缩, 因此多个归属于统一集合的元素会具有同一数值)
//!     - 如 self.fa[3] = 7 -> data 中第四个元素与第七个元素从属于同一个集合
#[derive(Debug, Clone)]
pub struct DSU {
    /// 一个代表一系列元素的集合指示符的 vector
    fa: Vec<usize>,
}

impl DSU {
    pub fn with_capacity(size: usize) -> Self {
       DSU { fa: (0..size).collect() } 
    }

    /// 
    fn find(&mut self, x: usize) -> usize {
        // 当前节点是否是 孤立的叶子节点 
        if self.fa[x] != x {
            self.fa[x] = self.find(self.fa[x]);
        }
        self.fa[x]
    }

    // 如果二者不归属于同一个集合, 则将 x 的集合指示符设定为 y 的集合指示符
    pub fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.fa[root_x] = root_y;
        }
    }
    
    pub fn count_sets(&self) -> usize {
        let mut cnt: usize = 0;
        for idx in 0..self.fa.len() {
            if self.fa.get(idx) == Some(&idx) {
                cnt += 1;
            }
        }
        cnt
    }
}