use noirc_frontend::{
    ast::{ItemVisibility, LetStatement, Pattern},
    token::Keyword,
};

use super::Formatter;
use crate::chunks::{ChunkFormatter, ChunkGroup};

impl Formatter<'_> {
    pub(super) fn format_global(
        &mut self,
        let_statement: LetStatement,
        visibility: ItemVisibility,
    ) {
        self.format_secondary_attributes(let_statement.attributes.clone());

        let group = self.chunk_formatter().format_global(let_statement, visibility);
        self.write_indentation();
        self.format_chunk_group(group);
    }
}

impl ChunkFormatter<'_, '_> {
    pub(super) fn format_global(
        &mut self,
        let_statement: LetStatement,
        visibility: ItemVisibility,
    ) -> ChunkGroup {
        let mut group = ChunkGroup::new();
        group.text(self.chunk(|formatter| {
            formatter.format_item_visibility(visibility);
        }));

        if let_statement.comptime {
            group.text(self.chunk(|formatter| {
                formatter.write_keyword(Keyword::Comptime);
                formatter.write_space();
            }));
        }

        let pattern = let_statement.pattern;
        let pattern = match pattern {
            Pattern::Identifier(..) => pattern,
            Pattern::Mutable(pattern, _span, _) => {
                // `mut global x` is represented in the AST with a mutable pattern.
                // A mutable pattern would be `mut x` but here we have `mut global x`,
                // so in that case we write `mut` now, then the pattern we'll write doesn't
                // have the `mut` part.
                group.text(self.chunk(|formatter| {
                    formatter.write_keyword(Keyword::Mut);
                    formatter.write_space();
                }));

                *pattern
            }
            Pattern::Tuple(..)
            | Pattern::Struct(..)
            | Pattern::Parenthesized(..)
            | Pattern::Interned(..) => {
                unreachable!("Global pattern cannot be a tuple, struct, parenthesized or interned")
            }
        };

        group.group(self.format_let_or_global(
            Keyword::Global,
            pattern,
            let_statement.r#type,
            Some(let_statement.expression),
            Vec::new(), // Attributes
        ));

        group
    }
}

#[cfg(test)]
mod tests {
    use crate::assert_format;

    #[test]
    fn format_global_without_type() {
        let src = " pub  global  x  =  1  ; ";
        let expected = "pub global x = 1;\n";
        assert_format(src, expected);
    }

    #[test]
    fn format_global_with_type() {
        let src = " pub  global  x  :  Field  =  1  ; ";
        let expected = "pub global x: Field = 1;\n";
        assert_format(src, expected);
    }

    #[test]
    fn format_comptime_global() {
        let src = " pub  comptime  global  x  :  Field  =  1  ; ";
        let expected = "pub comptime global x: Field = 1;\n";
        assert_format(src, expected);
    }

    #[test]
    fn format_comptime_mut_global() {
        let src = " pub  comptime  mut  global  x  :  Field  =  1  ; ";
        let expected = "pub comptime mut global x: Field = 1;\n";
        assert_format(src, expected);
    }

    #[test]
    fn format_global_with_attributes() {
        let src = " #[abi ( foo ) ]  global  x  =  1  ; ";
        let expected = "#[abi(foo)]
global x = 1;
";
        assert_format(src, expected);
    }
}
