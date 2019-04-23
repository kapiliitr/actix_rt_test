#[cfg(test)]
mod tests {
    use actix_rt::{System, Arbiter};
    use futures::future::{ok, Future, lazy};
    use tokio::runtime::current_thread::Runtime;

    #[test]
    fn test_sysrunner() {
        {
            let mut sys = System::new("kapiltest");

            sys.block_on(ok::<(), ()>(()).and_then(|()| {
                println!("{:?}", "system runner hello");
                Ok(())
            })).unwrap();
        }
    }

    #[test]
    fn test_arbiter() {
        {
            let mut sys = System::new("kapiltest");

            Arbiter::spawn(lazy(|| {
                println!("{:?}", "arbiter hello");
                System::with_current(|cur_sys| cur_sys.stop());
                ok(())
            }));

            sys.run().unwrap();
        }
    }

    #[test]
    fn test_tokio() {
        {
            let mut sys = Runtime::new().unwrap();

            sys.spawn(lazy(|| {
                println!("{:?}", "tokio hello");
                ok(())
            }));

            sys.run().unwrap();
        }
    }
}