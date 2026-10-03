//! Parsing betula JSON with the Rust bindings. Run in CI; the asserts are checked.

use betula_schema::{Annotation, Sequence, Strand, Tree};
use serde_json::json;

fn main() -> Result<(), betula_schema::Error> {
    // Parse JSON text. The type parameter picks both the schema and the result.
    let tree: Tree = betula_schema::parse_str(
        r#"{"name": "root", "length": 0, "children": [{"name": "A", "length": 0.1, "children": []}]}"#,
    )?;
    assert_eq!(tree.children[0].name, "A");

    // Or validate a serde_json::Value you've already decoded.
    let seq: Sequence = betula_schema::parse(json!({"type": "rna-sequence", "identifier": "s1", "sequence": "ACGU"}))?;

    // Union kinds are enums. Constrained strings (Identifier, the alphabets)
    // are newtypes that Deref to String.
    match &seq {
        Sequence::RnaSequence(rna) => assert_eq!((rna.identifier.as_str(), rna.sequence.as_str()), ("s1", "ACGU")),
        other => panic!("unexpected variant {other:?}"),
    }

    // GFF3 strand is a plain enum.
    let gene: Annotation = betula_schema::parse(json!({
        "ID": "gene1", "seqid": "chr1", "source": "example", "interval_type": "gene",
        "start": 1, "end": 900, "score": ".", "strand": "-", "phase": ".",
        "attributes": {}, "children": []
    }))?;
    assert_eq!(gene.strand, Strand::Reverse);

    // Invalid data is Error::Invalid, with one message per problem. parse
    // checks the schema before deserializing, which matters: the generated
    // types alone don't enforce every keyword.
    match betula_schema::parse_str::<Sequence>(r#"{"type": "dna-sequence", "identifier": "s2", "sequence": "ACGU"}"#) {
        Err(betula_schema::Error::Invalid { kind, errors }) => assert!(kind == "Sequence" && !errors.is_empty()),
        other => panic!("expected Error::Invalid, got {other:?}"),
    }

    // validate() checks a Value without deserializing it.
    assert!(betula_schema::validate::<Tree>(&json!({"name": "A", "length": -1, "children": []})).is_err());

    // Serialize with serde as usual.
    let text = serde_json::to_string(&tree).map_err(betula_schema::Error::Json)?;
    assert_eq!(betula_schema::parse_str::<Tree>(&text)?, tree);

    // Every generated type implements Kind, so generic code can take any of
    // them; KINDS lists the names.
    fn count_valid<K: betula_schema::Kind>(values: &[serde_json::Value]) -> usize {
        values.iter().filter(|v| betula_schema::validate::<K>(v).is_ok()).count()
    }
    let candidates = [json!({"name": "A", "length": 1, "children": []}), json!({"name": "B"})];
    assert_eq!(count_valid::<Tree>(&candidates), 1);
    assert!(betula_schema::KINDS.contains(&<Tree as betula_schema::Kind>::NAME));

    println!("rust usage example ok");
    Ok(())
}
