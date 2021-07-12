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
declare!(NMK_START_MODE);
declare!(NMK_TMUX_VERSION);
declare!(NMK_ZSH_GLOBAL_RCS);
declare!(PATH);
declare!(VIMINIT);
declare!(ZDOTDIR);
