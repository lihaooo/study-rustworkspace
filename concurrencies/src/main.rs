// 区分一下并发和并行
// 并发: 指程序结构上的一种设计，目的是为了让程序可以被分解成 "独立" 的、可以并发执行的子任务
// 并行: 指程序真正执行的一种方式，指同一时刻有多条指令在多个处理器上 "同时" 执行
// 架构上的并行计算，是为了提高程序的执行效率, 将一个任务分解成多个子任务，分配给多个处理者同时执行, 再汇总结果, 从而提高程序的执行效率
// 架构上的并发计算, 是综合考虑, 有可能为了执行效率, 也有可能为了可用性

// rust 采用的是 "线程" 的方式实现并发计算, 但是线程的创建和销毁, 以及线程间的通信, 都是需要消耗资源的, 所以线程的数量是有限的
// 为了提高线程的利用率, 采用了 "线程池" 的方式, 将线程的创建和销毁, 以及线程间的通信, 都交给线程池来管理, 从而提高线程的利用率

// rust 的线程间通信, 通常采用的是 "消息传递" 的方式, 也就是说, 线程间不会共享内存, 而是通过消息传递的方式, 传递数据
// 这和 go 中的不要通过共享内存来通信, 而要通过通信来共享内存的思想是一致的
// 但有些时候, 我们也可以用 mutex 来实现线程间的通信
// mutex 用于共享内存的并发访问, 但是如果多个线程都要访问同一个 mutex, 那么就会出现死锁的风险

// 实现了 Send trait 的类型, 可以跨线程传递, 实现了 Sync trait 的类型, 可以在多个线程中共享
// 所有的基本类型都实现了 Send 和 Sync, 所以可以跨线程传递, 也可以在多个线程中共享
// Rc<T> 没有实现 Send 和 Sync, Arc<T> 实现了 Send 和 Sync, 所以可以跨线程传递, 也可以在多个线程中共享
// Rc<T> 被设计为在单线程中使用, Arc<T> 被设计为在多线程中使用, Rc<T> 性能更好

use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

// concurrencies
fn main() {
    thread::spawn(|| {
        println!("sub thread: Hello, world!");
    });
    println!("main thread: Hello, world!");
    // 等待子线程执行完毕, sub thread: Hello, world! 才会打印出来
    // 发现 sub thread 总是在 main thread 之后打印出来
    thread::sleep(Duration::from_millis(1));

    thread::spawn(|| {
        for i in 1..100 {
            println!("sub thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // 如果在这里就结束, sub thread 有可能还没有执行完毕, 一般最多打印到 5, 有时候是 4 就结束了

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("handle sub thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("handle main thread: {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    // join() 方法会阻塞当前线程, 直到子线程执行完毕, 才会继续执行当前线程
    handle.join().unwrap();
    // 导致下面的主线程会等待 handle 线程执行完毕, 才会继续执行, 但不会阻塞其他线程的执行
    println!("recover main thread: Hello, world!");

    let v = vec![1, 2, 3];
    // move 把 v 移动到闭包中, 闭包中就拥有了 v 的所有权, 所以闭包中可以使用 v, 但是 main 函数中就不能使用 v 了
    let handle = thread::spawn(move || {
        println!("handle sub thread: here is a vector: {:?}", v);
    });
    handle.join().unwrap();
    // handle 结束后, v 也就被释放了, 所以这里不能再使用 v 了
    // println!("main thread: here is a vector: {:?}", v);

    // mpsc = multiple producer, single consumer, 多个生产者, 单个消费者
    // 想象一下多个支流汇聚成一个河流, 多个线程通过通道发送消息, 一个线程通过通道接收消息
    let (tx, rx) = mpsc::channel();
    // 因为 mpsc 是多个生产者, 所以可以有多个发送端
    let tx1 = tx.clone();

    thread::spawn(move || {
        let val = String::from("hello");
        println!("sub thread: send = {}", val);
        // send 方法会将消息发送到通道的另一端, 如果没有接收者, 则会阻塞当前线程
        tx.send(val).unwrap();
        // val 被 send 之后, 就不能再使用了, 因为 send 方法会获取 val 的所有权
        // println!("sub thread: send = {}", val);

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });
    // recv 方法会从通道的另一端接收消息, 如果没有发送者, 则会阻塞当前线程
    // try_recv 方法不会阻塞当前线程, 如果没有接收到消息, 则会返回一个错误, 这样就可以通过循环来不断尝试接收消息
    let recv = rx.recv().unwrap();
    println!("main thread: recv = {}", recv);

    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_millis(1));
        }
    });

    // 这里的 for 循环会一直阻塞, 直到通道的发送端被关闭, 才会结束循环
    for recv in rx {
        println!("main thread: recv = {}", recv);
    }

    // 单线程中的 mutex, Mutex 是一个智能指针, 内部包含了一个数据和一个 MutexGuard, MutexGuard 实现了 Deref 和 Drop
    let m = Mutex::new(5);
    {
        let mut num = m.lock().unwrap();
        // 代码段中对 num 的解引用, 即 m 进行赋值, 会被锁住, 直到 num 被 drop 为止
        *num = 6;
        // num 被 drop
    }
    // m 的值被修改为 6
    println!("m = {:?}", m);

    // 多线程中的 mutex, Arc = atomic reference count, 原子引用计数, 用于在多线程中共享所有权
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];
    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 如果不使用 Arc, 不能在多个线程中共享 counter
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }
    // 等待所有线程执行完毕
    for handle in handles {
        handle.join().unwrap();
    }
    println!("counter = {:?}", counter);
}
