macro_rules! static_rate_limit {
    (secs = $interval: expr) => {{
        static_rate_limit!($interval, true)
    }};
    (secs = $interval: expr, $body: expr) => {{
        use std::cell::RefCell;
        use std::time::{Instant};

        thread_local! (static LAST_LOGGED_AT: RefCell<Option<Instant>> = RefCell::new(None));
        let run_now = LAST_LOGGED_AT.with(|last_logged_at| {
            last_logged_at
                .borrow()
                .map(|last_logged_at| last_logged_at.elapsed().as_secs_f32() >= $interval as f32)
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

#[allow(unused_macros)]
macro_rules! at_most {
    ($a: expr, $b: expr) => {{
        ($a).min($b)
    }};
    ($a: expr, $b: expr, $($rest: expr),+) => {{
        at_most!($a, at_most!($b, $($rest),+))
    }};
}

#[allow(unused_macros)]
macro_rules! at_least {
    ($a: expr, $b: expr) => {{
        ($a).max($b)
    }};
    ($a: expr, $b: expr, $($rest: expr),+) => {{
        at_least!($a, at_least!($b, $($rest),+))
    }};
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_static_rate_limit() {
        assert_eq!(
            [0, 1, 2].map(|_| static_rate_limit!(secs = 1, true)),
            [true, false, false]
        );
    }

    #[test]
    fn test_at_most() {
        assert_eq!(at_most!(1, 2), 1);
        assert_eq!(at_most!(2, 1), 1);
        assert_eq!(at_most!(3, 1, 2), 1);
    }

    #[test]
    fn test_at_least() {
        assert_eq!(at_least!(1, 2), 2);
        assert_eq!(at_least!(2, 1), 2);
        assert_eq!(at_least!(1, 3, 2), 3);
    }
}
