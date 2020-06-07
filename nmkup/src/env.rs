macro_rules! declare {
    ($id: ident) => {pub const $id: &str = stringify!($id);};
}

declare!(DISPLAY);
declare!(EDITOR);
declare!(LD_LIBRARY_PATH);
declare!(NMK_BIN);
declare!(NMK_DIR);
declare!(PATH);
declare!(VIMINIT);
declare!(VIRTUAL_ENV);
declare!(WINDOWID);
declare!(ZDOTDIR);
