use crate::{
    code::UnparsedCode,
    effect::Effect,
    filename::Filename,
    return_type::ReturnType,
    span::FileSpan,
    word::{SpannedWord, Word},
};

salsa::entity2! {
    entity Function in crate::Jar {
        #[id] name: SpannedWord,

        /// Declared effect for the function body -- e.g., `async fn` would have
        /// this be `async`. This can affect validation and code generation.
        effect: Effect,

        /// If this func has a declared effect, this is the span of that keyword (e.g., `async`)
        /// Otherwise, it is the span of the `fn` keyword.
        effect_span: FileSpan,

        /// Return type of the function.
        return_type: ReturnType,

        /// The body and parameters of functions are only parsed
        /// on demand by invoking (e.g.) `syntax_tree` from the
        /// `dada_parse` crate.
        unparsed_code: UnparsedCode,

        /// Overall span of the function (including the code)
        span: FileSpan,
    }
}

impl<Db: ?Sized + crate::Db> salsa::DebugWithDb<Db> for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>, db: &Db) -> std::fmt::Result {
        let db = db.as_dyn_ir_db();
        write!(f, "{}", self.name(db).as_str(db))
    }
}

impl Function {
    pub fn filename(self, db: &dyn crate::Db) -> Filename {
        self.span(db).filename
    }
}

salsa::entity2! {
    entity Variable in crate::Jar {
        #[id] name: Word,
    }
}
