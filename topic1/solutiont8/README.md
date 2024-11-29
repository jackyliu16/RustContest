主要算法来源：
https://oi-wiki.org/math/number-theory/pollard-rho/#__tabbed_1_2

bmul 被替换成为一个在 rust 内不会出现溢出的针对于 (a * b) mod m 的二进制优化版本，
采用类似于 二进制乘法 的方式，在每次运算之后都计算 mod 以避免可能出现的溢出情况。