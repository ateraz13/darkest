
#[macro_export]
macro_rules! impl_error_conv {
    ($from:ty, $to:ty, $variant:ident) => {

        impl From<$from> for $to {

            fn from(e: $from) -> $to {
                <$to>::$variant(e)
            }

        }

    }
}

#[macro_export]
macro_rules! benchmark {
    {$name:expr; $a:stmt} => {
        let timer = std::time::Instant::now();
        $a
            let time = timer.elapsed();
        println!("benchmark \"{}\": {}ms", $name, time.as_millis());
    }
}
