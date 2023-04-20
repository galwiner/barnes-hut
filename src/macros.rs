macro_rules! static_rate_limit {
    ($interval: expr) => {{
        static_rate_limit!($interval, true)
    }};
    ($interval: expr, $body: expr) => {{
        use std::cell::RefCell;
        use std::time::{Duration, Instant};

        thread_local! (static LAST_LOGGED_AT: RefCell<Option<Instant>> = RefCell::new(None));
        let run_now = LAST_LOGGED_AT.with(|last_logged_at| {
            last_logged_at
                .borrow()
                .map(|last_logged_at| last_logged_at.elapsed() >= $interval)
                .unwrap_or(true)
        });
        if run_now {
            LAST_LOGGED_AT.with(|last_logged_at| {
                *last_logged_at.borrow_mut() = Some(Instant::now());
            });
            $body
        } else {
            Default::default()
        }
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_static_rate_limit() {
        {
            use std::cell::RefCell;
            use std::time::{Duration, Instant};

            thread_local!(static LAST_LOGGED_AT: RefCell<Option<Instant>> = RefCell::new(None));
            let run_now = LAST_LOGGED_AT.with(|last_logged_at| {
                last_logged_at
                    .borrow()
                    .map(|last_logged_at| last_logged_at.elapsed() >= Duration::from_secs(1))
                    .unwrap_or(true)
            });
            if run_now {
                LAST_LOGGED_AT.with(|last_logged_at| {
                    *last_logged_at.borrow_mut() = Some(Instant::now());
                });
                true
            } else {
                Default::default()
            }
        };

        // assert_eq!(
        //     [0, 1, 2].map(|_| static_rate_limit!(std::time::Duration::from_secs(1), true)),
        //     [true, false, false]
        // );
    }
}
