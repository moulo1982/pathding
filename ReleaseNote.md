# Release Note

`0.0.1` 初始版本，划分了文件结构

`0.0.2` 利用RefCell和Rc来解决点引用的问题。

`0.0.3` 发现trait的结构不正确，应该为：Map是一个trait，AStart或者Recast来实现它。

`0.0.4` PartialEq 这个trait原来也可以用来比较两个不同类型之间的比较，但是A == B 和 B == A需要实现两遍

`0.0.5` 2023.12月25日：尝试tokio以 多线程+携程+await 的模式来运行，MapManager是一个单例，new_astar需要写锁，但是find_path只需要读锁。
当中试图用到了Rust 1.75的新特性，也就是：trait中可以有async方法，这个是个很重要的修改。 

Rust 1.75 两个重大的修改就是：
1：trait支持async方法
2：返回类型支持 impl Trait

Rust团队承诺2023.12.28发布1.75 [Rust 1.75.0](https://releases.rs/docs/1.75.0/)

因为tokio的mutex.lock是await的，所以有些方法的签名必须是async，
等1.75发布就修改trait Map的find_path位async


`0.0.6` 优化一些不必要的锁，和一些返回类型

`0.0.7` 优化MapManager单例获取的方式

`0.0.8` 优化IdGenerator单例的获取方式，简化id生成的规则：（64位时间戳+64位序号）

`0.0.9` 实现A*算法，但是比较粗糙，16*16格子，从(1，0)寻到(14，15)，循环1000次，用时100ms左右

`0.0.10` 重大优化，将openList中存储点的数据结构改为BTreeMap<i64, Vec<PointType>> 
        其中key代表A*算法中的F，这样快速取出F最小的点只要first_entry就可以，然后从Vec中pop最后一个即可。
        查询一个点在BTreeMap是否存在，通过F取出vec后，循环判断。如果要删除就从vec中删除。
        优化后：
        16*16格子，从(1，0)寻到(14，15)，循环1000次，用时（20ms）左右.
        32*32格子，从(1，0)寻到(30，31)，循环1000次，用时（45ms）左右.