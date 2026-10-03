#![allow(clippy::redundant_closure_call)]
#![allow(clippy::needless_lifetimes)]
#![allow(clippy::match_single_binding)]
#![allow(clippy::clone_on_copy)]

#[doc = "A multiple sequence alignment: a list of Sequence records. Conventionally all sequences have the same length once gaps are included, but that is a semantic constraint, not checked structurally by this schema. Accepts either the bare-array shape every current producer (react-bio-viz's AlignedSequences, acacia's MSAData) actually emits, or a wrapped object carrying the wire-format 'type' discriminator for producers that want one."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Alignment {
    WrappedAlignment(WrappedAlignment),
    UnwrappedAlignment(UnwrappedAlignment),
}
impl ::std::convert::From<WrappedAlignment> for Alignment {
    fn from(value: WrappedAlignment) -> Self {
        Self::WrappedAlignment(value)
    }
}
impl ::std::convert::From<UnwrappedAlignment> for Alignment {
    fn from(value: UnwrappedAlignment) -> Self {
        Self::UnwrappedAlignment(value)
    }
}
#[doc = "A GFF3-derived sequence feature (gene model) node. Shared verbatim between picea (SequenceInterval) and react-bio-viz (SequenceInterval). seqid/start/end/strand stay flat rather than nesting under core/Location, to match the real current wire shape exactly."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Annotation {
    pub attributes: Metadata,
    #[doc = "Nested features, e.g. mRNA under gene, exon/CDS under mRNA."]
    pub children: ::std::vec::Vec<Annotation>,
    #[doc = "GFF3 column 5: 1-based inclusive end."]
    pub end: ::std::num::NonZeroU64,
    #[serde(rename = "ID")]
    pub id: Identifier,
    #[doc = "GFF3 column 3 (feature type), e.g. 'gene', 'mRNA', 'exon', 'CDS'."]
    pub interval_type: AnnotationIntervalType,
    #[doc = "GFF3 column 8 (CDS reading-frame phase). '.' is the GFF3 convention for 'not applicable'."]
    pub phase: AnnotationPhase,
    #[doc = "GFF3 column 6. '.' is the GFF3 convention for 'no score'."]
    pub score: AnnotationScore,
    #[doc = "GFF3 column 1: the reference sequence this feature is on."]
    pub seqid: Identifier,
    #[doc = "GFF3 column 2."]
    pub source: ::std::string::String,
    #[doc = "GFF3 column 4: 1-based inclusive start."]
    pub start: ::std::num::NonZeroU64,
    #[doc = "GFF3 column 7."]
    pub strand: ::betula_schema::Strand,
    #[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional. Distinct from 'interval_type': 'type' says this is a betula Annotation object, 'interval_type' says which GFF3 feature kind it is."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<AnnotationType>,
}
#[doc = "GFF3 column 3 (feature type), e.g. 'gene', 'mRNA', 'exon', 'CDS'."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct AnnotationIntervalType(::std::string::String);
impl ::std::ops::Deref for AnnotationIntervalType {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<AnnotationIntervalType> for ::std::string::String {
    fn from(value: AnnotationIntervalType) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for AnnotationIntervalType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for AnnotationIntervalType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnotationIntervalType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for AnnotationIntervalType {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "GFF3 column 8 (CDS reading-frame phase). '.' is the GFF3 convention for 'not applicable'."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnnotationPhase {
    Variant0(AnnotationPhaseVariant0),
    Variant1(AnnotationPhaseVariant1),
}
impl ::std::convert::From<AnnotationPhaseVariant0> for AnnotationPhase {
    fn from(value: AnnotationPhaseVariant0) -> Self {
        Self::Variant0(value)
    }
}
impl ::std::convert::From<AnnotationPhaseVariant1> for AnnotationPhase {
    fn from(value: AnnotationPhaseVariant1) -> Self {
        Self::Variant1(value)
    }
}
#[doc = "`AnnotationPhaseVariant0`"]
#[derive(:: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct AnnotationPhaseVariant0(f64);
impl ::std::ops::Deref for AnnotationPhaseVariant0 {
    type Target = f64;
    fn deref(&self) -> &f64 {
        &self.0
    }
}
impl ::std::convert::From<AnnotationPhaseVariant0> for f64 {
    fn from(value: AnnotationPhaseVariant0) -> Self {
        value.0
    }
}
impl ::std::convert::TryFrom<f64> for AnnotationPhaseVariant0 {
    type Error = self::error::ConversionError;
    fn try_from(value: f64) -> ::std::result::Result<Self, self::error::ConversionError> {
        if ![0_f64, 1_f64, 2_f64].contains(&value) {
            Err("invalid value".into())
        } else {
            Ok(Self(value))
        }
    }
}
impl<'de> ::serde::Deserialize<'de> for AnnotationPhaseVariant0 {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        Self::try_from(<f64>::deserialize(deserializer)?)
            .map_err(|e| <D::Error as ::serde::de::Error>::custom(e.to_string()))
    }
}
#[doc = "`AnnotationPhaseVariant1`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnotationPhaseVariant1 {
    #[serde(rename = ".")]
    X,
}
impl ::std::fmt::Display for AnnotationPhaseVariant1 {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str("."),
        }
    }
}
impl ::std::str::FromStr for AnnotationPhaseVariant1 {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "." => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnotationPhaseVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnotationPhaseVariant1 {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "GFF3 column 6. '.' is the GFF3 convention for 'no score'."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum AnnotationScore {
    Number(f64),
    String(AnnotationScoreString),
}
impl ::std::fmt::Display for AnnotationScore {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Number(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::str::FromStr for AnnotationScore {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if let Ok(v) = value.parse() {
            Ok(Self::Number(v))
        } else if let Ok(v) = value.parse() {
            Ok(Self::String(v))
        } else {
            Err("string conversion failed for all variants".into())
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnotationScore {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnotationScore {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::From<f64> for AnnotationScore {
    fn from(value: f64) -> Self {
        Self::Number(value)
    }
}
impl ::std::convert::From<AnnotationScoreString> for AnnotationScore {
    fn from(value: AnnotationScoreString) -> Self {
        Self::String(value)
    }
}
#[doc = "`AnnotationScoreString`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnotationScoreString {
    #[serde(rename = ".")]
    X,
}
impl ::std::fmt::Display for AnnotationScoreString {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::X => f.write_str("."),
        }
    }
}
impl ::std::str::FromStr for AnnotationScoreString {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "." => Ok(Self::X),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnotationScoreString {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnotationScoreString {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional. Distinct from 'interval_type': 'type' says this is a betula Annotation object, 'interval_type' says which GFF3 feature kind it is."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum AnnotationType {
    #[serde(rename = "annotation")]
    Annotation,
}
impl ::std::fmt::Display for AnnotationType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Annotation => f.write_str("annotation"),
        }
    }
}
impl ::std::str::FromStr for AnnotationType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "annotation" => Ok(Self::Annotation),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for AnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for AnnotationType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "All betula schemas bundled into one document for code generation. Generated by scripts/bundle_schema.py; do not edit."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Betula(pub ::serde_json::Value);
impl ::std::ops::Deref for Betula {
    type Target = ::serde_json::Value;
    fn deref(&self) -> &::serde_json::Value {
        &self.0
    }
}
impl ::std::convert::From<Betula> for ::serde_json::Value {
    fn from(value: Betula) -> Self {
        value.0
    }
}
impl ::std::convert::From<::serde_json::Value> for Betula {
    fn from(value: ::serde_json::Value) -> Self {
        Self(value)
    }
}
#[doc = "A full BLAST search result, matching blastserver's parsed-XML shape: hit-level, with nested HSPs and taxonomy/cluster enrichment. react-bio-viz's flattened single-row-per-hit visualization shape is a derived view of this, not a separate wire format. Numeric fields here are typed as numbers; blastserver's raw XML parse currently yields numeric strings and must cast before serializing against this schema."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct BlastResult {
    pub db: ::std::string::String,
    pub hits: ::std::vec::Vec<Hit>,
    pub message: ::std::string::String,
    pub program: ::std::string::String,
    #[serde(rename = "queryId")]
    pub query_id: Identifier,
    #[serde(rename = "queryLen")]
    pub query_len: ::std::num::NonZeroU64,
    #[serde(rename = "queryTitle")]
    pub query_title: ::std::string::String,
    pub stat: ::std::string::String,
    #[serde(
        rename = "taxonomyTrees",
        default,
        skip_serializing_if = "::std::vec::Vec::is_empty"
    )]
    pub taxonomy_trees: ::std::vec::Vec<TaxonomyNode>,
    #[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<BlastResultType>,
    pub version: ::std::string::String,
}
#[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum BlastResultType {
    #[serde(rename = "blast-result")]
    BlastResult,
}
impl ::std::fmt::Display for BlastResultType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::BlastResult => f.write_str("blast-result"),
        }
    }
}
impl ::std::str::FromStr for BlastResultType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "blast-result" => Ok(Self::BlastResult),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for BlastResultType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for BlastResultType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A pairwise distance matrix, shared by react-bio-viz and acacia (via @holmrenser/nj's DistanceResult). Squareness, symmetry, and a zero diagonal are semantic constraints, not checked structurally by this schema."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DistanceMatrix {
    #[doc = "Optional display-name lookup from label to a longer/human-readable name."]
    #[serde(
        rename = "labelNames",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub label_names: ::std::collections::HashMap<::std::string::String, ::std::string::String>,
    pub labels: ::std::vec::Vec<Identifier>,
    pub matrix: ::std::vec::Vec<::std::vec::Vec<f64>>,
    #[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<DistanceMatrixType>,
}
#[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DistanceMatrixType {
    #[serde(rename = "distance-matrix")]
    DistanceMatrix,
}
impl ::std::fmt::Display for DistanceMatrixType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DistanceMatrix => f.write_str("distance-matrix"),
        }
    }
}
impl ::std::str::FromStr for DistanceMatrixType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "distance-matrix" => Ok(Self::DistanceMatrix),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DistanceMatrixType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DistanceMatrixType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IUPAC nucleotide codes for DNA: A/C/G/T plus the standard ambiguity codes (R,Y,S,W,K,M,B,D,H,V,N). Also allows '-' and '.' as alignment gap/missing-data characters, since a Sequence doubles as an alignment row. Case-insensitive (lowercase is the common convention for soft-masked regions in genome assemblies)."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct DnaAlphabet(::std::string::String);
impl ::std::ops::Deref for DnaAlphabet {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<DnaAlphabet> for ::std::string::String {
    fn from(value: DnaAlphabet) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for DnaAlphabet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[ACGTRYSWKMBDHVNacgtryswkmbdhvn.-]*$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[ACGTRYSWKMBDHVNacgtryswkmbdhvn.-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for DnaAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnaAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for DnaAlphabet {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`DnaSequence`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct DnaSequence {
    pub identifier: Identifier,
    pub sequence: DnaAlphabet,
    #[serde(rename = "type")]
    pub type_: DnaSequenceType,
}
#[doc = "`DnaSequenceType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum DnaSequenceType {
    #[serde(rename = "dna-sequence")]
    DnaSequence,
}
impl ::std::fmt::Display for DnaSequenceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::DnaSequence => f.write_str("dna-sequence"),
        }
    }
}
impl ::std::str::FromStr for DnaSequenceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "dna-sequence" => Ok(Self::DnaSequence),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for DnaSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for DnaSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "`Hit`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Hit {
    pub accession: Identifier,
    #[doc = "Taxonomy lineage taxids."]
    pub ancestors: ::std::vec::Vec<i64>,
    #[serde(rename = "clusterSize")]
    pub cluster_size: ::std::num::NonZeroU64,
    pub hsps: ::std::vec::Vec<Hsp>,
    pub len: ::std::num::NonZeroU64,
    pub members: ::std::vec::Vec<HitMember>,
    #[doc = "Resolved taxon scientific name."]
    pub name: ::std::string::String,
    pub num: i64,
    #[serde(rename = "percentIdentity")]
    pub percent_identity: f64,
    #[serde(rename = "queryCover")]
    pub query_cover: f64,
    pub saccver: Identifier,
    pub taxid: i64,
    pub title: ::std::string::String,
}
#[doc = "One member of a clustered hit (e.g. in a clustered_nr database); a non-clustered hit has exactly one member."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct HitMember {
    pub accession: Identifier,
    pub name: ::std::string::String,
    pub taxid: i64,
    pub title: ::std::string::String,
}
#[doc = "`Hsp`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Hsp {
    #[serde(rename = "alignLen")]
    pub align_len: ::std::num::NonZeroU64,
    #[serde(rename = "bitScore")]
    pub bit_score: f64,
    pub evalue: f64,
    #[serde(rename = "hitFrom")]
    pub hit_from: ::std::num::NonZeroU64,
    #[serde(rename = "hitTo")]
    pub hit_to: ::std::num::NonZeroU64,
    pub hseq: ::std::string::String,
    pub identity: u64,
    pub midline: ::std::string::String,
    pub num: ::std::num::NonZeroU64,
    pub qseq: ::std::string::String,
    #[serde(rename = "queryFrom")]
    pub query_from: ::std::num::NonZeroU64,
    #[serde(rename = "queryTo")]
    pub query_to: ::std::num::NonZeroU64,
    pub score: f64,
}
#[doc = "A short, stable identifier/label string: a sequence identifier, a GFF3 feature ID, a BLAST accession, etc."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct Identifier(::std::string::String);
impl ::std::ops::Deref for Identifier {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<Identifier> for ::std::string::String {
    fn from(value: Identifier) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for Identifier {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for Identifier {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for Identifier {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for Identifier {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "A 1-based inclusive interval on a named reference sequence (GFF3 convention). Not yet consumed by another betula schema: annotation keeps seqid/start/end/strand flat to match picea's and react-bio-viz's current wire shape rather than nesting them under this. This exists as the reusable primitive for when a genomic-interval-only shape is needed, e.g. a future schema for react-bio-viz's GenomeBrowser feature tracks."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Location {
    pub end: ::std::num::NonZeroU64,
    pub seqid: Identifier,
    pub start: ::std::num::NonZeroU64,
    pub strand: ::betula_schema::Strand,
}
#[doc = "Free-form key/value metadata, GFF3-attribute-style: each value is a single string, or a list of strings for multi-valued tags."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct Metadata(pub ::std::collections::HashMap<::std::string::String, MetadataValue>);
impl ::std::ops::Deref for Metadata {
    type Target = ::std::collections::HashMap<::std::string::String, MetadataValue>;
    fn deref(&self) -> &::std::collections::HashMap<::std::string::String, MetadataValue> {
        &self.0
    }
}
impl ::std::convert::From<Metadata>
    for ::std::collections::HashMap<::std::string::String, MetadataValue>
{
    fn from(value: Metadata) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::collections::HashMap<::std::string::String, MetadataValue>>
    for Metadata
{
    fn from(value: ::std::collections::HashMap<::std::string::String, MetadataValue>) -> Self {
        Self(value)
    }
}
#[doc = "`MetadataValue`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MetadataValue {
    String(::std::string::String),
    Array(::std::vec::Vec<::std::string::String>),
}
impl ::std::convert::From<::std::vec::Vec<::std::string::String>> for MetadataValue {
    fn from(value: ::std::vec::Vec<::std::string::String>) -> Self {
        Self::Array(value)
    }
}
#[doc = "IUPAC amino acid codes: the standard 20 (A,C,D,E,F,G,H,I,K,L,M,N,P,Q,R,S,T,V,W,Y) plus the ambiguity/special codes B (Asx), Z (Glx), X (any), J (Leu/Ile), U (selenocysteine), O (pyrrolysine) -- together these cover all 26 letters, so this pattern is effectively any letter. Also allows '*' for a stop codon, and '-'/'.' as alignment gap/missing-data characters, since a Sequence doubles as an alignment row. Case-insensitive."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct ProteinAlphabet(::std::string::String);
impl ::std::ops::Deref for ProteinAlphabet {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<ProteinAlphabet> for ::std::string::String {
    fn from(value: ProteinAlphabet) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for ProteinAlphabet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| ::regress::Regex::new("^[A-Za-z*.-]*$").unwrap());
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[A-Za-z*.-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for ProteinAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProteinAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for ProteinAlphabet {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`ProteinSequence`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ProteinSequence {
    pub identifier: Identifier,
    pub sequence: ProteinAlphabet,
    #[serde(rename = "type")]
    pub type_: ProteinSequenceType,
}
#[doc = "`ProteinSequenceType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum ProteinSequenceType {
    #[serde(rename = "protein-sequence")]
    ProteinSequence,
}
impl ::std::fmt::Display for ProteinSequenceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::ProteinSequence => f.write_str("protein-sequence"),
        }
    }
}
impl ::std::str::FromStr for ProteinSequenceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "protein-sequence" => Ok(Self::ProteinSequence),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for ProteinSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for ProteinSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "IUPAC nucleotide codes for RNA: A/C/G/U plus the standard ambiguity codes (R,Y,S,W,K,M,B,D,H,V,N). 'T' is deliberately excluded (that's DnaAlphabet). Also allows '-' and '.' as alignment gap/missing-data characters, since a Sequence doubles as an alignment row. Case-insensitive."]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct RnaAlphabet(::std::string::String);
impl ::std::ops::Deref for RnaAlphabet {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<RnaAlphabet> for ::std::string::String {
    fn from(value: RnaAlphabet) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for RnaAlphabet {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        static PATTERN: ::std::sync::LazyLock<::regress::Regex> =
            ::std::sync::LazyLock::new(|| {
                ::regress::Regex::new("^[ACGURYSWKMBDHVNacguryswkmbdhvn.-]*$").unwrap()
            });
        if PATTERN.find(value).is_none() {
            return Err("doesn't match pattern \"^[ACGURYSWKMBDHVNacguryswkmbdhvn.-]*$\"".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for RnaAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RnaAlphabet {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for RnaAlphabet {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "`RnaSequence`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct RnaSequence {
    pub identifier: Identifier,
    pub sequence: RnaAlphabet,
    #[serde(rename = "type")]
    pub type_: RnaSequenceType,
}
#[doc = "`RnaSequenceType`"]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum RnaSequenceType {
    #[serde(rename = "rna-sequence")]
    RnaSequence,
}
impl ::std::fmt::Display for RnaSequenceType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::RnaSequence => f.write_str("rna-sequence"),
        }
    }
}
impl ::std::str::FromStr for RnaSequenceType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "rna-sequence" => Ok(Self::RnaSequence),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for RnaSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for RnaSequenceType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "A single biological sequence record, shared by picea, react-bio-viz, and acacia (FASTA-derived). 'type' is an optional alphabet discriminator: every current producer omits it and emits the plain UntypedSequence shape, which remains valid. Set it when the alphabet is known so consumers get both a proper discriminated union and IUPAC alphabet validation on 'sequence' (core/dna-alphabet, core/rna-alphabet, core/protein-alphabet) instead of an unconstrained string."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Sequence {
    DnaSequence(DnaSequence),
    RnaSequence(RnaSequence),
    ProteinSequence(ProteinSequence),
    UntypedSequence(UntypedSequence),
}
impl ::std::convert::From<DnaSequence> for Sequence {
    fn from(value: DnaSequence) -> Self {
        Self::DnaSequence(value)
    }
}
impl ::std::convert::From<RnaSequence> for Sequence {
    fn from(value: RnaSequence) -> Self {
        Self::RnaSequence(value)
    }
}
impl ::std::convert::From<ProteinSequence> for Sequence {
    fn from(value: ProteinSequence) -> Self {
        Self::ProteinSequence(value)
    }
}
impl ::std::convert::From<UntypedSequence> for Sequence {
    fn from(value: UntypedSequence) -> Self {
        Self::UntypedSequence(value)
    }
}
#[doc = "A pointer to a Sequence by identifier, without embedding the sequence itself. Not yet consumed by another betula schema (every current producer embeds full Sequence objects, e.g. in alignment); this exists as the reusable primitive for when a by-reference link is needed, e.g. a hit record pointing at a locally-known query sequence instead of repeating it."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct SequenceReference {
    pub identifier: Identifier,
}
#[doc = "`TaxonomyNode`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TaxonomyNode {
    pub ancestors: ::std::vec::Vec<i64>,
    #[serde(default, skip_serializing_if = "::std::vec::Vec::is_empty")]
    pub children: ::std::vec::Vec<TaxonomyNode>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub count: ::std::option::Option<u64>,
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub depth: ::std::option::Option<u64>,
    pub id: i64,
    pub name: ::std::string::String,
}
#[doc = "A phylogenetic (or other hierarchical) tree node, shared by picea, react-bio-viz, acacia, and iqtreeserver. The root of a tree is itself a Tree node; a whole tree and a single node use the same shape, so 'type' is optional and conventionally set only on a node transmitted as a standalone document (typically the root), not repeated on every nested child."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Tree {
    #[doc = "Child nodes. Empty array for a leaf."]
    pub children: ::std::vec::Vec<Tree>,
    #[doc = "Optional node identifier. Not every producer assigns one at parse time (acacia/iqtreeserver derive ids separately); consumers must not assume it is present."]
    #[serde(skip_serializing_if = "::std::option::Option::is_none")]
    pub id: ::std::option::Option<TreeId>,
    #[doc = "Branch length leading to this node."]
    pub length: f64,
    #[doc = "Raw node label: a leaf name, or an internal-node label such as a bootstrap value or support string (e.g. picea's \"0.989\", IQ-TREE's \"95.3/88\"). Kept as the original string for round-tripping; parse further only if you know the producer's convention."]
    pub name: ::std::string::String,
    #[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional; omit on nested children."]
    #[serde(
        rename = "type",
        skip_serializing_if = "::std::option::Option::is_none"
    )]
    pub type_: ::std::option::Option<TreeType>,
}
#[doc = "Optional node identifier. Not every producer assigns one at parse time (acacia/iqtreeserver derive ids separately); consumers must not assume it is present."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum TreeId {
    Integer(i64),
    String(::std::string::String),
}
impl ::std::fmt::Display for TreeId {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match self {
            Self::Integer(x) => x.fmt(f),
            Self::String(x) => x.fmt(f),
        }
    }
}
impl ::std::convert::From<i64> for TreeId {
    fn from(value: i64) -> Self {
        Self::Integer(value)
    }
}
#[doc = "Wire-format object-kind discriminator (see betula's versioning policy). Optional; omit on nested children."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum TreeType {
    #[serde(rename = "tree")]
    Tree,
}
impl ::std::fmt::Display for TreeType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Tree => f.write_str("tree"),
        }
    }
}
impl ::std::str::FromStr for TreeType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "tree" => Ok(Self::Tree),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for TreeType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for TreeType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = "The shape every current producer (picea, react-bio-viz, acacia) actually emits: no alphabet discriminator."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct UntypedSequence {
    pub identifier: Identifier,
    pub sequence: UntypedSequenceSequence,
}
#[doc = "`UntypedSequenceSequence`"]
#[derive(:: serde :: Serialize, Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
#[serde(transparent)]
pub struct UntypedSequenceSequence(::std::string::String);
impl ::std::ops::Deref for UntypedSequenceSequence {
    type Target = ::std::string::String;
    fn deref(&self) -> &::std::string::String {
        &self.0
    }
}
impl ::std::convert::From<UntypedSequenceSequence> for ::std::string::String {
    fn from(value: UntypedSequenceSequence) -> Self {
        value.0
    }
}
impl ::std::str::FromStr for UntypedSequenceSequence {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        if value.chars().count() < 1usize {
            return Err("shorter than 1 characters".into());
        }
        Ok(Self(value.to_string()))
    }
}
impl ::std::convert::TryFrom<&str> for UntypedSequenceSequence {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for UntypedSequenceSequence {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl<'de> ::serde::Deserialize<'de> for UntypedSequenceSequence {
    fn deserialize<D>(deserializer: D) -> ::std::result::Result<Self, D::Error>
    where
        D: ::serde::Deserializer<'de>,
    {
        ::std::string::String::deserialize(deserializer)?
            .parse()
            .map_err(|e: self::error::ConversionError| {
                <D::Error as ::serde::de::Error>::custom(e.to_string())
            })
    }
}
#[doc = "The shape every current producer (react-bio-viz, acacia) actually emits: no wrapper, no discriminator."]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(transparent)]
pub struct UnwrappedAlignment(pub ::std::vec::Vec<Sequence>);
impl ::std::ops::Deref for UnwrappedAlignment {
    type Target = ::std::vec::Vec<Sequence>;
    fn deref(&self) -> &::std::vec::Vec<Sequence> {
        &self.0
    }
}
impl ::std::convert::From<UnwrappedAlignment> for ::std::vec::Vec<Sequence> {
    fn from(value: UnwrappedAlignment) -> Self {
        value.0
    }
}
impl ::std::convert::From<::std::vec::Vec<Sequence>> for UnwrappedAlignment {
    fn from(value: ::std::vec::Vec<Sequence>) -> Self {
        Self(value)
    }
}
#[doc = "`WrappedAlignment`"]
#[derive(:: serde :: Deserialize, :: serde :: Serialize, Clone, Debug, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct WrappedAlignment {
    pub sequences: ::std::vec::Vec<Sequence>,
    #[doc = "Wire-format object-kind discriminator (see betula's versioning policy)."]
    #[serde(rename = "type")]
    pub type_: WrappedAlignmentType,
}
#[doc = "Wire-format object-kind discriminator (see betula's versioning policy)."]
#[derive(
    :: serde :: Deserialize,
    :: serde :: Serialize,
    Clone,
    Copy,
    Debug,
    Eq,
    Hash,
    Ord,
    PartialEq,
    PartialOrd,
)]
pub enum WrappedAlignmentType {
    #[serde(rename = "alignment")]
    Alignment,
}
impl ::std::fmt::Display for WrappedAlignmentType {
    fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
        match *self {
            Self::Alignment => f.write_str("alignment"),
        }
    }
}
impl ::std::str::FromStr for WrappedAlignmentType {
    type Err = self::error::ConversionError;
    fn from_str(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        match value {
            "alignment" => Ok(Self::Alignment),
            _ => Err("invalid value".into()),
        }
    }
}
impl ::std::convert::TryFrom<&str> for WrappedAlignmentType {
    type Error = self::error::ConversionError;
    fn try_from(value: &str) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
impl ::std::convert::TryFrom<::std::string::String> for WrappedAlignmentType {
    type Error = self::error::ConversionError;
    fn try_from(
        value: ::std::string::String,
    ) -> ::std::result::Result<Self, self::error::ConversionError> {
        value.parse()
    }
}
#[doc = " Error types."]
pub mod error {
    #[doc = r" Error from a `TryFrom` or `FromStr` implementation."]
    pub struct ConversionError(::std::borrow::Cow<'static, str>);
    impl ::std::error::Error for ConversionError {}
    impl ::std::fmt::Display for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Display::fmt(&self.0, f)
        }
    }
    impl ::std::fmt::Debug for ConversionError {
        fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> Result<(), ::std::fmt::Error> {
            ::std::fmt::Debug::fmt(&self.0, f)
        }
    }
    impl From<&'static str> for ConversionError {
        fn from(value: &'static str) -> Self {
            Self(value.into())
        }
    }
    impl From<String> for ConversionError {
        fn from(value: String) -> Self {
            Self(value.into())
        }
    }
}
