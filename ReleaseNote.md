# Release Note

`0.0.1` 初始版本，划分了文件结构

`0.0.2` 利用RefCell和Rc来解决点引用的问题。

`0.0.3` 发现trait的结构不正确，应该为：Map是一个trait，AStart或者Recast来实现它。

`0.0.4` PartialEq 这个trait原来也可以用来比较两个不同类型之间的比较，但是A == B 和 B == A需要实现两便