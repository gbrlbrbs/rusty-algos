macro_rules! clone_fields {
    ($name:ident, $($field:ident),+) => (
        fn clone(&self) -> Self {
            $name {
                $(
                    $field : self . $field .clone()
                ),*
            }
        }
    );
}

macro_rules! index_impl {
    ($type:ty) => {
        fn new(x: usize) -> Self {
            x as $type
        }

        fn max() -> Self {
            <$type>::MAX
        }

        fn index(&self) -> usize {
            *self as usize
        }
    }
}