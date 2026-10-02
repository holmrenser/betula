//! Parsing betula JSON with the Rust bindings. Run in CI; the asserts are checked.

use betula::{Annotation, Sequence, Strand, Tree};
use serde_json::json;

fn main() -> Result<(), betula::Error> {
    // Parse JSON text. The type parameter picks both the schema and the result.
    let tree: Tree = betula::parse_str(
        r#"{"name": "root", "length": 0, "children": [{"name": "A", "length": 0.1, "children": []}]}"#,
    )?;
    assert_eq!(tree.children[0].name, "A");

    // Or validate a serde_json::Value you've already decoded.
    let seq: Sequence = betula::parse(json!({"type": "rna-sequence", "identifier": "s1", "sequence": "ACGU"}))?;

    // Union kinds are enums. Constrained strings (Identifier, the alphabets)
    // are newtypes that Deref to String.
    match &seq {
        Sequence::RnaSequence(rna) => assert_eq!((rna.identifier.as_str(), rna.sequence.as_str()), ("s1", "ACGU")),
        other => panic!("unexpected variant {other:?}"),
    }

    // GFF3 strand is a plain enum.
    let gene: Annotation = betula::parse(json!({
        "ID": "gene1", "seqid": "chr1", "source": "example", "interval_type": "gene",
        "start": 1, "end": 900, "score": ".", "strand": "-", "phase": ".",
        "attributes": {}, "children": []
    }))?;
    assert_eq!(gene.strand, Strand::Reverse);

    // Invalid data is Error::Invalid, with one message per problem. parse
    // checks the schema before deserializing, which matters: the generated
    // types alone don't enforce every keyword.
    match betula::parse_str::<Sequence>(r#"{"type": "dna-sequence", "identifier": "s2", "sequence": "ACGU"}"#) {
        Err(betula::Error::Invalid { kind, errors }) => assert!(kind == "Sequence" && !errors.is_empty()),
        other => panic!("expected Error::Invalid, got {other:?}"),
    }

    // validate() checks a Value without deserializing it.
    assert!(betula::validate::<Tree>(&json!({"name": "A", "length": -1, "children": []})).is_err());

    // Serialize with serde as usual.
    let text = serde_json::to_string(&tree).map_err(betula::Error::Json)?;
    assert_eq!(betula::parse_str::<Tree>(&text)?, tree);

    // Every generated type implements Kind, so generic code can take any of
    // them; KINDS lists the names.
    fn count_valid<K: betula::Kind>(values: &[serde_json::Value]) -> usize {
        values.iter().filter(|v| betula::validate::<K>(v).is_ok()).count()
    }
    let candidates = [json!({"name": "A", "length": 1, "children": []}), json!({"name": "B"})];
    assert_eq!(count_valid::<Tree>(&candidates), 1);
    assert!(betula::KINDS.contains(&<Tree as betula::Kind>::NAME));

    println!("rust usage example ok");
    Ok(())
}
