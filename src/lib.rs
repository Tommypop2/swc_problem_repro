use std::sync::Arc;

use swc_core::ecma::transforms::typescript::strip;
use swc_core::{
    common::{
        errors::{ColorConfig, Handler},
        FileName, Mark, SourceMap, GLOBALS,
    },
    ecma::{
        ast::EsVersion,
        parser::{Syntax, TsConfig},
        transforms::testing::test,
        visit::{FoldWith, VisitMut},
    },
};
use wasm_bindgen::prelude::wasm_bindgen;

pub struct TransformVisitor;

impl VisitMut for TransformVisitor {
    // Implement necessary visit_mut_* methods for actual custom transform.
    // A comprehensive list of possible visitor methods can be found here:
    // https://rustdoc.swc.rs/swc_ecma_visit/trait.VisitMut.html
}

// #[plugin_transform]
// pub fn process_transform(program: Program, _metadata: TransformPluginProgramMetadata) -> Program {
//     program.fold_with(&mut as_folder(TransformVisitor))
// }
#[wasm_bindgen]
pub fn transform(code: String, id: String) -> String {
    let cm: Arc<SourceMap> = Arc::<SourceMap>::default();
    let handler: Handler =
        Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));
    let compiler: swc::Compiler = swc::Compiler::new(cm.clone());

    let fm: Arc<swc_core::common::SourceFile> = cm.new_source_file(FileName::Custom(id), code);
    GLOBALS.set(&Default::default(), || {
        let mut result = compiler.parse_js(
            fm,
            &handler,
            EsVersion::EsNext,
            Syntax::Typescript(TsConfig {
                tsx: true,
                decorators: false,
                dts: false,
                no_early_errors: false,
                disallow_ambiguous_jsx_like: true,
            }),
            swc::config::IsModule::Bool(true),
            None,
        );
        // Commenting out the following 2 lines means that correct js is generated
        let top_level_mark = Mark::new();
        result = Ok(result.unwrap().fold_with(&mut strip(top_level_mark)));
        //
        let out = compiler.print(
            &result.unwrap(),
            None,
            None,
            false,
            EsVersion::Es2015,
            swc::config::SourceMapsConfig::Bool(false),
            &Default::default(),
            None,
            false,
            None,
            false,
            false,
            "",
        );
        out.unwrap().code
    })
}

#[test]
fn test_transform() {
    let result = transform("let integer: number = 123;".into(), "main.ts".into());
    assert_eq!(result, "let integer = 123;\n")
}
