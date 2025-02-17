use rnix::{
    ast::Ident,
    NixLanguage, SyntaxKind,
};
use rowan::api::SyntaxNode;

/// This string in a Nix comment above an unused declaration shall
/// force us to skip it.
///
/// ```nix
/// let
///   # deadnix: skip
///   skeletonsInTheBasement =
/// ```
const PRAGMA_SKIP: &str = "deadnix: skip";

#[derive(Debug, Clone)]
pub struct Binding {
    pub name: Ident,
    pub body_node: SyntaxNode<NixLanguage>,
    pub decl_node: SyntaxNode<NixLanguage>,
    mortal: bool,
}

impl Binding {
    pub fn new(
        name: Ident,
        body_node: SyntaxNode<NixLanguage>,
        decl_node: SyntaxNode<NixLanguage>,
        mortal: bool,
    ) -> Self {
        Binding {
            name,
            body_node,
            decl_node,
            mortal,
        }
    }

    pub fn is_mortal(&self) -> bool {
        self.mortal
    }

    /// Searches through tokens backwards for PRAGMA_SKIP until at
    /// least two linebreaks are seen
    pub fn has_pragma_skip(&self) -> bool {
        let mut line_breaks = 0;
        let mut token = self.decl_node.first_token().unwrap();
        while let Some(prev) = token.prev_token() {
            token = prev;

            match token.kind() {
                SyntaxKind::TOKEN_WHITESPACE => {
                    line_breaks += token.text().matches('\n').count();
                    if line_breaks > 1 {
                        break;
                    }
                }

                SyntaxKind::TOKEN_COMMENT if token.text().contains(PRAGMA_SKIP) =>
                    return true,

                _ => {}
            }
        }

        // No PRAGMA_SKIP found
        false
    }
}
