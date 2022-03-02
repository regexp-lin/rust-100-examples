/*
题目：学习使用按位取反~。

说明：

二进制数在内存中以补码的形式存储。

按位取反：二进制每一位取反，0 变 1，1 变 0。

最高位为符号位，正数的符号位为 0，负数为 1。

对正数来说，最高位为 0，其余各位代表数值本身(以二进制表示)，如 +42 的补码为 00101010。

对负数而言，把该数绝对值的补码按位取反，然后对整个数加 1，即得该数的补码。如 -42 的补码为 11010110(00101010 按位取反
11010101+1 即 11010110)。

~9 的计算步骤：
 */
fn main() {
    println!("Hello, world!");

    let x = 0b10101010101011100101;

    println!("{:b} {:b}", x, !x);
}
