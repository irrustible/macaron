use syn::{custom_keyword, custom_punctuation};

// --- operators ---

custom_punctuation!(Etc, ...);

// --- inbocation markers

// invocation

custom_keyword!(define);
custom_keyword!(call);
custom_keyword!(include);

// ---- builtin fragspecs ----

custom_keyword!(block);
custom_keyword!(expr);
custom_keyword!(ident);
custom_keyword!(item);
custom_keyword!(lifetime);
custom_keyword!(literal);
custom_keyword!(meta);
custom_keyword!(pat);
custom_keyword!(path);
custom_keyword!(stmt);
custom_keyword!(tt);
custom_keyword!(ty);
custom_keyword!(vis);

// extra fragspecs

custom_keyword!(attrs);
// custom_keyword!(attrs);
custom_keyword!(inattrs);
// custom_keyword!(inattrs);
custom_keyword!(name);
custom_keyword!(genarg);
// custom_keyword!(genargs);
custom_keyword!(genparam);
// custom_keyword!(genparams);
