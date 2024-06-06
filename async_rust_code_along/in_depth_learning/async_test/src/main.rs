#![allow(unused_variables)]
/// Trait std::future::Future
/// https://doc.rust-lang.org/beta/std/future/trait.Future.html
/// trait can be implemented on custom types
/// simplified version of the future trait:
/* 
pub trait Future {
    type Output;
    // Required method
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
enum Poll<T> {
    Ready(T),
    Pending,
}
*/

use futures::{join, pin_mut, select};
use futures::future::FutureExt;

fn main() {
    let num = get_number1(); // nothing happens here because we don't have an executor
    
    // using runtime/executor to invoke future
    // the smol crate is a small and fast async runtime
    // smol::block_on() blocks the current thread on a future, 
    // processing I/O events when idle
    let result = smol::block_on(num);
    println!("waiting for the future, final value id {}", result);

    // kicking off multiple futures at the same time with the futures crate
    // potential futures can be executed at the same time using the join and select macro
    // it means that they are in progress at the same time, not necessarily worked on
    let num1 = get_number1(); // nothing happens here because we don't have an executor
    let num2 = get_number2();
    let num3 = get_number3();

    let result = smol::block_on(
        // join has to be used inside an async block
        async {
            join!(num1, num2, num3)
        }
    );
    println!("waiting for many futures, final value id {:?}", result);

    // select allows us to examine the different futures 
    // and run some code once a specific future is done
    let num4 = get_number1().fuse(); 
    let num5 = get_number2().fuse();
    let num6 = get_number3().fuse();

    pin_mut!(num4, num5, num6);

    let result = smol::block_on(
        // join has to be used inside an async block
        async {
            loop {
                select!{
                    x = num4 => println!("num4 is completed -> {}", x),
                    x = num5 => println!("num5 is completed -> {}", x),
                    x = num6 => println!("num6 is completed -> {}", x),
                    complete => {
                        println!("we're finished, breaking out of loop");
                        break;
                    }     
                }
            }
        }
    );
    // println!("waiting for many futures selectively, final value id {:?}", result);

}

// async syntax returns a future
async fn get_number1() -> u8 {
    4
}

async fn get_number2() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(50));
    10
}

async fn get_number3() -> u8 {
    std::thread::sleep(std::time::Duration::from_millis(75));
    9
}
