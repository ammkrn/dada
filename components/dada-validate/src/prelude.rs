use dada_ir::{code::validated, filename::Filename, function::Function, item::Item};

#[extension_trait::extension_trait]
pub impl DadaValidateFilenameExt for Filename {
    fn validate_root(self, db: &dyn crate::Db) {
        crate::validate::root_definitions(db, self);
    }
}

#[extension_trait::extension_trait]
pub impl DadaValidateFunctionExt for Function {
    fn validated_tree(self, db: &dyn crate::Db) -> validated::Tree {
        crate::validate::validate_function(db, self)
    }
}

#[extension_trait::extension_trait]
pub impl DadaValidateItemExt for Item {
    fn validated_tree(self, db: &dyn crate::Db) -> Option<validated::Tree> {
        match self {
            Item::Function(f) => Some(f.validated_tree(db)),
            Item::Class(_) => None,
        }
    }
}
