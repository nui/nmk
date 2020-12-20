#[rustfmt::skip]
macro_rules! declare {
    ($id: ident) => {
        pub const $id: &str = stringify!($id);
    };
}

declare!(EDITOR);
declare!(LD_LIBRARY_PATH);
declare!(NMK_BIN);
declare!(NMK_HOME);
declare!(NMK_TMP_TMUX_CONF);
declare!(NMK_TMUX_VERSION);
declare!(PATH);
declare!(TMPDIR);
declare!(VIMINIT);
declare!(ZDOTDIR);
