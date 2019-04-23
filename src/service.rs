use actix_rt::{System, Arbiter};
use futures::future::{ok, Future, lazy};
use tokio::runtime::current_thread::Runtime;

pub fn run() {
    // Uncomment one of the three blocks to run

    /*
    {
        let mut sys = System::new("kapiltest");

        sys.block_on(ok::<(), ()>(()).and_then(|()| {
            println!("{:?}", "system runner hello");
            Ok(())
        })).unwrap();

        sys.run().unwrap();
    }
    */

    /*
    {
        let mut sys = System::new("kapiltest");

        Arbiter::spawn(lazy(|| {
            println!("{:?}", "arbiter hello");
            ok(())
        }));

        sys.run().unwrap();
    }
    */

    /*
    {
        let mut sys = Runtime::new().unwrap();

        sys.spawn(lazy(|| {
            println!("{:?}", "tokio hello");
            ok(())
        }));

        sys.run().unwrap();
    }
    */
}