use ast::{IdentifierNode, ImportNode, ImportedIdentifier};

fn filter_identifiers<'a>(
    imported_identifiers: &[ImportedIdentifier<'a>],
) -> Vec<IdentifierNode<'a>> {
    imported_identifiers
        .iter()
        .filter_map(|imported_identifier| match imported_identifier {
            ImportedIdentifier::Identifier(identifier) => Some(identifier.clone()),
            ImportedIdentifier::TypeIdentifier(_) => None,
        })
        .collect()
}

fn format_path(path: &str) -> String {
    let mut result = String::new();
    result.push('"');
    result.push_str(path.replace(".buri", ".mjs").as_str());
    result.push('"');
    result
}

fn print_import(import: &ImportNode) -> String {
    let identifiers = filter_identifiers(&import.value.identifiers);
    if identifiers.is_empty() {
        return String::new();
    }
    let mut result = String::new();
    result.push_str("import {");
    for (index, identifier) in identifiers.iter().enumerate() {
        result.push_str(&identifier.value.name);
        if index < identifiers.len() - 1 {
            result.push(',');
        }
    }
    result.push_str("} from ");
    result.push_str(format_path(import.value.path).as_str());
    result
}

pub fn print_imports(imports: &[ImportNode]) -> String {
    let mut result = String::new();
    for import in imports {
        let import_statement = print_import(import);
        if import_statement.is_empty() {
            continue;
        }
        result.push_str(&import_statement);
        result.push('\n');
    }
    result
}

#[cfg(test)]
mod test {
    use super::*;
    use parser::parse_buri_file;

    #[test]
    fn prints_import_with_single_identifier() {
        let file = "import foo from \"foo.buri\"";
        let document = parse_buri_file(file).unwrap();
        assert_eq!(
            print_imports(&document.value.imports),
            "import {foo} from \"foo.mjs\"\n"
        );
    }

    #[test]
    fn prints_import_with_multiple_identifiers() {
        let file = "import foo, bar from \"foo.buri\"";
        let document = parse_buri_file(file).unwrap();
        assert_eq!(
            print_imports(&document.value.imports),
            "import {foo,bar} from \"foo.mjs\"\n"
        );
    }

    #[test]
    fn ignore_type_identifiers() {
        let file = "import foo, bar, Baz from \"foo.buri\"";
        let document = parse_buri_file(file).unwrap();
        assert_eq!(
            print_imports(&document.value.imports),
            "import {foo,bar} from \"foo.mjs\"\n"
        );
    }

    #[test]
    fn prints_import_with_multiple_imports() {
        let file = "import foo from \"foo.buri\"\nimport bar from \"bar.buri\"";
        let document = parse_buri_file(file).unwrap();
        assert_eq!(
            print_imports(&document.value.imports),
            "import {foo} from \"foo.mjs\"\nimport {bar} from \"bar.mjs\"\n"
        );
    }

    #[test]
    fn if_only_types_are_imported_delete_the_import_statement() {
        let file = "import Baz from \"foo.buri\"";
        let document = parse_buri_file(file).unwrap();
        assert_eq!(print_imports(&document.value.imports), "");
    }
}
