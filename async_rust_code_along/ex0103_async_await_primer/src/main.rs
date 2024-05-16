#![allow(dead_code)]
/// coding along with Asynchronous Programming in Rust, ch 1.3. async/.await Primer
/// @ https://rust-lang.github.io/async-book/01_getting_started/04_async_await_primer.html
/// 
/// async transforms a block of code into a state machine that implements a trait called Future
/// blocked Futures will yield control of the thread, allowing other Futures to run
/// value returned by async fn is a Future
/// inside an async fn .await can be used to wait for the completion 
/// of another type that implements the Future trait
/// .await asynchronously waits for the future to complete

// `block_on` blocks the current thread until the provided future has run to
// completion. Other executors provide more complex behavior, like scheduling
// multiple futures onto the same thread.
use futures::executor::block_on;

async fn _hello_world() {
    println!("hello world");
}

// example, learning a song via three async funtions:
// learn_song, sing_song, and dance
#[derive(Debug)]
struct Song {
    lyrics: String,
    melody: String,
} // defining Song as a struct for this example

async fn learn_song() -> Song { 
    let song = Song {
        lyrics: "some_lyrics".to_string(),
        melody: "some_melody".to_string(),
    };
    println!("singing the song {:?}", &song);
    song
}
async fn sing_song(song: Song) { 
    println!("singing the song {:?}", song);    
}
async fn dance() { 
    println!("now let's dance");   
}

// we don't have to do only one thing at once
// we have to learn the song before we can sing it
// but it's possible to dance at the same time as learning and singing the song
// we'll achieve this by creating two separate async fn that can be run concurrently
async fn learn_and_sing() {
    // Wait until the song has been learned before singing it.
    // We use `.await` here rather than `block_on` to prevent blocking the
    // thread, which makes it possible to `dance` at the same time.
    let song = learn_song().await;
    sing_song(song).await;
}

async fn async_main() {
    let f1 = learn_and_sing();
    let f2 = dance();
    
    // `join!` is like `.await` but can wait for multiple futures concurrently.
    // If we're temporarily blocked in the `learn_and_sing` future, the `dance`
    // future will take over the current thread. If `dance` becomes blocked,
    // `learn_and_sing` can take back over. If both futures are blocked, then
    // `async_main` is blocked and will yield to the executor.
    futures::join!(f1, f2);
}

fn main() {
    // let future = hello_world();
    block_on(async_main()); 
}
