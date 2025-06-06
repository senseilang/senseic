use async_lsp::lsp_types::CompletionItemKind;
use noirc_frontend::{ast::AttributeTarget, elaborator::PrimitiveType, token::Keyword};
use strum::IntoEnumIterator;

use super::{
    NodeFinder,
    completion_items::{
        completion_item_with_trigger_parameter_hints_command, simple_completion_item,
        snippet_completion_item,
    },
    kinds::FunctionCompletionKind,
    name_matches,
};

impl NodeFinder<'_> {
    pub(super) fn builtin_functions_completion(
        &mut self,
        prefix: &str,
        function_completion_kind: FunctionCompletionKind,
    ) {
        for keyword in Keyword::iter() {
            if let Some(func) = keyword_builtin_function(&keyword) {
                if name_matches(func.name, prefix) {
                    let description = Some(func.description.to_string());
                    let label;
                    let insert_text;
                    match function_completion_kind {
                        FunctionCompletionKind::Name => {
                            label = func.name.to_string();
                            insert_text = func.name.to_string();
                        }
                        FunctionCompletionKind::NameAndParameters => {
                            label = format!("{}(…)", func.name);
                            insert_text = format!("{}({})", func.name, func.parameters);
                        }
                    }

                    self.completion_items.push(
                        completion_item_with_trigger_parameter_hints_command(
                            snippet_completion_item(
                                label,
                                CompletionItemKind::FUNCTION,
                                insert_text,
                                description,
                            ),
                        ),
                    );
                }
            }
        }
    }

    pub(super) fn builtin_values_completion(&mut self, prefix: &str) {
        for keyword in ["false", "true"] {
            if name_matches(keyword, prefix) {
                self.completion_items.push(simple_completion_item(
                    keyword,
                    CompletionItemKind::KEYWORD,
                    Some("bool".to_string()),
                ));
            }
        }
    }

    pub(super) fn builtin_types_completion(&mut self, prefix: &str) {
        for primitive_type in PrimitiveType::iter() {
            let name = primitive_type.name();
            if name_matches(name, prefix) {
                self.completion_items.push(simple_completion_item(
                    name,
                    CompletionItemKind::STRUCT,
                    Some(name.to_string()),
                ));
            }
        }
    }

    pub(super) fn suggest_builtin_attributes(&mut self, prefix: &str, target: AttributeTarget) {
        match target {
            AttributeTarget::Module => (),
            AttributeTarget::Trait => {
                self.suggest_allow("dead_code", prefix);
            }
            AttributeTarget::Struct => {
                self.suggest_one_argument_attributes(prefix, &["abi"]);
                self.suggest_allow("dead_code", prefix);
            }
            AttributeTarget::Enum => {
                self.suggest_one_argument_attributes(prefix, &["abi"]);
                self.suggest_allow("dead_code", prefix);
            }
            AttributeTarget::Function => {
                let no_arguments_attributes = &[
                    "contract_library_method",
                    "deprecated",
                    "export",
                    "fold",
                    "no_predicates",
                    "recursive",
                    "test",
                    "fuzz",
                    "varargs",
                ];
                self.suggest_no_arguments_attributes(prefix, no_arguments_attributes);

                let one_argument_attributes = &["abi", "field", "foreign", "oracle"];
                self.suggest_one_argument_attributes(prefix, one_argument_attributes);

                if name_matches("deprecated", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "deprecated(\"...\")",
                        CompletionItemKind::METHOD,
                        "deprecated(\"${1:message}\")",
                        None,
                    ));
                }

                if name_matches("test", prefix) || name_matches("should_fail", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "test(should_fail)",
                        CompletionItemKind::METHOD,
                        "test(should_fail)",
                        None,
                    ));
                }

                if name_matches("test", prefix) || name_matches("should_fail_with", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "test(should_fail_with = \"...\")",
                        CompletionItemKind::METHOD,
                        "test(should_fail_with = \"${1:message}\")",
                        None,
                    ));
                }

                if name_matches("fuzz", prefix) || name_matches("only_fail_with", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "fuzz(only_fail_with = \"...\")",
                        CompletionItemKind::METHOD,
                        "fuzz(only_fail_with = \"${1:message}\")",
                        None,
                    ));
                }

                if name_matches("fuzz", prefix) || name_matches("should_fail", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "fuzz(should_fail)",
                        CompletionItemKind::METHOD,
                        "fuzz(should_fail)",
                        None,
                    ));
                }

                if name_matches("fuzz", prefix) || name_matches("should_fail_with", prefix) {
                    self.completion_items.push(snippet_completion_item(
                        "fuzz(should_fail_with = \"...\")",
                        CompletionItemKind::METHOD,
                        "fuzz(should_fail_with = \"${1:message}\")",
                        None,
                    ));
                }

                self.suggest_allow("dead_code", prefix);
            }
            AttributeTarget::Let => {
                self.suggest_allow("unused_variables", prefix);
            }
        }
    }

    fn suggest_allow(&mut self, name: &'static str, prefix: &str) {
        if name_matches("allow", prefix) || name_matches(name, prefix) {
            self.completion_items.push(simple_completion_item(
                format!("allow({name})"),
                CompletionItemKind::METHOD,
                None,
            ));
        }
    }
}

pub(super) struct BuiltInFunction {
    pub(super) name: &'static str,
    pub(super) parameters: &'static str,
    pub(super) description: &'static str,
}

/// If a keyword corresponds to a built-in function, returns info about it
pub(super) fn keyword_builtin_function(keyword: &Keyword) -> Option<BuiltInFunction> {
    match keyword {
        Keyword::Assert => Some(BuiltInFunction {
            name: "assert",
            parameters: "${1:predicate}",
            description: "fn(T)",
        }),
        Keyword::AssertEq => Some(BuiltInFunction {
            name: "assert_eq",
            parameters: "${1:lhs}, ${2:rhs}",
            description: "fn(T, T)",
        }),

        Keyword::As
        | Keyword::Break
        | Keyword::CallData
        | Keyword::Comptime
        | Keyword::Constrain
        | Keyword::Continue
        | Keyword::Contract
        | Keyword::Crate
        | Keyword::Dep
        | Keyword::Else
        | Keyword::Enum
        | Keyword::Fn
        | Keyword::For
        | Keyword::Global
        | Keyword::If
        | Keyword::Impl
        | Keyword::In
        | Keyword::Let
        | Keyword::Loop
        | Keyword::Match
        | Keyword::Mod
        | Keyword::Mut
        | Keyword::Pub
        | Keyword::Return
        | Keyword::ReturnData
        | Keyword::Struct
        | Keyword::Super
        | Keyword::Trait
        | Keyword::Type
        | Keyword::Unchecked
        | Keyword::Unconstrained
        | Keyword::Unsafe
        | Keyword::Use
        | Keyword::Where
        | Keyword::While => None,
    }
}
