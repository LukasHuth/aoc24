#[macro_export]
macro_rules! build_execute {
    ($x:expr, $($day:tt),*) => {
        match $x {
        $(
            $day => {paste::paste! {
                [< day $day >]::[< Day $day >]::run()
            }},
        )*
        _ => panic!("The specified day is not available"),
        }
    };
}
#[macro_export]
macro_rules! build_run {
    ($day:tt, $function_1:ident$(, $function_2:ident)?) => {
        paste::paste! {
        pub struct [< Day $day >];
        impl crate::RunDay for [< Day $day >]  {
            #[allow(unreachable_code)]
            fn run() -> std::time::Duration {
                let start = std::time::Instant::now();
                let result = $function_1();
                let run_time = start.elapsed();
                println!("The result of part 1 is: {result}");
                println!("It took: {run_time:?}");
                $(
                let start_2 = std::time::Instant::now();
                let result = $function_2();
                let run_time_2 = start_2.elapsed();
                println!("The result of part 2 is: {result}");
                println!("It took: {run_time_2:?}");
                return run_time + run_time_2;
                )?
                return run_time;
            }
        }
        }
    };
}
#[macro_export]
macro_rules! build_mods {
    ($($day:tt),+) => {
        paste::paste! {
        $(
            mod [< day $day >];
        )*
        }
    };
}
