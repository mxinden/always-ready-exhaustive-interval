use futures_timer::Interval;
use futures::task::{ Poll, Context };
use futures::prelude::*;
use futures::Stream;
use std::time::{ Instant, Duration };
use std::pin::Pin;

struct Clock {
    second: Interval,
    decisecond: Interval,
}

impl Clock {
    fn new() -> Clock {
        Clock{
            second: Interval::new_at(Instant::now(), Duration::from_secs(1)),
            decisecond: Interval::new_at(Instant::now(), Duration::from_secs_f32(0.1)),
        }
    }

    async fn print(&mut self) {
        //////////////////////////////////////////////////////////////
        // Print second

        // Check whether a second passed. If not, move on - do *not* block.
        let second = futures::future::poll_fn(|cx: &mut Context<'_>| -> Poll<bool> {
            match Stream::poll_next(Pin::new(&mut self.second), cx) {
                Poll::Ready(Some(())) => {
                    // One should consume all elements in the stream to ensure
                    // the task is registered for wakeup once another second
                    // passes. Omitted here for simplicity.
                    return Poll::Ready(true)
                }
                Poll::Ready(None) => {
                    panic!("stream should never end");
                }
                Poll::Pending => {
                    return Poll::Ready(false)
                }
            }
        }).await;

        if second {
            println!("new second");
        }

        //////////////////////////////////////////////////////////////
        // Print decisecond

        // Check whether a decisecond passed. Given that this is the smallest
        // unit of the clock, it is safe to block.
        self.decisecond.next().await;
        println!("new decisecond");
    }
}

///////////////////////////////////////////////////////////////////////
// Expected test output
//
// > running 1 test
// >     new second
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new second
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     new decisecond
// >     [...]

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        futures::executor::block_on(async {
            let mut  c = Clock::new();

            let start = Instant::now();

            // Give intervals some time to get ready.
            std::thread::sleep(Duration::from_millis(1));

            while Instant::now() - start < Duration::from_secs(120) {
                c.print().await;
            }
        })
    }
}
