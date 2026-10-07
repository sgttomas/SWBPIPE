//! Borrowed facts and the admission law for the retained route (U4; D-5).
//!
//! G-A (`admit`) runs the census, the D1 predicate (D1.0–D1.11), the D-6 build
//! status and the cap-priced admission bound against M. G-B and G-C
//! (`check_late`, `check_complete`) cross-check the live owners. Nothing here
//! allocates. One production profile is registered (`REGISTERED_PROFILES`: the
//! dev/test identity, M = 4,026,531,840 B under D-7; G6/G7). Every other build is
//! Stale and keeps the ordinary route; a refusal keeps every fact and the first failing clause.
use crate::{source_receipt::CapturedInvocation, LinearStaticPreviewRequest};
use serde_json::Value;

// D-6: the one identity encoder, shared with build.rs (G2_AMENDMENTS.md §1).
#[path = "build_identity.rs"]
mod build_identity;

/// Local census work limits, not restrictions on the ordinary producer.
const DEPTH_LIMIT: usize = 64;
const VALUE_LIMIT: usize = 16_384;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CensusStatus {
    Complete,
    DepthLimit,
    ValueLimit,
    ArithmeticOverflow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapacityFact {
    pub length: usize,
    pub capacity: usize,
}
impl CapacityFact {
    fn vector<T>(value: &Vec<T>) -> Self {
        Self {
            length: value.len(),
            capacity: value.capacity(),
        }
    }
    fn string(value: &String) -> Self {
        Self {
            length: value.len(),
            capacity: value.capacity(),
        }
    }
}

/// Actual entered prefix. Capacity units are elements for arrays and bytes for
/// strings. Object node/backing capacity is intentionally not inferred from len.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedValueFacts {
    pub status: CensusStatus,
    pub values: usize,
    pub objects: usize,
    pub object_entries: usize,
    pub arrays: usize,
    pub array_elements: usize,
    pub array_capacity_elements: usize,
    pub strings: usize,
    pub string_bytes: usize,
    pub string_capacity_bytes: usize,
    pub key_bytes: usize,
    pub key_capacity_bytes: usize,
    pub maximum_depth: usize,
}
impl BorrowedValueFacts {
    fn empty() -> Self {
        Self {
            status: CensusStatus::Complete,
            values: 0,
            objects: 0,
            object_entries: 0,
            arrays: 0,
            array_elements: 0,
            array_capacity_elements: 0,
            strings: 0,
            string_bytes: 0,
            string_capacity_bytes: 0,
            key_bytes: 0,
            key_capacity_bytes: 0,
            maximum_depth: 0,
        }
    }
}

enum Frame<'a> {
    Array(std::slice::Iter<'a, Value>),
    Object(serde_json::map::Iter<'a>),
}
impl<'a> Frame<'a> {
    fn next(&mut self) -> Option<(Option<&'a String>, &'a Value)> {
        match self {
            Self::Array(values) => values.next().map(|v| (None, v)),
            Self::Object(values) => values.next().map(|(k, v)| (Some(k), v)),
        }
    }
}
fn add(sum: &mut usize, value: usize) -> Result<(), CensusStatus> {
    *sum = sum
        .checked_add(value)
        .ok_or(CensusStatus::ArithmeticOverflow)?;
    Ok(())
}

/// No clone, parser, encoding, recursion or heap workspace. The fixed iterator
/// array bounds local traversal storage; its build/stack profile is still an
/// explicit missing admission premise. A partial result never means complete.
pub fn borrowed_value_census(root: &Value) -> BorrowedValueFacts {
    let mut facts = BorrowedValueFacts::empty();
    let mut stack: [Option<Frame<'_>>; DEPTH_LIMIT] = [const { None }; DEPTH_LIMIT];
    let mut depth = 0;
    let mut pending = Some((None::<&String>, root));
    loop {
        if let Some((key, value)) = pending.take() {
            if facts.values == VALUE_LIMIT {
                facts.status = CensusStatus::ValueLimit;
                break;
            }
            let entered = (|| {
                add(&mut facts.values, 1)?;
                facts.maximum_depth = facts.maximum_depth.max(depth);
                if let Some(key) = key {
                    add(&mut facts.key_bytes, key.len())?;
                    add(&mut facts.key_capacity_bytes, key.capacity())?;
                }
                match value {
                    Value::String(s) => {
                        add(&mut facts.strings, 1)?;
                        add(&mut facts.string_bytes, s.len())?;
                        add(&mut facts.string_capacity_bytes, s.capacity())?;
                    }
                    Value::Array(a) => {
                        add(&mut facts.arrays, 1)?;
                        add(&mut facts.array_elements, a.len())?;
                        add(&mut facts.array_capacity_elements, a.capacity())?;
                    }
                    Value::Object(o) => {
                        add(&mut facts.objects, 1)?;
                        add(&mut facts.object_entries, o.len())?;
                    }
                    _ => {}
                }
                Ok::<_, CensusStatus>(())
            })();
            if let Err(status) = entered {
                facts.status = status;
                break;
            }
            let children = match value {
                Value::Array(a) if !a.is_empty() => Some(Frame::Array(a.iter())),
                Value::Object(o) if !o.is_empty() => Some(Frame::Object(o.iter())),
                _ => None,
            };
            if let Some(children) = children {
                if depth == DEPTH_LIMIT {
                    facts.status = CensusStatus::DepthLimit;
                    break;
                }
                stack[depth] = Some(children);
                depth += 1;
            }
        }
        while depth != 0 {
            pending = stack[depth - 1].as_mut().and_then(Frame::next);
            if pending.is_some() {
                break;
            }
            stack[depth - 1] = None;
            depth -= 1;
        }
        if pending.is_none() {
            break;
        }
    }
    facts
}

/// Top-level typed owners only. Nested strings/vectors and ordinary working
/// owners are not hidden in a claimed total; see required_unknown_terms().
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BorrowedRequestFacts {
    pub nodes: CapacityFact,
    pub members: CapacityFact,
    pub sections: CapacityFact,
    pub supports: CapacityFact,
    pub components: CapacityFact,
    pub model_materials: CapacityFact,
    pub request_materials: CapacityFact,
    pub load_cases: CapacityFact,
    pub combinations: CapacityFact,
    pub material_expansion_laws: CapacityFact,
    pub request_expansion_law_indices: CapacityFact,
    pub dof_upper: Result<usize, CensusStatus>,
}
pub fn borrowed_request_census(request: &LinearStaticPreviewRequest) -> BorrowedRequestFacts {
    let m = &request.model;
    BorrowedRequestFacts {
        nodes: CapacityFact::vector(&m.nodes),
        members: CapacityFact::vector(&m.pipe_segments),
        sections: CapacityFact::vector(&m.sections),
        supports: CapacityFact::vector(&m.supports),
        components: CapacityFact::vector(&m.components),
        model_materials: CapacityFact::vector(&m.materials),
        request_materials: CapacityFact::vector(&request.materials),
        load_cases: CapacityFact::vector(&m.load_cases),
        combinations: CapacityFact::vector(&m.combinations),
        material_expansion_laws: CapacityFact::vector(&m.material_expansion_laws),
        request_expansion_law_indices: CapacityFact::vector(&m.request_material_expansion_laws),
        dof_upper: checked_dof(m.nodes.len()),
    }
}
fn checked_dof(nodes: usize) -> Result<usize, CensusStatus> {
    nodes.checked_mul(6).ok_or(CensusStatus::ArithmeticOverflow)
}

/// D1.9's maximum lengths and D1.11's control bytes over a borrowed raw Value
/// (G5; DOMAIN.md §2, G2_AMENDMENTS.md §6). Same cursor and limits as
/// `borrowed_value_census`; no clone, parser, recursion or heap workspace.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct RawTextFacts {
    pub(super) status: CensusStatus,
    pub(super) max_string_bytes: usize,
    pub(super) max_key_bytes: usize,
    /// Bytes below 0x20 or equal to 0x7F, over every string value and key.
    pub(super) control_bytes: usize,
}
fn control_bytes_in(text: &str) -> usize {
    text.bytes().filter(|b| *b < 0x20 || *b == 0x7F).count()
}
pub(super) fn raw_text_census(root: &Value) -> RawTextFacts {
    let mut facts = RawTextFacts {
        status: CensusStatus::Complete,
        max_string_bytes: 0,
        max_key_bytes: 0,
        control_bytes: 0,
    };
    let mut stack: [Option<Frame<'_>>; DEPTH_LIMIT] = [const { None }; DEPTH_LIMIT];
    let mut depth = 0;
    let mut values = 0;
    let mut pending = Some((None::<&String>, root));
    loop {
        if let Some((key, value)) = pending.take() {
            if values == VALUE_LIMIT {
                facts.status = CensusStatus::ValueLimit;
                break;
            }
            values += 1;
            let entered = (|| {
                if let Some(key) = key {
                    facts.max_key_bytes = facts.max_key_bytes.max(key.len());
                    add(&mut facts.control_bytes, control_bytes_in(key))?;
                }
                if let Value::String(s) = value {
                    facts.max_string_bytes = facts.max_string_bytes.max(s.len());
                    add(&mut facts.control_bytes, control_bytes_in(s))?;
                }
                Ok::<_, CensusStatus>(())
            })();
            if let Err(status) = entered {
                facts.status = status;
                break;
            }
            let children = match value {
                Value::Array(a) if !a.is_empty() => Some(Frame::Array(a.iter())),
                Value::Object(o) if !o.is_empty() => Some(Frame::Object(o.iter())),
                _ => None,
            };
            if let Some(children) = children {
                if depth == DEPTH_LIMIT {
                    facts.status = CensusStatus::DepthLimit;
                    break;
                }
                stack[depth] = Some(children);
                depth += 1;
            }
        }
        while depth != 0 {
            pending = stack[depth - 1].as_mut().and_then(Frame::next);
            if pending.is_some() {
                break;
            }
            stack[depth - 1] = None;
            depth -= 1;
        }
        if pending.is_none() {
            break;
        }
    }
    facts
}

/// RESIDUALS.md T03: the nested typed owners of a D1 request, read from the
/// borrowed typed request (actual lengths and capacities, never construction
/// history). Owners of excluded families (hangers, nonlinear supports, pressure,
/// sections, components, combinations, generated and load-state inputs) are not
/// read: their presence is refused by D1.3–D1.6 instead. Allocation-free.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct NestedTypedFacts {
    pub(super) status: CensusStatus,
    /// Every typed String read: count, maximum length and capacity, total capacity.
    pub(super) strings: usize,
    pub(super) max_string_bytes: usize,
    pub(super) max_string_capacity: usize,
    pub(super) string_capacity_bytes: usize,
    /// r: Σ `supports[i].restraints.len()`; their capacities, summed and maximum.
    pub(super) restraints: usize,
    pub(super) restraint_capacity: usize,
    pub(super) max_restraint_capacity: usize,
    /// s: supports with a scalar spring (`stiffness` present).
    pub(super) springs: usize,
    /// The largest `temperature_points` length and capacity over both material lists.
    pub(super) max_temperature_points: CapacityFact,
    /// l (B1 SA): the largest `primitive_loads` length and the largest capacity over every
    /// load case, so that each case is bounded by `l`.
    pub(super) primitive_loads: CapacityFact,
    /// Σ l_i (B1 SA): the primitive loads over every load case. A `u32` (an in-domain total
    /// is at most L = 384; a total that does not fit is D1.2's `ArithmeticOverflow`) so that
    /// it sits in `status`'s padding: this struct, inside the report that the profile's
    /// `s(ThreadPacketOutput)` atom prices, keeps its size until SQ re-derives the profile.
    pub(super) total_loads: u32,
    /// The typed `project.units` Value: a separate owner, within the raw caps.
    pub(super) units: BorrowedValueFacts,
    pub(super) units_text: RawTextFacts,
}
struct TypedWalk {
    facts: NestedTypedFacts,
}
impl TypedWalk {
    fn string(&mut self, s: &String) -> Result<(), CensusStatus> {
        let f = &mut self.facts;
        add(&mut f.strings, 1)?;
        f.max_string_bytes = f.max_string_bytes.max(s.len());
        f.max_string_capacity = f.max_string_capacity.max(s.capacity());
        add(&mut f.string_capacity_bytes, s.capacity())
    }
    fn optional(&mut self, s: &Option<String>) -> Result<(), CensusStatus> {
        s.as_ref().map_or(Ok(()), |s| self.string(s))
    }
    fn quantity(&mut self, q: &crate::Quantity) -> Result<(), CensusStatus> {
        self.string(&q.unit)
    }
    fn optional_quantity(&mut self, q: &Option<crate::Quantity>) -> Result<(), CensusStatus> {
        q.as_ref().map_or(Ok(()), |q| self.quantity(q))
    }
    fn material(&mut self, m: &crate::MaterialInput) -> Result<(), CensusStatus> {
        self.string(&m.id)?;
        self.optional(&m.constitutive_basis)?;
        self.optional(&m.provenance)?;
        self.quantity(&m.elastic_modulus)?;
        for q in [&m.poisson_ratio, &m.shear_modulus, &m.thermal_expansion_coefficient] {
            self.optional_quantity(q)?;
        }
        let points = &mut self.facts.max_temperature_points;
        points.length = points.length.max(m.temperature_points.len());
        points.capacity = points.capacity.max(m.temperature_points.capacity());
        for p in &m.temperature_points {
            self.string(&p.id)?;
            self.optional(&p.provenance)?;
            for q in [&p.poisson_ratio, &p.temperature, &p.elastic_modulus, &p.shear_modulus, &p.thermal_expansion_coefficient] {
                self.optional_quantity(q)?;
            }
        }
        Ok(())
    }
    fn walk(&mut self, request: &LinearStaticPreviewRequest) -> Result<(), CensusStatus> {
        let m = &request.model;
        for material in request.materials.iter().chain(&m.materials) {
            self.material(material)?;
        }
        self.string(&m.schema_version)?;
        self.string(&m.document_kind)?;
        self.string(&m.project.id)?;
        let s = &m.analysis_status;
        for text in [&s.mechanics, &s.rule_check, &s.professional_acceptance] {
            self.string(text)?;
        }
        for node in &m.nodes {
            self.string(&node.id)?;
            self.optional(&node.provenance)?;
        }
        for pipe in &m.pipe_segments {
            for text in [&pipe.id, &pipe.from, &pipe.to, &pipe.material] {
                self.string(text)?;
            }
            self.optional(&pipe.section_ref)?;
            self.optional(&pipe.provenance)?;
            let section = &pipe.section;
            self.quantity(&section.outside_diameter)?;
            self.quantity(&section.wall_thickness)?;
            for q in [&section.mill_tolerance, &section.material_density, &section.contents_density, &section.insulation_thickness, &section.insulation_density] {
                self.optional_quantity(q)?;
            }
        }
        for support in &m.supports {
            self.string(&support.id)?;
            self.string(&support.node)?;
            self.optional(&support.family)?;
            self.optional(&support.provenance)?;
            let f = &mut self.facts;
            add(&mut f.restraints, support.restraints.len())?;
            add(&mut f.restraint_capacity, support.restraints.capacity())?;
            f.max_restraint_capacity = f.max_restraint_capacity.max(support.restraints.capacity());
            for restraint in &support.restraints {
                self.string(restraint)?;
            }
            if let Some(stiffness) = &support.stiffness {
                add(&mut self.facts.springs, 1)?;
                self.string(&stiffness.dof)?;
                self.quantity(&stiffness.value)?;
            }
        }
        for case in &m.load_cases {
            self.string(&case.id)?;
            self.optional(&case.provenance)?;
            let loads = &mut self.facts.primitive_loads;
            loads.length = loads.length.max(case.primitive_loads.len());
            loads.capacity = loads.capacity.max(case.primitive_loads.capacity());
            let total = u32::try_from(case.primitive_loads.len()).ok().and_then(|l| self.facts.total_loads.checked_add(l));
            self.facts.total_loads = total.ok_or(CensusStatus::ArithmeticOverflow)?;
            for load in &case.primitive_loads {
                for text in [&load.id, &load.category, &load.direction, &load.dimension] {
                    self.string(text)?;
                }
                self.optional(&load.provenance)?;
                match &load.target {
                    crate::LoadTargetInput::Node { node } => self.string(node)?,
                    crate::LoadTargetInput::Element { pipe } => self.string(pipe)?,
                }
                self.quantity(&load.magnitude)?;
            }
        }
        let units = &m.project.units;
        self.facts.units = borrowed_value_census(units);
        self.facts.units_text = raw_text_census(units);
        for status in [self.facts.units.status, self.facts.units_text.status] {
            if status != CensusStatus::Complete {
                return Err(status);
            }
        }
        Ok(())
    }
}
pub(super) fn nested_typed_census(request: &LinearStaticPreviewRequest) -> NestedTypedFacts {
    let empty = CapacityFact { length: 0, capacity: 0 };
    let mut walk = TypedWalk {
        facts: NestedTypedFacts {
            status: CensusStatus::Complete,
            strings: 0,
            max_string_bytes: 0,
            max_string_capacity: 0,
            string_capacity_bytes: 0,
            restraints: 0,
            restraint_capacity: 0,
            max_restraint_capacity: 0,
            springs: 0,
            max_temperature_points: empty,
            primitive_loads: empty,
            total_loads: 0,
            units: BorrowedValueFacts::empty(),
            units_text: raw_text_census(&Value::Null),
        },
    };
    if let Err(status) = walk.walk(request) {
        walk.facts.status = status;
    }
    walk.facts
}

// ---- D1: the domain predicate (DOMAIN.md as amended) ------------------------

/// D1's cap table: DOMAIN.md §2, with G2_AMENDMENTS.md §2 (S-3 typed capacities)
/// and §3 (S-4: sections = 0), and `l ≤ 128` (RR "U4 G4: the margin rule trips";
/// ADDENDUM_L128.md §4). Every pricing formula is evaluated at these caps.
/// B1 (option S3; RR "I82's addendum: B1's target is S3…"; PLAN_v2 §2.3): one tier at
/// these model caps with C = 3 load cases, every case `l_i ≤ 128`, and a stated
/// `Σ l_i ≤ L = 384` that does not bind (ADDENDUM_01 §2). `LOAD_CASES` and `TOTAL_LOADS`
/// change here, at SA (RV107 A1-N-11); SQ's registration re-prices the profile at them.
pub(super) mod caps {
    pub(crate) const NODES: usize = 32;
    pub(crate) const MEMBERS: usize = 32;
    pub(crate) const SUPPORTS: usize = 32;
    pub(crate) const RESTRAINTS: usize = 192;
    pub(crate) const SPRINGS: usize = 192;
    /// l: the primitive loads of each load case.
    pub(crate) const LOADS: usize = 128;
    pub(crate) const MATERIALS: usize = 4;
    pub(crate) const TEMPERATURE_POINTS: usize = 16;
    /// C: the load cases of one invocation (D1.4: 1 ≤ c ≤ C).
    pub(crate) const LOAD_CASES: usize = 3;
    /// L: the primitive loads over every load case (Σ l_i ≤ L). At option S3 it equals
    /// C·l, so it is stated and does not bind.
    pub(crate) const TOTAL_LOADS: usize = 384;
    pub(crate) const TEXT_BYTES: usize = 128;
    pub(crate) const RAW_VALUES: usize = 16_384;
    pub(crate) const RAW_DEPTH: usize = 16;
    pub(crate) const RAW_STRING_BYTES: usize = 65_536;
    pub(crate) const RAW_KEY_BYTES: usize = 65_536;
    pub(crate) const RAW_ARRAY_CAPACITY: usize = 32_768;
    pub(crate) const RAW_STRING_CAPACITY: usize = 131_072;
    pub(crate) const RAW_KEY_CAPACITY: usize = 131_072;
    pub(crate) const DIGEST_CAPACITY: usize = 128;
    /// D1.11: no control byte in any raw string value or key.
    pub(crate) const CONTROL_BYTES: usize = 0;
}

/// The clause that refused, by DOMAIN.md §1's numbering. Read by U3's facade
/// (grant 2, the public notice mapping) and the tests.
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum D1Clause {
    Caller,
    Build,
    Census,
    Namespace,
    Invocation,
    Case,
    Supports,
    Loads,
    Members,
    Caps,
    Provenance,
    ControlBytes,
}
impl D1Clause {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn id(self) -> &'static str {
        match self {
            Self::Caller => "D1.0",
            Self::Build => "D1.1",
            Self::Census => "D1.2",
            Self::Namespace => "D1.3",
            Self::Invocation => "D1.4",
            Self::Case => "D1.5",
            Self::Supports => "D1.6",
            Self::Loads => "D1.7",
            Self::Members => "D1.8",
            Self::Caps => "D1.9",
            Self::Provenance => "D1.10",
            Self::ControlBytes => "D1.11",
        }
    }
}
/// The accepted schema's `unavailable_precondition` kinds this law uses
/// (retained_precision_mp_v2.schema.json, `precondition` enum).
#[cfg_attr(not(test), allow(dead_code))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum UnavailablePrecondition {
    Caller,
    SourceFamily,
    ResourceAdmission,
}
impl UnavailablePrecondition {
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn as_str(self) -> &'static str {
        match self {
            Self::Caller => "caller",
            Self::SourceFamily => "source_family",
            Self::ResourceAdmission => "resource_admission",
        }
    }
}
/// The part of the census that was not complete (D1.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CensusPart {
    Raw,
    RawText,
    TypedDof,
    TypedNested,
    Headless,
}
/// The field a family clause (D1.3–D1.8, D1.10) found outside the domain.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum FamilyFact {
    SchemaVersion,
    PressureContract,
    ReferenceConfigurations,
    MaterialExpansionLaw,
    RequestExpansionLaws,
    Sections,
    SectionRef,
    LoadCases,
    Combinations,
    Components,
    PressureRegions,
    EquivalentStatic,
    ModulusBasisRef,
    ModulusBasisTemperature,
    AnalysisState,
    Hanger,
    Nonlinear,
    SupportFamily,
    LoadTarget,
    LoadDimension,
    ObjectProvenance,
}
/// The fact a cap row (D1.9, D1.11) bounds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum CapFact {
    Nodes,
    NodesCapacity,
    Members,
    MembersCapacity,
    Supports,
    SupportsCapacity,
    Restraints,
    RestraintCapacity,
    RestraintCapacityTotal,
    Springs,
    Loads,
    LoadsCapacity,
    /// B1 SA: Σ l_i over every load case.
    TotalLoads,
    ModelMaterials,
    ModelMaterialsCapacity,
    RequestMaterials,
    RequestMaterialsCapacity,
    TemperaturePoints,
    TemperaturePointsCapacity,
    LoadCasesCapacity,
    SectionsCapacity,
    ComponentsCapacity,
    CombinationsCapacity,
    RequestExpansionLawsCapacity,
    MaterialExpansionLawsCapacity,
    TypedTextBytes,
    TypedTextCapacity,
    RawTextBytes,
    RawKeyTextBytes,
    RawValues,
    RawDepth,
    RawStringBytes,
    RawKeyBytes,
    RawArrayCapacity,
    RawStringCapacity,
    RawKeyCapacity,
    DigestCapacity,
    UnitsValues,
    UnitsDepth,
    UnitsStringBytes,
    UnitsKeyBytes,
    UnitsArrayCapacity,
    UnitsStringCapacity,
    UnitsKeyCapacity,
    UnitsTextBytes,
    UnitsKeyTextBytes,
    ControlBytes,
}
/// Why the admission bound refused (S-1: E_mov,max + R ≤ M).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum BoundRefusal {
    /// The cap-priced maximum has no in-build expression yet (fail-closed).
    Unpriced,
    /// Checked arithmetic overflowed.
    Overflow,
    /// The required bytes exceed the threshold M.
    Exceeds { required: u64, threshold: u64 },
}
/// The first failing clause, with the facts it read (DOMAIN.md §3's refusal map).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum AdmissionRefusal {
    Caller(RetainedCaller),
    Profile(ProfileStatus),
    Census(CensusPart, CensusStatus),
    Family(D1Clause, FamilyFact),
    Cap { fact: CapFact, observed: usize, cap: usize },
    Bound(BoundRefusal),
}
#[cfg_attr(not(test), allow(dead_code))]
impl AdmissionRefusal {
    /// The clause, or `None` for the admission bound (which follows D1).
    pub(super) fn clause(&self) -> Option<D1Clause> {
        match self {
            Self::Caller(_) => Some(D1Clause::Caller),
            Self::Profile(_) => Some(D1Clause::Build),
            Self::Census(..) => Some(D1Clause::Census),
            Self::Family(clause, _) => Some(*clause),
            Self::Cap { fact: CapFact::ControlBytes, .. } => Some(D1Clause::ControlBytes),
            Self::Cap { .. } => Some(D1Clause::Caps),
            Self::Bound(_) => None,
        }
    }
    /// DOMAIN.md §3; D1.10 and D1.11 as confirmed in NOTES_G4.md §1.
    pub(super) fn precondition(&self) -> UnavailablePrecondition {
        match self {
            Self::Caller(_) => UnavailablePrecondition::Caller,
            Self::Family(..) => UnavailablePrecondition::SourceFamily,
            Self::Profile(_) | Self::Census(..) | Self::Cap { .. } | Self::Bound(_) => {
                UnavailablePrecondition::ResourceAdmission
            }
        }
    }
}
/// The facts the D1 predicate reads, borrowed from one admission's census.
pub(super) struct DomainFacts<'a> {
    pub(super) raw: &'a BorrowedValueFacts,
    pub(super) raw_text: &'a RawTextFacts,
    pub(super) typed: &'a BorrowedRequestFacts,
    pub(super) nested: &'a NestedTypedFacts,
    pub(super) headless: Option<&'a HeadlessRootFacts>,
    pub(super) digest: &'a CapacityFact,
}
fn census_complete_part(f: &DomainFacts<'_>) -> Result<(), AdmissionRefusal> {
    let incomplete = |part, status| match status {
        CensusStatus::Complete => Ok(()),
        status => Err(AdmissionRefusal::Census(part, status)),
    };
    incomplete(CensusPart::Raw, f.raw.status)?;
    incomplete(CensusPart::RawText, f.raw_text.status)?;
    if let Err(status) = f.typed.dof_upper {
        return Err(AdmissionRefusal::Census(CensusPart::TypedDof, status));
    }
    incomplete(CensusPart::TypedNested, f.nested.status)?;
    if let Some(h) = f.headless {
        incomplete(CensusPart::Headless, h.payload.status)?;
        incomplete(CensusPart::Headless, h.invocation.status)?;
    }
    Ok(())
}
/// D1.3–D1.8 over the borrowed typed request (DOMAIN.md §1, G2_AMENDMENTS §3).
fn family_clauses(request: &LinearStaticPreviewRequest) -> Result<(), AdmissionRefusal> {
    use crate::Authored;
    use D1Clause as C;
    use FamilyFact as F;
    let refuse = |clause, fact| Err(AdmissionRefusal::Family(clause, fact));
    let m = &request.model;
    // D1.3: the legacy source-blocks namespace, no sections (S-4).
    if !matches!(m.schema_version.as_str(), "0.1.0" | "0.2.0") {
        return refuse(C::Namespace, F::SchemaVersion);
    }
    if m.pressure_contract.is_some() {
        return refuse(C::Namespace, F::PressureContract);
    }
    if !matches!(m.reference_configurations, Authored::Absent) {
        return refuse(C::Namespace, F::ReferenceConfigurations);
    }
    if m.material_expansion_laws.iter().any(|law| !matches!(law, Authored::Absent)) {
        return refuse(C::Namespace, F::MaterialExpansionLaw);
    }
    if !m.request_material_expansion_laws.is_empty() {
        return refuse(C::Namespace, F::RequestExpansionLaws);
    }
    if !m.sections.is_empty() {
        return refuse(C::Namespace, F::Sections);
    }
    if m.pipe_segments.iter().any(|pipe| pipe.section_ref.is_some()) {
        return refuse(C::Namespace, F::SectionRef);
    }
    // D1.4 (B1 SA): 1 ≤ c ≤ C load cases, no combinations or components.
    if m.load_cases.is_empty() || m.load_cases.len() > caps::LOAD_CASES {
        return refuse(C::Invocation, F::LoadCases);
    }
    if !m.combinations.is_empty() {
        return refuse(C::Invocation, F::Combinations);
    }
    if !m.components.is_empty() {
        return refuse(C::Invocation, F::Components);
    }
    // D1.5 (B1 SA): every case, in request order.
    for case in &m.load_cases {
        if case.pressure_regions.is_some() {
            return refuse(C::Case, F::PressureRegions);
        }
        if case.equivalent_static.is_some() {
            return refuse(C::Case, F::EquivalentStatic);
        }
        if case.modulus_basis_ref.is_some() {
            return refuse(C::Case, F::ModulusBasisRef);
        }
        if case.modulus_basis_temperature.is_some() {
            return refuse(C::Case, F::ModulusBasisTemperature);
        }
        if !matches!(case.analysis_state, Authored::Absent) {
            return refuse(C::Case, F::AnalysisState);
        }
    }
    // D1.6: rigid restraints and one scalar spring only; exact family strings.
    for support in &m.supports {
        if support.hanger.is_some() {
            return refuse(C::Supports, F::Hanger);
        }
        if support.nonlinear.is_some() {
            return refuse(C::Supports, F::Nonlinear);
        }
        if let Some(family) = &support.family {
            if !matches!(family.as_str(), "anchor" | "guide" | "line_stop" | "vertical_support" | "spring") {
                return refuse(C::Supports, F::SupportFamily);
            }
        }
    }
    // D1.7: nodal force and moment primitives only, in every case (B1 SA).
    for load in m.load_cases.iter().flat_map(|case| &case.primitive_loads) {
        if !matches!(load.target, crate::LoadTargetInput::Node { .. }) {
            return refuse(C::Loads, F::LoadTarget);
        }
        if !matches!(load.dimension.as_str(), "force" | "moment") {
            return refuse(C::Loads, F::LoadDimension);
        }
    }
    // D1.8: straight members only. With no components (D1.4) every pipe segment
    // is a straight member, so this re-states D1.4 for the members it implies.
    if !m.components.is_empty() {
        return refuse(C::Members, F::Components);
    }
    Ok(())
}
/// One D1.9/D1.11 row: an observed fact against its cap.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct CapRow {
    pub(super) fact: CapFact,
    pub(super) observed: usize,
    pub(super) cap: usize,
}
pub(super) const CAP_ROWS: usize = 47;
/// D1.9's rows in DOMAIN.md §2 order, then D1.11's (the last row). B1 SA: `Loads` and
/// `LoadsCapacity` bound every case (the census's maxima over cases), `TotalLoads` bounds
/// Σ l_i, and `LoadCasesCapacity` is capped by C.
pub(super) fn cap_rows(f: &DomainFacts<'_>) -> [CapRow; CAP_ROWS] {
    use caps::*;
    use CapFact as K;
    let (t, n, raw, u) = (f.typed, f.nested, f.raw, &f.nested.units);
    let row = |fact, observed, cap| CapRow { fact, observed, cap };
    [
        row(K::Nodes, t.nodes.length, NODES),
        row(K::NodesCapacity, t.nodes.capacity, NODES),
        row(K::Members, t.members.length, MEMBERS),
        row(K::MembersCapacity, t.members.capacity, MEMBERS),
        row(K::Supports, t.supports.length, SUPPORTS),
        row(K::SupportsCapacity, t.supports.capacity, SUPPORTS),
        row(K::Restraints, n.restraints, RESTRAINTS),
        row(K::RestraintCapacity, n.max_restraint_capacity, RESTRAINTS),
        row(K::RestraintCapacityTotal, n.restraint_capacity, RESTRAINTS),
        row(K::Springs, n.springs, SPRINGS),
        row(K::Loads, n.primitive_loads.length, LOADS),
        row(K::LoadsCapacity, n.primitive_loads.capacity, LOADS),
        row(K::TotalLoads, n.total_loads as usize, TOTAL_LOADS),
        row(K::ModelMaterials, t.model_materials.length, MATERIALS),
        row(K::ModelMaterialsCapacity, t.model_materials.capacity, MATERIALS),
        row(K::RequestMaterials, t.request_materials.length, MATERIALS),
        row(K::RequestMaterialsCapacity, t.request_materials.capacity, MATERIALS),
        row(K::TemperaturePoints, n.max_temperature_points.length, TEMPERATURE_POINTS),
        row(K::TemperaturePointsCapacity, n.max_temperature_points.capacity, TEMPERATURE_POINTS),
        row(K::LoadCasesCapacity, t.load_cases.capacity, LOAD_CASES),
        row(K::SectionsCapacity, t.sections.capacity, 0),
        row(K::ComponentsCapacity, t.components.capacity, 0),
        row(K::CombinationsCapacity, t.combinations.capacity, 0),
        row(K::RequestExpansionLawsCapacity, t.request_expansion_law_indices.capacity, 0),
        row(K::MaterialExpansionLawsCapacity, t.material_expansion_laws.capacity, MATERIALS),
        row(K::TypedTextBytes, n.max_string_bytes, TEXT_BYTES),
        row(K::TypedTextCapacity, n.max_string_capacity, TEXT_BYTES),
        row(K::RawTextBytes, f.raw_text.max_string_bytes, TEXT_BYTES),
        row(K::RawKeyTextBytes, f.raw_text.max_key_bytes, TEXT_BYTES),
        row(K::RawValues, raw.values, RAW_VALUES),
        row(K::RawDepth, raw.maximum_depth, RAW_DEPTH),
        row(K::RawStringBytes, raw.string_bytes, RAW_STRING_BYTES),
        row(K::RawKeyBytes, raw.key_bytes, RAW_KEY_BYTES),
        row(K::RawArrayCapacity, raw.array_capacity_elements, RAW_ARRAY_CAPACITY),
        row(K::RawStringCapacity, raw.string_capacity_bytes, RAW_STRING_CAPACITY),
        row(K::RawKeyCapacity, raw.key_capacity_bytes, RAW_KEY_CAPACITY),
        row(K::DigestCapacity, f.digest.capacity, DIGEST_CAPACITY),
        row(K::UnitsValues, u.values, RAW_VALUES),
        row(K::UnitsDepth, u.maximum_depth, RAW_DEPTH),
        row(K::UnitsStringBytes, u.string_bytes, RAW_STRING_BYTES),
        row(K::UnitsKeyBytes, u.key_bytes, RAW_KEY_BYTES),
        row(K::UnitsArrayCapacity, u.array_capacity_elements, RAW_ARRAY_CAPACITY),
        row(K::UnitsStringCapacity, u.string_capacity_bytes, RAW_STRING_CAPACITY),
        row(K::UnitsKeyCapacity, u.key_capacity_bytes, RAW_KEY_CAPACITY),
        row(K::UnitsTextBytes, n.units_text.max_string_bytes, TEXT_BYTES),
        row(K::UnitsKeyTextBytes, n.units_text.max_key_bytes, TEXT_BYTES),
        row(K::ControlBytes, f.raw_text.control_bytes, CONTROL_BYTES),
    ]
}
/// The first row whose observed fact exceeds its cap.
pub(super) fn first_cap_violation(rows: &[CapRow]) -> Result<(), AdmissionRefusal> {
    match rows.iter().find(|r| r.observed > r.cap) {
        Some(r) => Err(AdmissionRefusal::Cap { fact: r.fact, observed: r.observed, cap: r.cap }),
        None => Ok(()),
    }
}
/// D1.10 (RR "U4 G3 verified"): no primitive load's provenance begins, after
/// ASCII whitespace, with `{`. Reads one byte per load; no parse.
fn provenance_clause(request: &LinearStaticPreviewRequest) -> Result<(), AdmissionRefusal> {
    let object = |text: &str| text.trim_start_matches(|c: char| c.is_ascii_whitespace()).starts_with('{');
    let loads = request.model.load_cases.iter().flat_map(|case| &case.primitive_loads);
    for load in loads {
        if load.provenance.as_deref().is_some_and(object) {
            return Err(AdmissionRefusal::Family(D1Clause::Provenance, FamilyFact::ObjectProvenance));
        }
    }
    Ok(())
}
/// D1.2–D1.11 over one census, in clause order: the request-level domain, which
/// does not depend on the caller or the build. Pure and allocation-free.
pub(super) fn domain_clauses(f: &DomainFacts<'_>, request: &LinearStaticPreviewRequest) -> Result<(), AdmissionRefusal> {
    census_complete_part(f)?;
    family_clauses(request)?;
    let rows = cap_rows(f);
    first_cap_violation(&rows[..CAP_ROWS - 1])?;
    provenance_clause(request)?;
    first_cap_violation(&rows[CAP_ROWS - 1..])
}
/// D1.0: Direct only (D-2; U3 F-5: `admit` refuses Headless).
fn caller_clause(caller: RetainedCaller) -> Result<(), AdmissionRefusal> {
    match caller {
        RetainedCaller::Direct => Ok(()),
        caller => Err(AdmissionRefusal::Caller(caller)),
    }
}

// ---- D-6: the build status --------------------------------------------------

/// The compiled identity text from build.rs (`None` when the variable is absent,
/// for example in a build without the script: Stale, never a compile error).
const COMPILED_IDENTITY: Option<&str> = option_env!("OPS_RETAINED_BUILD_IDENTITY");
/// The compiled reviewed-input record from build.rs: the PP lock and the
/// precommit reader's 13 `include_str!` inputs, by SHA-256 (D-6 as extended by
/// RR "U4 G4: the margin rule trips").
const COMPILED_REVIEWED_INPUTS: Option<&str> = option_env!("OPS_RETAINED_REVIEWED_INPUTS");

/// BUILD.md §2.3: the layouts the formulas assume. Evaluated by the compiler; a
/// false witness makes a registered profile Stale. Never a compile error.
pub(super) const LAYOUT_WITNESSES: bool = {
    use std::mem::{align_of, size_of};
    size_of::<usize>() == 8
        && size_of::<serde_json::Number>() == 16
        && size_of::<serde_json::Map<String, Value>>()
            == size_of::<std::collections::BTreeMap<String, Value>>()
        && size_of::<Value>() == 32
        && align_of::<Value>() == 8
        && size_of::<String>() == 24
        && size_of::<Vec<u8>>() == 24
};
/// One in-build layout a registered profile records (RR "U4 G4": the reader
/// types T17 prices). A recorded layout different from the build's is Stale.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct TypeLayout {
    pub(super) size: usize,
    pub(super) align: usize,
}
const fn layout_of<T>() -> TypeLayout {
    TypeLayout { size: std::mem::size_of::<T>(), align: std::mem::align_of::<T>() }
}
#[cfg_attr(not(test), allow(dead_code))]
pub(super) const READER_LAYOUT_NAMES: [&str; 4] = ["Validation", "ValidationError", "RowClassification", "AccuracyClass"];
pub(super) const READER_LAYOUTS: [TypeLayout; 4] = {
    use open_pipe_stress_result_export::retained_precision as rx;
    [
        layout_of::<rx::Validation>(),
        layout_of::<rx::ValidationError>(),
        layout_of::<rx::RowClassification>(),
        layout_of::<rx::AccuracyClass>(),
    ]
};

/// A qualified build (G6): its identity, reviewed inputs, reader layouts and M.
/// Values exist only as entries of `REGISTERED_PROFILES`, added at G6 by a
/// reviewed source change; there is no constructor and no test value.
pub(super) struct RegisteredProfile {
    /// The exact D-6 identity text of the qualified build.
    identity: &'static str,
    /// The exact reviewed-input text of the qualified build.
    reviewed_inputs: &'static str,
    /// `READER_LAYOUTS` as qualified.
    reader_layouts: [TypeLayout; 4],
    /// M (D-7): the per-invocation admission threshold on requested and moving
    /// heap bytes plus R, selected by ROOT at G6.
    threshold_bytes: u64,
}
/// The registered builds (G6, R/I65/u4_g6_01/QUALIFICATION.md): the qualified
/// dev/test build only. Decision 7 still holds: this is the production profile,
/// registered by reviewed change, not a test permit; every other build identity is
/// `Stale` and keeps the ordinary route.
static REGISTERED_PROFILES: &[RegisteredProfile] = &[RegisteredProfile {
    // aarch64-apple-darwin, rustc 1.97.1 (8bab26f4f68e), profile=debug, opt_level=0,
    // debug_assertions=true, panic=unwind, no RUSTFLAGS.
    identity: "v1;rustc.release=1.97.1;rustc.commit=8bab26f4f68e0e26f0bb7960be334d5b520ea452;rustc.host=aarch64-apple-darwin;rustc.llvm=22.1.6;target=aarch64-apple-darwin;target.arch=aarch64;target.pointer_width=64;target.endian=little;target.os=macos;target.env=;panic=unwind;profile=debug;opt_level=0;debug_assertions=true;rustflags=;pkg=open_pipe_stress_product_physics@0.2.0",
    reviewed_inputs: "v1;Cargo.lock=4f494db6d8a6eca87e7a16d8561197f20b1951a033bd3a6c424acfff5613475b;../../schemas/physics_source_recovery.schema.json=3bb969555d5616af6eefdb68788ee4a74a8a3681c42fe9aae577a5d25a51ac5c;../../schemas/retained_precision_mp_v2.schema.json=07951edacfedd410c153929ee75bb5bada15dbd222369ec63240c678b233b61c;../../fixtures/results/retained_precision_prepared_ordinary_v1.json=3e0779a45a74cf0bb3a4ed08ed3a6b44347aea8a3c33b59e9dd92130426ee296;../../fixtures/results/semantic_contract_v0_3_preview_physics_retained_1.json=c74742ce6a936384e00986006e6a0b2e6bb11f190451e876eed9ffa11903c6a8;../../fixtures/results/semantic_contract_v0_2.json=4d6886d19e304db897e5e9f8f0054cbee91ba7795868f9698e2bbe070bde94da;../../fixtures/results/semantic_contract_v0_3_precision_1.json=d75aacee175e178dbdeb256d89a65f4b375265f7da077725ee635af33df51d7e;../../fixtures/results/semantic_contract_v0_3_physics_1.json=9a2cf6268b57bd5265a1a115497c07450819dd4d03cd5ab618097bd9d19da8cc;../../fixtures/results/semantic_contract_v0_3_load_reference_1.json=44bc41c06f589fab6ce931ac0eaa5344765ff64fd5f880cc2dd69ecb839c4f4d;../../fixtures/results/semantic_contract_v0_3_load_reference_source_1.json=d1628194a7730f427843b00228dd233cf92b8e7d26f3bc31c660a3ea59e28337;../../fixtures/results/semantic_contract_v0_3_preview_physics_1.json=ae55503d44a4750714a35c423623e38cf4132099134097193024d1635bfbc88a;../../fixtures/results/semantic_contract_v0_3_physics_source_1.json=ba13f2aefd7a38bd725e5f111e6ec30144bc8776aa957c6278ee7b1178298ba1;../../fixtures/results/semantic_contract_v0_3_source_blocks_1.json=5f299065f15a157bbedf9467a598994ae684c4ecb3f851bbcb291981ec550a9f;../../schemas/source_block_recovery.schema.json=544e196d2f7bef27276acc160aa19ab738a4f7949e846d2e8871328d2208129c",
    reader_layouts: [
        TypeLayout { size: 56, align: 8 },
        TypeLayout { size: 64, align: 8 },
        TypeLayout { size: 96, align: 8 },
        TypeLayout { size: 16, align: 8 },
    ],
    // M (D-7, proposed in QUALIFICATION.md §6): E_mov,max + R <= 0.8927 M in this build.
    threshold_bytes: 4_026_531_840,
}];

/// D-6 identity matching (G2_AMENDMENTS §1): `Missing` with nothing registered;
/// otherwise the index of the byte-equal registered identity, or `Stale` when the
/// compiled value is absent, unavailable or unequal. Pure.
pub(super) fn identity_match<'a>(
    compiled: Option<&str>,
    mut registered: impl ExactSizeIterator<Item = &'a str>,
) -> Result<usize, ProfileStatus> {
    if registered.len() == 0 {
        return Err(ProfileStatus::Missing);
    }
    let compiled = match compiled {
        Some(text) if text != build_identity::IDENTITY_UNAVAILABLE => text,
        _ => return Err(ProfileStatus::Stale),
    };
    registered.position(|identity| identity == compiled).ok_or(ProfileStatus::Stale)
}
/// The bindings a matched identity also needs: the layout witnesses, the
/// reviewed inputs (byte-equal, and every input read: a field build.rs could
/// not read, `<path>=unavailable`, is a mismatch exactly as `identity_match`
/// treats an unavailable identity, RV89 N-4) and the reader layouts. Pure.
pub(super) fn bindings_hold(
    witnesses: bool,
    compiled_inputs: Option<&str>,
    registered_inputs: &str,
    build_layouts: &[TypeLayout],
    registered_layouts: &[TypeLayout],
) -> bool {
    let every_input_read = |text: &str| !text.split(';').any(|field| field.ends_with("=unavailable"));
    witnesses
        && compiled_inputs.is_some_and(every_input_read)
        && compiled_inputs == Some(registered_inputs)
        && build_layouts == registered_layouts
}
/// D1.1's status of this build: a registered profile's index, or why not.
fn build_status() -> Result<usize, ProfileStatus> {
    let index = identity_match(COMPILED_IDENTITY, REGISTERED_PROFILES.iter().map(|p| p.identity))?;
    let p = &REGISTERED_PROFILES[index];
    if bindings_hold(LAYOUT_WITNESSES, COMPILED_REVIEWED_INPUTS, p.reviewed_inputs, &READER_LAYOUTS, &p.reader_layouts) {
        Ok(index)
    } else {
        Err(ProfileStatus::Stale)
    }
}

// ---- The admission bound (S-1, S-5) -----------------------------------------

/// S1 (D-3 = S1; STACK_PLAN.md §3, STACK_INVENTORY.md §5): the reserved stack R,
/// a priced term of the profile, and the witness divisor k (witness at R/k).
pub(super) const RESERVED_STACK_BYTES: usize = 64 << 20;
#[cfg_attr(not(test), allow(dead_code))]
pub(super) const STACK_WITNESS_DIVISOR: usize = 16;
#[cfg(test)]
thread_local! {
    /// The `cfg(test)` override of R (STACK_PLAN.md §4), read on the caller's thread.
    pub(super) static RESERVED_STACK_OVERRIDE: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) };
}
fn reserved_stack() -> usize {
    #[cfg(test)]
    if let Some(bytes) = RESERVED_STACK_OVERRIDE.with(std::cell::Cell::get) {
        return bytes;
    }
    RESERVED_STACK_BYTES
}
/// A reserved-stack spawn failure (U3's `W1Fallback::StackReservation`) refuses
/// under this precondition (DOMAIN.md §3).
#[allow(dead_code)]
pub(super) const STACK_RESERVATION_PRECONDITION: UnavailablePrecondition = UnavailablePrecondition::ResourceAdmission;
/// The cap-priced admission maximum without R (E_mov,max): the maximum over both branches of
/// every phase through caller completion (COMPOSITION_G4.md §1; RR "RV84 on U4 G3", S-5), in
/// requested plus moving heap bytes, for the invocation's mode, evaluated in this build by the
/// generated `profile`. It fails closed (Unpriced) while any atom is still a design-record
/// Estimate (G6 closes them), and on overflow.
fn cap_priced_maximum(mode: crate::PreviewSolverMode) -> Result<u64, BoundRefusal> {
    priced_maximum(profile::ESTIMATES, mode)
}
/// The generated profile's in-build maximum for a mode (without R), or `Unpriced`
/// while any Estimate atom remains. Pure.
pub(super) fn priced_maximum(estimates: usize, mode: crate::PreviewSolverMode) -> Result<u64, BoundRefusal> {
    if estimates != 0 {
        return Err(BoundRefusal::Unpriced);
    }
    let maximum = match mode {
        crate::PreviewSolverMode::SparseInteractive => profile::SPARSE,
        crate::PreviewSolverMode::DenseScrutiny => profile::DENSE,
    };
    maximum.map(|(bytes, _)| bytes).ok_or(BoundRefusal::Overflow)
}

// ---- BEGIN GENERATED PROFILE (part2/_run_records/g5_profile.py from profile_tree.json; do not edit by hand) ----
/// U4 G5 part 2: the cap-priced admission maximum as named in-build expressions. Every term is a
/// linear form over layout atoms at the D1 caps (l <= 128); every maximum (stages, phases, moving
/// candidates) is taken here, in the build. Source: the G4 chain with RV84/RV87's corrections at
/// NUM 1e323058f3 (G5 part 1 code); text at l <= 128 on the R-4 graph.
pub(super) mod profile {
    #![allow(clippy::all, dead_code)]
    use open_pipe_stress_frame_kernel::structural::retained_resource as fkr;
    use serde_json::Value;
    use std::mem::{align_of, size_of};

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub(crate) enum Binding {
        /// size_of/align_of of the actual type in this build.
        InBuild,
        /// A source-derived upper bound for a type the product cannot name (cited).
        SourceUpper,
        /// A text byte count from the T08 closure (layout-free).
        Text,
        /// A G3 design-record stride with no in-build type identified: the profile is incomplete.
        Estimate,
    }
    const fn up(x: usize, a: usize) -> usize {
        (x + a - 1) / a * a
    }
    const fn max_usize(a: usize, b: usize) -> usize {
        if a > b { a } else { b }
    }
    /// BUILD.md §4: a BTreeMap<K, V> node, max(Leaf_up, Internal_up), at A = max(8, align K, align V).
    pub(crate) const fn btree_node_upper(sk: usize, ak: usize, sv: usize, av: usize) -> usize {
        let a = max_usize(8, max_usize(ak, av));
        let leaf = up(8, a) + up(2, a) + up(2, a) + up(11 * sk, a) + up(11 * sv, a);
        let internal = up(up(leaf, 8) + 12 * 8, a);
        max_usize(leaf, internal)
    }
    /// The text atoms of the T08 closure at this basis (byte counts, layout-free), and the
    /// longest-string atoms of the hash route (RV84 C-N1; RV87 N-3).
    pub(crate) const TEXT_D: u64 = 14734; // D
    pub(crate) const TEXT_D_ENV: u64 = 9361; // D_env
    pub(crate) const TEXT_TAV_TEXT_MOVING: u64 = 2153400792; // TAV_text_moving
    pub(crate) const TEXT_TAV_TEXT_REQUESTED: u64 = 2150800830; // TAV_text_requested
    pub(crate) const TEXT_TEXT_AUDIT_ERROR: u64 = 16384; // Text(audit_error)
    pub(crate) const TEXT_TEXT_DIAG_ENV: u64 = 68720236; // Text(diag_env)
    pub(crate) const TEXT_TEXT_DIAG_TOTAL: u64 = 94906464; // Text(diag_total)
    pub(crate) const TEXT_TEXT_ERR: u64 = 16384; // Text(err)
    pub(crate) const TEXT_TEXT_FORMATION_DETAIL: u64 = 16426; // Text(formation_detail)
    pub(crate) const TEXT_TEXT_RECOVERY_FINDING: u64 = 10466306; // Text(recovery_finding)
    pub(crate) const TEXT_TEXT_ROW: u64 = 11474; // Text(row)
    pub(crate) const TEXT_TEXT_SYM: u64 = 368; // Text(sym)
    pub(crate) const L_DIAGID: u64 = 2330;
    pub(crate) const L_PUB: u64 = 2599962;
    pub(crate) const ATOMS: usize = 244;
    /// The atoms, in index order: their names (as the records write them) and bindings.
    pub(crate) const ATOM_NAMES: [&str; ATOMS] = [
        "Node(&String,())",
        "Node(&str,&ResultItem)",
        "Node(&str,())",
        "Node((String,String),())",
        "Node(String,())",
        "Node(String,BTreeMap)",
        "Node(String,ResultItem)",
        "Node(String,RowTreatment)",
        "Node(String,Value)",
        "Node(TrackerKey,())",
        "Node(TrackerKey,Tracker)",
        "Node(usize,())",
        "Node(usize,ResultItem)",
        "Node(usize,String)",
        "Text(audit_error)",
        "Text(err)",
        "Text(formation_detail)",
        "Text(recovery_finding)",
        "Text(row)",
        "Text(sym)",
        "s(&Value)",
        "s(&str)",
        "s((&str,&T))",
        "s((&str,&Value))",
        "s((&str,StressRecoveryResult))",
        "s((&str,usize))",
        "s((&str,usize,u64))",
        "s((Content,Content))",
        "s((EWS,EWS,f64))",
        "s((QuantityId,Binary64Outcome))",
        "s((QuantityId,u64))",
        "s((String,DerivedSection))",
        "s((String,ResultItem))",
        "s((String,String))",
        "s((String,[f64;3]))",
        "s((String,[f64;6]))",
        "s((String,f64,f64))",
        "s((String,usize))",
        "s((Wide,Wide,Wide))",
        "s((bool,f64,Wide))",
        "s((f64,f64))",
        "s((u32,Kind,u64))",
        "s((u32,SpringKind))",
        "s((usize,(f64,f64)))",
        "s((usize,ExactAccumulator,bool))",
        "s((usize,LedgerNet))",
        "s((usize,PublishedValue))",
        "s((usize,VecPair))",
        "s((usize,[f64;3],usize,[f64;3]))",
        "s((usize,f64))",
        "s((usize,f64,f64))",
        "s((usize,usize))",
        "s((usize,usize,Matrix12))",
        "s((usize,usize,bool))",
        "s(AdapterSnapshot)",
        "s(AffineTerm)",
        "s(AliasE)",
        "s(AnalysisStatus)",
        "s(ArcCasePrep)",
        "s(ArcGroupPrep)",
        "s(ArcPrepared)",
        "s(AttemptRecord)",
        "s(Authored<Vec<ExpansionLawInput>>)",
        "s(BasisRecord)",
        "s(Binary64Outcome)",
        "s(BlockBound<16>)",
        "s(BlockBound<4>)",
        "s(BlockBound<8>)",
        "s(BlockCertificate<16>)",
        "s(BlockCertificate<4>)",
        "s(BlockCertificate<8>)",
        "s(BlockNorms<16>)",
        "s(BlockNorms<4>)",
        "s(BlockNorms<8>)",
        "s(BlockRefusal)",
        "s(BlockWitness)",
        "s(BodyCoverage)",
        "s(BodyGeometry)",
        "s(BodyMapE)",
        "s(BodyReport<16>)",
        "s(BodyReport<4>)",
        "s(BodyReport<8>)",
        "s(BoundedCoefficients<16>)",
        "s(BoundedCoefficients<4>)",
        "s(BoundedCoefficients<8>)",
        "s(CaptureError)",
        "s(ConstraintMapE)",
        "s(ContributionE)",
        "s(ContributionRounding)",
        "s(ConversionE)",
        "s(ConversionEvent)",
        "s(CoverageFact)",
        "s(Derived)",
        "s(Diagnostic)",
        "s(EnclosureE)",
        "s(ExactAccumulator)",
        "s(ExactWideSum)",
        "s(Expansion)",
        "s(Expansion6)",
        "s(FinalRowConversion)",
        "s(FinalizedSourceBlockCase)",
        "s(ForceContribution)",
        "s(ForceTerm)",
        "s(FrameElement)",
        "s(FrameNode)",
        "s(FunctionalDescriptor)",
        "s(IntervalE)",
        "s(LaneTerminal)",
        "s(LawE)",
        "s(LazyE)",
        "s(LinearSupport)",
        "s(LoadFidelityRow)",
        "s(MaterialDescriptor)",
        "s(MaterialInput)",
        "s(MaterialSelection)",
        "s(MaximumE)",
        "s(MechanicsEnvelope)",
        "s(MemberIdentity)",
        "s(MemberMapE)",
        "s(MemberOperators<16>)",
        "s(MemberOperators<4>)",
        "s(MemberOperators<8>)",
        "s(MemberRecord)",
        "s(MemberRecovery)",
        "s(NodalLoadContribution)",
        "s(NodalMapE)",
        "s(NodeMapE)",
        "s(Node_SR)",
        "s(OperationalSpent)",
        "s(Option<BoundRefusal>)",
        "s(Option<FormationRecord>)",
        "s(Option<Wide<16>>)",
        "s(Option<Wide<4>>)",
        "s(Option<Wide<8>>)",
        "s(Option<[f64;3]>)",
        "s(Option<f64>)",
        "s(Option<i32>)",
        "s(Option<usize>)",
        "s(OrdinarySeed)",
        "s(Pair)",
        "s(PivotEvidence)",
        "s(PivotScreen<16>)",
        "s(PivotScreen<4>)",
        "s(PivotScreen<8>)",
        "s(PrecisionState)",
        "s(PreparedAttemptView)",
        "s(PreparedMemberEvent)",
        "s(PreviewLoadCase)",
        "s(PreviewNode)",
        "s(PreviewPipe)",
        "s(PreviewSupport)",
        "s(PrimitiveLoad)",
        "s(PrimitiveLoadInput)",
        "s(ProductMemberFacts)",
        "s(ProductRecipe)",
        "s(ProductRow)",
        "s(ProductRowSpec)",
        "s(ProductRowVerdict)",
        "s(ProductValue)",
        "s(Projection)",
        "s(ProjectionOutcome)",
        "s(PublishedRow)",
        "s(PublishedValue)",
        "s(QuadraticStressSpan)",
        "s(QualifiedFunctionalProjection)",
        "s(QualifiedProjection)",
        "s(QuantityMeta)",
        "s(Ratio)",
        "s(RecordOutcome)",
        "s(RecordedCase)",
        "s(RecordedInvocation)",
        "s(RecoveryRecord)",
        "s(RegistryE)",
        "s(ResidualRow)",
        "s(ResultBasisRef)",
        "s(ResultItem)",
        "s(RetainedFunctionalProjection)",
        "s(RetainedProjection)",
        "s(RetainedSolve)",
        "s(Row6)",
        "s(RowBinding)",
        "s(RowClassification)",
        "s(RowTreatment)",
        "s(RunRow)",
        "s(SectionMapE)",
        "s(Shared<16>)",
        "s(Shared<4>)",
        "s(Shared<8>)",
        "s(Solved<16>)",
        "s(Solved<4>)",
        "s(Solved<8>)",
        "s(SolverObservations)",
        "s(SpringAction)",
        "s(SpringEntry)",
        "s(SpringIdentity)",
        "s(SpringMapE)",
        "s(SrcConstraint)",
        "s(SrcCoord)",
        "s(SrcMember)",
        "s(SrcNodal)",
        "s(SrcSpring)",
        "s(SrcStation)",
        "s(SrcSupport)",
        "s(StationMapE)",
        "s(StationResultants)",
        "s(StiffnessContribution)",
        "s(StraightPipeElement)",
        "s(StressFinding)",
        "s(String)",
        "s(SupportActions)",
        "s(SupportCoverage)",
        "s(SupportFinding)",
        "s(SupportMapE)",
        "s(SupportVector)",
        "s(SymmetricMatrixEntry)",
        "s(TableE)",
        "s(Tag)",
        "s(TemperaturePoint)",
        "s(TermIdentity)",
        "s(ThreadPacketOutput)",
        "s(TrackerE)",
        "s(Validation)",
        "s(Value)",
        "s(Vec)",
        "s(Vec<&ForceTerm>)",
        "s(Vec<Expansion>)",
        "s(Vec<Wide>)",
        "s(Vec<f64>)",
        "s(Vec<usize>)",
        "s(VerificationReport<16>)",
        "s(VerificationReport<4>)",
        "s(VerificationReport<8>)",
        "s(VerifyShared<16>)",
        "s(VerifyShared<4>)",
        "s(VerifyShared<8>)",
        "s(Wide<16>)",
        "s(Wide<4>)",
        "s(Wide<8>)",
        "s([PublishedValue;12])",
        "s([bool;6])",
        "s(f64)",
        "s(u32)",
        "s(u64)",
        "s(usize)",
    ];
    pub(crate) const ATOM_BINDINGS: [Binding; ATOMS] = [
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::Text,
        Binding::Text,
        Binding::Text,
        Binding::Text,
        Binding::Text,
        Binding::Text,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::SourceUpper,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
        Binding::InBuild,
    ];
    /// The in-build value of each atom (Estimate atoms carry their design-record stride).
    pub(crate) const ATOM_VALUES: [u64; ATOMS] = [
        (btree_node_upper(size_of::<&'static String>(), align_of::<&'static String>(), size_of::<()>(), align_of::<()>())) as u64, // Node(&String,()) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<&'static str>(), align_of::<&'static str>(), size_of::<&'static crate::ResultItem>(), align_of::<&'static crate::ResultItem>())) as u64, // Node(&str,&ResultItem) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<&'static str>(), align_of::<&'static str>(), size_of::<()>(), align_of::<()>())) as u64, // Node(&str,()) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<(String, String)>(), align_of::<(String, String)>(), size_of::<()>(), align_of::<()>())) as u64, // Node((String,String),()) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<String>(), align_of::<String>(), size_of::<()>(), align_of::<()>())) as u64, // Node(String,()) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<String>(), align_of::<String>(), size_of::<std::collections::BTreeMap<String, String>>(), align_of::<std::collections::BTreeMap<String, String>>())) as u64, // Node(String,BTreeMap) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<String>(), align_of::<String>(), size_of::<crate::ResultItem>(), align_of::<crate::ResultItem>())) as u64, // Node(String,ResultItem) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<String>(), align_of::<String>(), up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<&'static str>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Option<String>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Option<&'static str>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Vec<String>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>()))) as u64, // Node(String,RowTreatment) (BUILD.md §4 node at K = String and V = RowTreatment's field-sum upper (monotone in the value size))
        (btree_node_upper(size_of::<String>(), align_of::<String>(), size_of::<Value>(), align_of::<Value>())) as u64, // Node(String,Value) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper((up(1, max_usize(4, fkr::KIND_ALIGN)) + up(4, max_usize(4, fkr::KIND_ALIGN)) + up(fkr::KIND, max_usize(4, fkr::KIND_ALIGN))), max_usize(4, fkr::KIND_ALIGN), 0, 1)) as u64, // Node(TrackerKey,()) (TrackerSet.holding: BTreeSet<(RuleTest, u32, Kind)>)
        (btree_node_upper((up(1, max_usize(4, fkr::KIND_ALIGN)) + up(4, max_usize(4, fkr::KIND_ALIGN)) + up(fkr::KIND, max_usize(4, fkr::KIND_ALIGN))), max_usize(4, fkr::KIND_ALIGN), fkr::TRACKER, fkr::TRACKER_ALIGN)) as u64, // Node(TrackerKey,Tracker) (TrackerSet.trackers: BTreeMap<(RuleTest, u32, Kind), BoundedExtremeTracker>; RuleTest (private, fieldless) <= 1 byte)
        (btree_node_upper(size_of::<usize>(), align_of::<usize>(), size_of::<()>(), align_of::<()>())) as u64, // Node(usize,()) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<usize>(), align_of::<usize>(), size_of::<crate::ResultItem>(), align_of::<crate::ResultItem>())) as u64, // Node(usize,ResultItem) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        (btree_node_upper(size_of::<usize>(), align_of::<usize>(), size_of::<String>(), align_of::<String>())) as u64, // Node(usize,String) (BUILD.md §4: max(Leaf_up, Internal_up) at the in-build size and alignment of K and V)
        16384, // Text(audit_error): text closure
        16384, // Text(err): text closure
        16426, // Text(formation_detail): text closure
        10466306, // Text(recovery_finding): text closure
        11474, // Text(row): text closure
        368, // Text(sym): text closure
        (size_of::<&'static Value>()) as u64, // s(&Value)
        (size_of::<&'static str>()) as u64, // s(&str)
        (size_of::<(&'static str, &'static u8)>()) as u64, // s((&str,&T))
        (size_of::<(&'static str, &'static Value)>()) as u64, // s((&str,&Value))
        (size_of::<(&'static str, open_pipe_stress_stress_recovery::StressRecoveryResult)>()) as u64, // s((&str,StressRecoveryResult))
        (size_of::<(&'static str, usize)>()) as u64, // s((&str,usize))
        (size_of::<(&'static str, usize, u64)>()) as u64, // s((&str,usize,u64))
        (64) as u64, // s((Content,Content)) (serde 1.0.228 private::de::Content: its largest variants (String, ByteBuf, Seq, Map) hold one 24-byte owner, plus the tag, <= 32; a pair <= 64 (G6 witness))
        (fkr::EXACT_WIDE_SUM_PAIR) as u64, // s((EWS,EWS,f64)) (kernel export structural::retained_resource)
        (fkr::QUANTITY_ID_OUTCOME) as u64, // s((QuantityId,Binary64Outcome)) (kernel export structural::retained_resource)
        (fkr::QUANTITY_ID_U64) as u64, // s((QuantityId,u64)) (kernel export structural::retained_resource)
        (size_of::<(String, crate::DerivedSection)>()) as u64, // s((String,DerivedSection))
        (size_of::<(String, crate::ResultItem)>()) as u64, // s((String,ResultItem))
        (size_of::<(String, String)>()) as u64, // s((String,String))
        (size_of::<(String, [f64; 3])>()) as u64, // s((String,[f64;3]))
        (size_of::<(String, [f64; 6])>()) as u64, // s((String,[f64;6]))
        (size_of::<(String, f64, f64)>()) as u64, // s((String,f64,f64))
        (size_of::<(String, usize)>()) as u64, // s((String,usize))
        (fkr::WIDE_TRIPLE_16) as u64, // s((Wide,Wide,Wide)) (kernel export structural::retained_resource)
        (fkr::FLAGGED_WIDE_16) as u64, // s((bool,f64,Wide)) (kernel export structural::retained_resource)
        (size_of::<(f64, f64)>()) as u64, // s((f64,f64))
        (fkr::U32_KIND_U64) as u64, // s((u32,Kind,u64)) (kernel export structural::retained_resource)
        (fkr::U32_SPRING_KIND) as u64, // s((u32,SpringKind)) (kernel export structural::retained_resource)
        (size_of::<(usize, (f64, f64))>()) as u64, // s((usize,(f64,f64)))
        (fkr::INDEXED_ACCUMULATOR) as u64, // s((usize,ExactAccumulator,bool)) (kernel export structural::retained_resource)
        (fkr::INDEXED_LEDGER_NET) as u64, // s((usize,LedgerNet)) (kernel export structural::retained_resource)
        (fkr::INDEXED_PUBLISHED_VALUE) as u64, // s((usize,PublishedValue)) (kernel export structural::retained_resource)
        (size_of::<(usize, Vec<(f64, f64)>)>()) as u64, // s((usize,VecPair)) (CasePrep.prescribed: Vec<(usize, Vec<(f64, f64)>)> (adaptive.rs:926))
        (size_of::<(usize, [f64; 3], usize, [f64; 3])>()) as u64, // s((usize,[f64;3],usize,[f64;3]))
        (size_of::<(usize, f64)>()) as u64, // s((usize,f64))
        (size_of::<(usize, f64, f64)>()) as u64, // s((usize,f64,f64))
        (size_of::<(usize, usize)>()) as u64, // s((usize,usize))
        (size_of::<(usize, usize, open_pipe_stress_frame_kernel::Matrix12)>()) as u64, // s((usize,usize,Matrix12))
        (size_of::<(usize, usize, bool)>()) as u64, // s((usize,usize,bool))
        (size_of::<crate::retained_receipt::PrivateAdapterSnapshot>()) as u64, // s(AdapterSnapshot) (PreparedTrace.adapter: PrivateAdapterSnapshot)
        (fkr::AFFINE_TERM) as u64, // s(AffineTerm) (kernel export structural::retained_resource)
        (size_of::<crate::LocatedQuantity>()) as u64, // s(AliasE) (the two alias LocatedQuantity values (their strings are B(P_final*RID)))
        (size_of::<open_pipe_stress_stress_recovery::AnalysisStatus>()) as u64, // s(AnalysisStatus)
        (2 * size_of::<usize>() + fkr::CASE_PREP) as u64, // s(ArcCasePrep) (Arc<CasePrep>'s allocation: two counters and the value)
        (2 * size_of::<usize>() + fkr::GROUP_PREP) as u64, // s(ArcGroupPrep) (Arc<GroupPrep>'s allocation: two counters and the value)
        (2 * size_of::<usize>() + fkr::PRODUCT_OWNER_STAMP) as u64, // s(ArcPrepared) (Arc<ProofAnchor{owner: ProductOwnerStamp}>: two counters and the value)
        (fkr::ATTEMPT_RECORD) as u64, // s(AttemptRecord) (kernel export structural::retained_resource)
        (size_of::<crate::Authored<Vec<crate::ExpansionLawInput>>>()) as u64, // s(Authored<Vec<ExpansionLawInput>>)
        (size_of::<crate::retained_product::BasisRecord>()) as u64, // s(BasisRecord)
        (fkr::BINARY64_OUTCOME) as u64, // s(Binary64Outcome) (kernel export structural::retained_resource)
        (fkr::BLOCK_BOUND_16) as u64, // s(BlockBound<16>) (kernel export structural::retained_resource)
        (fkr::BLOCK_BOUND_4) as u64, // s(BlockBound<4>) (kernel export structural::retained_resource)
        (fkr::BLOCK_BOUND_8) as u64, // s(BlockBound<8>) (kernel export structural::retained_resource)
        (fkr::BLOCK_CERTIFICATE_16) as u64, // s(BlockCertificate<16>) (kernel export structural::retained_resource)
        (fkr::BLOCK_CERTIFICATE_4) as u64, // s(BlockCertificate<4>) (kernel export structural::retained_resource)
        (fkr::BLOCK_CERTIFICATE_8) as u64, // s(BlockCertificate<8>) (kernel export structural::retained_resource)
        (fkr::BLOCK_NORMS_16) as u64, // s(BlockNorms<16>) (kernel export structural::retained_resource)
        (fkr::BLOCK_NORMS_4) as u64, // s(BlockNorms<4>) (kernel export structural::retained_resource)
        (fkr::BLOCK_NORMS_8) as u64, // s(BlockNorms<8>) (kernel export structural::retained_resource)
        (fkr::BLOCK_REFUSAL) as u64, // s(BlockRefusal) (kernel export structural::retained_resource)
        (fkr::BLOCK_WITNESS) as u64, // s(BlockWitness) (kernel export structural::retained_resource)
        (2 * fkr::PRODUCT_SUMMARY_COVERAGE + size_of::<[f64; 4]>()) as u64, // s(BodyCoverage) (per body: the kernel's and PP's ProductSummaryCoverage and row_scales' scales [f64; 4])
        (fkr::BODY_GEOMETRY) as u64, // s(BodyGeometry) (kernel export structural::retained_resource)
        (2 * size_of::<Vec<u32>>() + size_of::<u32>()) as u64, // s(BodyMapE) (C2 body map entry (two u32 child lists and an id): no distinct owner at this basis (source.body_of_node is in PrimitiveSource; body_nodes() temporaries are covered by C_u32(n,b)); the entry upper is kept)
        (fkr::BODY_REPORT_16) as u64, // s(BodyReport<16>) (kernel export structural::retained_resource)
        (fkr::BODY_REPORT_4) as u64, // s(BodyReport<4>) (kernel export structural::retained_resource)
        (fkr::BODY_REPORT_8) as u64, // s(BodyReport<8>) (kernel export structural::retained_resource)
        (fkr::BOUNDED_COEFFICIENTS_16) as u64, // s(BoundedCoefficients<16>) (kernel export structural::retained_resource)
        (fkr::BOUNDED_COEFFICIENTS_4) as u64, // s(BoundedCoefficients<4>) (kernel export structural::retained_resource)
        (fkr::BOUNDED_COEFFICIENTS_8) as u64, // s(BoundedCoefficients<8>) (kernel export structural::retained_resource)
        (size_of::<crate::retained_product::CaptureError>()) as u64, // s(CaptureError)
        (max_usize(fkr::SOURCE_CONSTRAINT, size_of::<(String, usize)>())) as u64, // s(ConstraintMapE) (C2 constraint map: the kernel Constraint or a support identity entry, the larger)
        (fkr::CONTRIBUTION) as u64, // s(ContributionE) (assemble.rs: Structure.items Vec<Contribution>)
        (fkr::CONTRIBUTION_ROUNDING) as u64, // s(ContributionRounding) (kernel export structural::retained_resource)
        (fkr::INDEXED_OUTCOME) as u64, // s(ConversionE) (the projection conversion record (usize, Binary64Outcome): no second owner; a double count)
        (fkr::OPTION_PREPARATION_CONVERSION) as u64, // s(ConversionEvent) (the 9 inline conversion outcomes per member (inside SectionPreparationWork): a double count)
        (size_of::<[f64; 3]>()) as u64, // s(CoverageFact) (coverage_facts' coordinates Vec<[f64; 3]> (<= n))
        (up(size_of::<crate::ResultItem>(), max_usize(max_usize(align_of::<crate::ResultItem>(), align_of::<&'static str>()), align_of::<Vec<String>>())) + up(size_of::<&'static str>(), max_usize(max_usize(align_of::<crate::ResultItem>(), align_of::<&'static str>()), align_of::<Vec<String>>())) + up(size_of::<Vec<String>>(), max_usize(max_usize(align_of::<crate::ResultItem>(), align_of::<&'static str>()), align_of::<Vec<String>>()))) as u64, // s(Derived) (source_receipt/rows.rs:324 Derived {row: ResultItem, recipe: &str, inputs: Vec<String>}: field-sum upper)
        (size_of::<crate::Diagnostic>()) as u64, // s(Diagnostic)
        (2 * fkr::WIDE_16) as u64, // s(EnclosureE) (Enclosure {lo, hi: Wide<16>} (product_certificate.rs:16): LaneReadouts.rows)
        (fkr::EXACT_ACCUMULATOR) as u64, // s(ExactAccumulator) (kernel export structural::retained_resource)
        (fkr::EXACT_WIDE_SUM) as u64, // s(ExactWideSum) (kernel export structural::retained_resource)
        (fkr::EXPANSION) as u64, // s(Expansion) (kernel export structural::retained_resource)
        (fkr::EXPANSION_6) as u64, // s(Expansion6) (rigid_body.rs:210: [Expansion; 6] per node)
        (fkr::INDEXED_OUTCOME) as u64, // s(FinalRowConversion) (the final-row conversion record (usize, Binary64Outcome): a double count of projection_outcomes)
        (size_of::<crate::source_receipt::FinalizedSourceBlockCase>()) as u64, // s(FinalizedSourceBlockCase)
        (fkr::FORCE_CONTRIBUTION) as u64, // s(ForceContribution) (kernel export structural::retained_resource)
        (fkr::FORCE_TERM) as u64, // s(ForceTerm) (kernel export structural::retained_resource)
        (size_of::<open_pipe_stress_frame_kernel::FrameElement>()) as u64, // s(FrameElement)
        (size_of::<open_pipe_stress_frame_kernel::FrameNode>()) as u64, // s(FrameNode)
        (fkr::FUNCTIONAL_DESCRIPTOR) as u64, // s(FunctionalDescriptor) (kernel export structural::retained_resource)
        (2 * fkr::WIDE_16) as u64, // s(IntervalE) (run_case's k: Vec<Enclosure> (Q); the second Q term is slack)
        (fkr::PRODUCT_CERTIFICATE_SPENT) as u64, // s(LaneTerminal) (the lanes' terminal work (Option<ResidualWork> x2 inline in ProductCertificateSpent): the whole record, per lane)
        (size_of::<&'static open_pipe_stress_frame_kernel::structural::retained_api::StraightMember>() + 3 * size_of::<f64>() + 9 * size_of::<f64>() + size_of::<u64>()) as u64, // s(LawE) (bridge::ProposedMemberLaw {&StraightMember, D, t, MaterialOperands (<= 9 f64 + tag), z} by field sum)
        (fkr::LAZY_ROW) as u64, // s(LazyE) (BoundedExtremeTracker.lazy: (ExactWideSum, ExactWideSum, u64, u64))
        (size_of::<open_pipe_stress_linear_supports::LinearSupport>()) as u64, // s(LinearSupport)
        (fkr::LOAD_FIDELITY_ROW) as u64, // s(LoadFidelityRow) (kernel export structural::retained_resource)
        (max_usize(size_of::<crate::retained_product::MaterialSelection>(), size_of::<(String, f64, f64)>())) as u64, // s(MaterialDescriptor) (ProductCapture.selections / materials, the larger)
        (size_of::<crate::MaterialInput>()) as u64, // s(MaterialInput)
        (size_of::<crate::retained_product::MaterialSelection>()) as u64, // s(MaterialSelection)
        (size_of::<(usize, u32)>() + fkr::PRODUCT_MAXIMUM_VALUE + 2 * up(8, max_usize(8, align_of::<serde_json::Number>())) + up(4, max_usize(8, align_of::<serde_json::Number>())) + up(8 * size_of::<serde_json::Number>(), max_usize(8, align_of::<serde_json::Number>())) + size_of::<bool>()) as u64, // s(MaximumE) (per member: the kernel maxima slot (usize, u32), PP's ProductMaximumValue output, PP's PreparedMaximumPatch {usize, usize, u32, [Number; 8]} (field sum) and the completion flag)
        (size_of::<crate::MechanicsEnvelope>()) as u64, // s(MechanicsEnvelope)
        (size_of::<crate::retained_product::MemberIdentity>()) as u64, // s(MemberIdentity)
        (size_of::<crate::retained_product::MemberIdentity>()) as u64, // s(MemberMapE) (C2 member map = ProductCapture.members (MemberIdentity); also in T11.3)
        (fkr::MEMBER_OPERATORS_16) as u64, // s(MemberOperators<16>) (kernel export structural::retained_resource)
        (fkr::MEMBER_OPERATORS_4) as u64, // s(MemberOperators<4>) (kernel export structural::retained_resource)
        (fkr::MEMBER_OPERATORS_8) as u64, // s(MemberOperators<8>) (kernel export structural::retained_resource)
        (size_of::<crate::preview_physics::MemberRecord>()) as u64, // s(MemberRecord)
        (size_of::<crate::source_recovery::MemberRecovery>()) as u64, // s(MemberRecovery)
        (size_of::<open_pipe_stress_primitive_loads::NodalLoadContribution>()) as u64, // s(NodalLoadContribution)
        (size_of::<crate::retained_product::TermIdentity>()) as u64, // s(NodalMapE) (C2 nodal-load map = ProductCapture.terms (TermIdentity); also in T11.3)
        (size_of::<(String, [f64; 3])>()) as u64, // s(NodeMapE) (C2 node map = ProductCapture.nodes (String, [f64; 3]); also priced in T11.1)
        (open_pipe_stress_stress_recovery::elastic_extrema::NODE_STRIDE) as u64, // s(Node_SR) (SR export elastic_extrema::NODE_STRIDE)
        (size_of::<crate::retained_product::OperationalSpent>()) as u64, // s(OperationalSpent)
        (fkr::OPTION_BOUND_REFUSAL) as u64, // s(Option<BoundRefusal>) (kernel export structural::retained_resource)
        (up(fkr::FORMATION, max_usize(fkr::FORMATION_ALIGN, align_of::<f64>())) + up(size_of::<f64>(), max_usize(fkr::FORMATION_ALIGN, align_of::<f64>())) + up(size_of::<bool>(), max_usize(fkr::FORMATION_ALIGN, align_of::<f64>())) + up(size_of::<bool>(), max_usize(fkr::FORMATION_ALIGN, align_of::<f64>()))) as u64, // s(Option<FormationRecord>) (FK load_ledger.rs:101 FormationRecord {formation: Formation, operand_bound: f64, self_equilibrated: bool} (private): field-sum upper at the exported Formation layout, plus one aligned slot for the Option tag)
        (fkr::OPTION_WIDE_16) as u64, // s(Option<Wide<16>>) (kernel export structural::retained_resource)
        (fkr::OPTION_WIDE_4) as u64, // s(Option<Wide<4>>) (kernel export structural::retained_resource)
        (fkr::OPTION_WIDE_8) as u64, // s(Option<Wide<8>>) (kernel export structural::retained_resource)
        (size_of::<Option<[f64; 3]>>()) as u64, // s(Option<[f64;3]>)
        (size_of::<Option<f64>>()) as u64, // s(Option<f64>)
        (size_of::<Option<i32>>()) as u64, // s(Option<i32>)
        (size_of::<Option<usize>>()) as u64, // s(Option<usize>)
        (size_of::<crate::retained_product::OrdinarySeed>()) as u64, // s(OrdinarySeed)
        (size_of::<(f64, f64)>()) as u64, // s(Pair) (the prescribed pairs (f64, f64) (adaptive.rs:926))
        (fkr::PIVOT_EVIDENCE) as u64, // s(PivotEvidence) (kernel export structural::retained_resource)
        (fkr::PIVOT_SCREEN_16) as u64, // s(PivotScreen<16>) (kernel export structural::retained_resource)
        (fkr::PIVOT_SCREEN_4) as u64, // s(PivotScreen<4>) (kernel export structural::retained_resource)
        (fkr::PIVOT_SCREEN_8) as u64, // s(PivotScreen<8>) (kernel export structural::retained_resource)
        (fkr::PRECISION_STATE) as u64, // s(PrecisionState) (kernel export structural::retained_resource)
        (size_of::<crate::retained_receipt::PreparedAttemptView<'static>>()) as u64, // s(PreparedAttemptView)
        (size_of::<crate::retained_receipt::PreparationEntry>() + size_of::<crate::retained_product::PreparedAssociation>() + fkr::PREPARED_ANNULUS + fkr::SECTION_PREPARATION_WORK) as u64, // s(PreparedMemberEvent) (per member: trace.members PreparationEntry, associations, preparations PreparedAnnulus, preparation_work SectionPreparationWork)
        (size_of::<crate::PreviewLoadCase>()) as u64, // s(PreviewLoadCase)
        (size_of::<crate::PreviewNode>()) as u64, // s(PreviewNode)
        (size_of::<crate::PreviewPipe>()) as u64, // s(PreviewPipe)
        (size_of::<crate::PreviewSupport>()) as u64, // s(PreviewSupport)
        (size_of::<open_pipe_stress_primitive_loads::PrimitiveLoad>()) as u64, // s(PrimitiveLoad)
        (size_of::<crate::PreviewPrimitiveLoad>()) as u64, // s(PrimitiveLoadInput)
        (fkr::PRODUCT_MEMBER_FACTS) as u64, // s(ProductMemberFacts) (kernel export structural::retained_resource)
        (fkr::PRODUCT_RECIPE) as u64, // s(ProductRecipe) (kernel export structural::retained_resource)
        (fkr::PRODUCT_FINAL_ROW + fkr::PRODUCT_ROW_VERDICT) as u64, // s(ProductRow) (per final row: the descriptor ProductFinalRow and PP's verdict copy)
        (fkr::PRODUCT_ROW_SPEC) as u64, // s(ProductRowSpec) (kernel export structural::retained_resource)
        (fkr::PRODUCT_ROW_VERDICT) as u64, // s(ProductRowVerdict) (kernel export structural::retained_resource)
        (size_of::<f64>() + size_of::<bool>()) as u64, // s(ProductValue) (per final row: the values f64 and (21m <= P_final) derivative_coverage bools)
        (up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<&'static str>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<f64>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<&'static str>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<[f64; 2]>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<f64>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<f64>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<f64>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>())) + up(size_of::<&'static str>(), max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<String>()), align_of::<String>()), align_of::<&'static str>()), align_of::<f64>()), align_of::<String>()), align_of::<&'static str>()), align_of::<[f64; 2]>()), align_of::<f64>()), align_of::<f64>()), align_of::<f64>()), align_of::<&'static str>()))) as u64, // s(Projection) (source_receipt.rs:582 Projection {4 String, 3 &str, 4 f64, [f64; 2]}: field-sum upper)
        (fkr::INDEXED_OUTCOME) as u64, // s(ProjectionOutcome) (ProductCertificateSpent.projection_outcomes: Vec<(usize, Binary64Outcome)>)
        (fkr::PUBLISHED_ROW) as u64, // s(PublishedRow) (kernel export structural::retained_resource)
        (fkr::PUBLISHED_VALUE) as u64, // s(PublishedValue) (kernel export structural::retained_resource)
        (size_of::<open_pipe_stress_stress_recovery::elastic_extrema::QuadraticStressSpan>()) as u64, // s(QuadraticStressSpan)
        (fkr::QUALIFIED_FUNCTIONAL_PROJECTION) as u64, // s(QualifiedFunctionalProjection) (kernel export structural::retained_resource)
        (fkr::QUALIFIED_PROJECTION) as u64, // s(QualifiedProjection) (kernel export structural::retained_resource)
        (fkr::QUANTITY_META) as u64, // s(QuantityMeta) (kernel export structural::retained_resource)
        (fkr::RATIO) as u64, // s(Ratio) (kernel export structural::retained_resource)
        (fkr::RECORD_OUTCOME) as u64, // s(RecordOutcome) (kernel export structural::retained_resource)
        (fkr::RECORDED_CASE) as u64, // s(RecordedCase) (kernel export structural::retained_resource)
        (fkr::RECORDED_INVOCATION) as u64, // s(RecordedInvocation) (kernel export structural::retained_resource)
        (size_of::<crate::formation_guard::RecoveryRecord>()) as u64, // s(RecoveryRecord)
        (fkr::CALL_ORIGIN + fkr::SOURCE_ORIGIN + fkr::GROUP_ORIGIN + fkr::BUILD_ORIGIN + fkr::RUN_ORIGINS + 3 * size_of::<usize>() + fkr::SLOT_SNAPSHOT) as u64, // s(RegistryE) (OriginStore's six registries (origins.rs:290-297), each <= 8 entries for one case: one entry of each, SelectedOrigin {Arc, usize, usize, SlotSnapshot} by field sum)
        (fkr::RESIDUAL_ROW) as u64, // s(ResidualRow) (kernel export structural::retained_resource)
        (size_of::<crate::ResultBasisRef>()) as u64, // s(ResultBasisRef)
        (size_of::<crate::ResultItem>()) as u64, // s(ResultItem)
        (fkr::RETAINED_FUNCTIONAL_PROJECTION) as u64, // s(RetainedFunctionalProjection) (kernel export structural::retained_resource)
        (fkr::RETAINED_PROJECTION) as u64, // s(RetainedProjection) (kernel export structural::retained_resource)
        (fkr::RETAINED_SOLVE) as u64, // s(RetainedSolve) (kernel export structural::retained_resource)
        (size_of::<[f64; 6]>()) as u64, // s(Row6) (rigid_body.rs: Vec<[f64; 6]> rows and motions)
        (fkr::PRODUCT_FINAL_ROW) as u64, // s(RowBinding) (bind_rows' element (retained_product.rs:1742): the kernel's ProductFinalRow, exported by structural::retained_resource)
        (size_of::<open_pipe_stress_result_export::retained_precision::RowClassification>()) as u64, // s(RowClassification)
        (up(size_of::<String>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<&'static str>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Option<String>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Option<&'static str>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>())) + up(size_of::<Vec<String>>(), max_usize(max_usize(max_usize(max_usize(align_of::<String>(), align_of::<&'static str>()), align_of::<Option<String>>()), align_of::<Option<&'static str>>()), align_of::<Vec<String>>()))) as u64, // s(RowTreatment) (source_receipt.rs:597 RowTreatment {result_id: String, treatment: &str, projection_id: Option<String>, recipe_id: Option<&str>, input_result_ids: Vec<String>}: field-sum upper)
        (size_of::<bool>()) as u64, // s(RunRow) (row_scales' native_coverage Vec<bool> (Q))
        (fkr::PRODUCT_MEMBER_FACTS) as u64, // s(SectionMapE) (C2 section map = ProductCapture.facts (ProductMemberFacts); also in T11.3)
        (fkr::SHARED_16_16) as u64, // s(Shared<16>) (kernel export structural::retained_resource)
        (max_usize(fkr::SHARED_4_4, fkr::SHARED_4_8)) as u64, // s(Shared<4>) (the two L = 4 instantiations, Shared<4,4> and Shared<4,8>)
        (fkr::SHARED_8_16) as u64, // s(Shared<8>) (kernel export structural::retained_resource)
        (fkr::SOLVED_16) as u64, // s(Solved<16>) (kernel export structural::retained_resource)
        (fkr::SOLVED_4) as u64, // s(Solved<4>) (kernel export structural::retained_resource)
        (fkr::SOLVED_8) as u64, // s(Solved<8>) (kernel export structural::retained_resource)
        (size_of::<crate::retained_product::SolverObservations>()) as u64, // s(SolverObservations)
        (size_of::<crate::source_recovery::SpringAction>()) as u64, // s(SpringAction)
        (size_of::<open_pipe_stress_linear_supports::SpringEntry>()) as u64, // s(SpringEntry)
        (size_of::<crate::retained_product::SpringIdentity>()) as u64, // s(SpringIdentity)
        (size_of::<crate::retained_product::SpringIdentity>()) as u64, // s(SpringMapE) (C2 spring map = ProductCapture.spring_map (SpringIdentity); also in T11.3)
        (fkr::SOURCE_CONSTRAINT) as u64, // s(SrcConstraint) (kernel export structural::retained_resource)
        (fkr::SOURCE_COORD) as u64, // s(SrcCoord) (kernel export structural::retained_resource)
        (fkr::SOURCE_MEMBER) as u64, // s(SrcMember) (kernel export structural::retained_resource)
        (fkr::SOURCE_NODAL) as u64, // s(SrcNodal) (kernel export structural::retained_resource)
        (fkr::SOURCE_SPRING) as u64, // s(SrcSpring) (kernel export structural::retained_resource)
        (fkr::SOURCE_STATION) as u64, // s(SrcStation) (kernel export structural::retained_resource)
        (fkr::SOURCE_SUPPORT) as u64, // s(SrcSupport) (kernel export structural::retained_resource)
        (fkr::SOURCE_STATION) as u64, // s(StationMapE) (C2 station map: the kernel Station (SourceParts.stations))
        (size_of::<crate::StationResultants>()) as u64, // s(StationResultants)
        (fkr::STIFFNESS_CONTRIBUTION) as u64, // s(StiffnessContribution) (kernel export structural::retained_resource)
        (size_of::<open_pipe_stress_straight_pipe::StraightPipeElement>()) as u64, // s(StraightPipeElement)
        (size_of::<open_pipe_stress_stress_recovery::StressFinding>()) as u64, // s(StressFinding)
        (size_of::<String>()) as u64, // s(String)
        (size_of::<crate::source_recovery::SupportActions>()) as u64, // s(SupportActions)
        (6 * size_of::<bool>()) as u64, // s(SupportCoverage) (row_scales' support_coverage Vec<bool> (6g))
        (size_of::<open_pipe_stress_linear_supports::SupportFinding>()) as u64, // s(SupportFinding)
        (size_of::<(String, usize)>() + size_of::<[bool; 6]>()) as u64, // s(SupportMapE) (C2 support map = ProductCapture.supports (String, usize) and support_fixed [bool; 6]; also in T11.3)
        (size_of::<(String, [f64; 6])>()) as u64, // s(SupportVector) (CaseRecord.support_vectors: Vec<(String, [f64; 6])> (preview_physics.rs:170))
        (size_of::<open_pipe_stress_sparse_direct::SymmetricMatrixEntry>()) as u64, // s(SymmetricMatrixEntry)
        ((up(8, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + (up(8, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + up(fkr::ATTEMPT_STOP, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + up(1, max_usize(8, fkr::ATTEMPT_STOP_ALIGN))))) as u64, // s(TableE) (BoundedExtremeTracker.table: (u64, Evaluated); Evaluated (private) <= tag + seq u64 + AttemptStop by field sum)
        (fkr::TAGGED_CONTRIBUTION) as u64, // s(Tag) (assemble.rs: tagged Vec<(usize, Contribution)>)
        (size_of::<crate::MaterialTemperaturePointInput>()) as u64, // s(TemperaturePoint)
        (size_of::<crate::retained_product::TermIdentity>()) as u64, // s(TermIdentity)
        (size_of::<Option<std::thread::Result<Option<Result<crate::RetainedPreviewOutput, String>>>>>()) as u64, // s(ThreadPacketOutput)
        (fkr::LAZY_ROW + 3 * (up(8, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + (up(8, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + up(fkr::ATTEMPT_STOP, max_usize(8, fkr::ATTEMPT_STOP_ALIGN)) + up(1, max_usize(8, fkr::ATTEMPT_STOP_ALIGN))))) as u64, // s(TrackerE) (one BoundedExtremeTracker over F offers, per offer: a lazy row, a table and a kept-table entry and the stable-sort scratch entry (P3 ENVELOPE Tracker(O) law; the pivot, residual and fallback trackers))
        (size_of::<open_pipe_stress_result_export::retained_precision::Validation>()) as u64, // s(Validation)
        (size_of::<Value>()) as u64, // s(Value)
        (size_of::<Vec<u8>>()) as u64, // s(Vec)
        (size_of::<Vec<u8>>()) as u64, // s(Vec<&ForceTerm>)
        (size_of::<Vec<u8>>()) as u64, // s(Vec<Expansion>)
        (size_of::<Vec<u8>>()) as u64, // s(Vec<Wide>)
        (size_of::<Vec<f64>>()) as u64, // s(Vec<f64>)
        (size_of::<Vec<usize>>()) as u64, // s(Vec<usize>)
        (fkr::VERIFICATION_REPORT_16) as u64, // s(VerificationReport<16>) (kernel export structural::retained_resource)
        (fkr::VERIFICATION_REPORT_4) as u64, // s(VerificationReport<4>) (kernel export structural::retained_resource)
        (fkr::VERIFICATION_REPORT_8) as u64, // s(VerificationReport<8>) (kernel export structural::retained_resource)
        (fkr::VERIFY_SHARED_16_16) as u64, // s(VerifyShared<16>) (kernel export structural::retained_resource)
        (fkr::VERIFY_SHARED_4_8) as u64, // s(VerifyShared<4>) (kernel export structural::retained_resource)
        (fkr::VERIFY_SHARED_8_16) as u64, // s(VerifyShared<8>) (kernel export structural::retained_resource)
        (fkr::WIDE_16) as u64, // s(Wide<16>) (kernel export structural::retained_resource)
        (fkr::WIDE_4) as u64, // s(Wide<4>) (kernel export structural::retained_resource)
        (fkr::WIDE_8) as u64, // s(Wide<8>) (kernel export structural::retained_resource)
        (fkr::PUBLISHED_VALUES_12) as u64, // s([PublishedValue;12]) (kernel export structural::retained_resource)
        (size_of::<[bool; 6]>()) as u64, // s([bool;6])
        (size_of::<f64>()) as u64, // s(f64)
        (size_of::<u32>()) as u64, // s(u32)
        (size_of::<u64>()) as u64, // s(u64)
        (size_of::<usize>()) as u64, // s(usize)
    ];
    /// G3/G4's illustrative values, for the transcription check against the Python chain.
    #[cfg(test)]
    pub(crate) const ATOM_ASSUMED: [u64; ATOMS] = [
        200,
        376,
        288,
        600,
        376,
        640,
        3632,
        1520,
        736,
        512,
        4096,
        200,
        3456,
        464,
        16384,
        16384,
        16426,
        10466306,
        11474,
        368,
        8,
        16,
        24,
        32,
        216,
        24,
        32,
        64,
        2216,
        32,
        16,
        120,
        320,
        48,
        48,
        72,
        40,
        32,
        432,
        160,
        16,
        16,
        8,
        24,
        1120,
        40,
        56,
        32,
        64,
        16,
        24,
        16,
        1168,
        24,
        512,
        32,
        32,
        24,
        1024,
        1024,
        512,
        256,
        32,
        96,
        24,
        576,
        192,
        320,
        576,
        192,
        320,
        288,
        96,
        160,
        48,
        120,
        32,
        160,
        56,
        1152,
        384,
        640,
        20736,
        6912,
        11520,
        48,
        40,
        40,
        96,
        48,
        64,
        32,
        360,
        152,
        304,
        1104,
        1104,
        32,
        144,
        64,
        1200,
        40,
        48,
        200,
        40,
        128,
        32,
        256,
        64,
        2232,
        120,
        160,
        96,
        224,
        64,
        128,
        640,
        120,
        64,
        20736,
        6912,
        11520,
        200,
        696,
        64,
        48,
        40,
        64,
        96,
        56,
        64,
        152,
        56,
        88,
        32,
        16,
        8,
        16,
        400,
        16,
        48,
        448,
        160,
        256,
        64,
        512,
        256,
        512,
        96,
        384,
        400,
        120,
        256,
        512,
        16,
        96,
        96,
        48,
        32,
        80,
        64,
        64,
        48,
        64,
        96,
        96,
        32,
        64,
        48,
        1024,
        1024,
        152,
        128,
        120,
        48,
        296,
        72,
        72,
        2048,
        48,
        64,
        128,
        104,
        32,
        64,
        400,
        400,
        400,
        120,
        120,
        120,
        96,
        48,
        80,
        64,
        48,
        16,
        24,
        96,
        48,
        24,
        24,
        72,
        32,
        104,
        24,
        256,
        80,
        24,
        128,
        64,
        80,
        64,
        80,
        24,
        48,
        8,
        208,
        64,
        2048,
        64,
        96,
        32,
        24,
        24,
        24,
        24,
        24,
        24,
        400,
        400,
        400,
        240,
        240,
        240,
        144,
        48,
        80,
        576,
        6,
        8,
        4,
        8,
        8,
    ];
    /// One term: a constant plus coefficient x atom pairs.
    pub(crate) struct Form {
        pub(crate) name: &'static str,
        pub(crate) constant: u64,
        pub(crate) terms: &'static [(usize, u64)],
    }
    pub(crate) const FORMS: [Form; 47] = [
        Form { name: "BODY", constant: 27280370, terms: &[(8, 30639), (222, 21865)] },
        Form { name: "HELPER_moving", constant: 8388608, terms: &[] },
        Form { name: "INVOC", constant: 262272, terms: &[(8, 19662), (222, 32768)] },
        Form { name: "NOTICE", constant: 718, terms: &[(93, 1), (208, 1)] },
        Form { name: "NOTICE_moving", constant: 0, terms: &[(93, 9361)] },
        Form { name: "O_base_dense", constant: 55367725, terms: &[(0, 1891), (5, 2115), (6, 2403), (8, 45463), (14, 8), (15, 98), (16, 5), (17, 1), (18, 8302), (19, 5), (21, 640), (22, 464), (24, 3), (25, 256), (26, 32), (31, 64), (32, 512), (33, 8192), (35, 64), (37, 192), (40, 28032), (43, 64), (46, 192), (48, 96), (49, 992), (51, 224), (52, 96), (53, 128), (55, 49552), (57, 49), (62, 4), (75, 448), (88, 47776), (93, 16384), (95, 576), (97, 480192), (101, 384), (102, 1024), (103, 512), (104, 32), (105, 6608), (110, 64), (111, 2048), (113, 16), (122, 32), (123, 32), (124, 256), (127, 262144), (130, 640), (134, 224), (135, 384), (136, 576), (137, 1344), (140, 1728), (147, 1), (148, 32), (149, 32), (150, 32), (151, 256), (152, 128), (162, 448), (163, 4), (164, 2048), (165, 512), (167, 4288), (168, 5248), (171, 32), (173, 2560), (174, 1), (175, 10191), (176, 1760), (177, 384), (182, 1891), (192, 32), (193, 32), (204, 4), (205, 42048), (206, 32), (207, 24), (208, 7815), (209, 32), (211, 256), (213, 32), (214, 8192), (217, 256), (222, 70102), (223, 52874), (224, 576), (225, 2304), (227, 576), (228, 2656), (238, 64), (243, 2558)] },
        Form { name: "O_base_sparse", constant: 48247885, terms: &[(0, 1891), (5, 2115), (6, 2403), (8, 45463), (14, 8), (15, 98), (16, 5), (17, 1), (18, 8302), (19, 5), (21, 640), (22, 464), (24, 3), (25, 256), (26, 32), (31, 64), (32, 512), (33, 8192), (35, 64), (37, 192), (40, 28032), (43, 64), (46, 192), (48, 96), (49, 992), (50, 1152), (51, 1376), (52, 96), (53, 128), (55, 49552), (57, 49), (62, 4), (75, 448), (88, 47776), (93, 16384), (95, 576), (97, 93504), (101, 384), (102, 1024), (103, 512), (104, 32), (105, 6608), (110, 64), (111, 2048), (113, 16), (122, 32), (123, 32), (124, 256), (127, 262144), (130, 640), (134, 224), (135, 384), (136, 576), (137, 1152), (140, 1728), (147, 1), (148, 32), (149, 32), (150, 32), (151, 256), (152, 128), (162, 448), (163, 4), (164, 2048), (165, 512), (167, 4288), (168, 5248), (171, 32), (173, 2560), (174, 1), (175, 10191), (176, 1760), (177, 384), (182, 1891), (192, 32), (193, 32), (204, 4), (205, 42048), (206, 32), (207, 24), (208, 7815), (209, 32), (211, 256), (213, 32), (214, 4096), (217, 256), (222, 70102), (223, 47114), (224, 576), (227, 1152), (228, 3424), (238, 64), (243, 2556)] },
        Form { name: "STAGED", constant: 93142218, terms: &[(8, 177), (93, 9361), (116, 1), (175, 2115), (208, 8460), (222, 160)] },
        Form { name: "STATICS", constant: 369376, terms: &[(8, 5801), (222, 23712)] },
        Form { name: "SUCC", constant: 122167538, terms: &[(8, 64583), (222, 79858)] },
        Form { name: "T07_moving", constant: 1797413, terms: &[] },
        Form { name: "T11", constant: 99904, terms: &[(34, 32), (36, 8), (37, 32), (63, 1), (85, 1), (114, 8), (117, 32), (128, 64), (135, 192), (138, 4), (153, 32), (191, 1), (194, 32), (196, 256), (197, 32), (198, 32), (199, 128), (200, 32), (201, 128), (202, 32), (218, 128), (239, 32), (241, 192)] },
        Form { name: "T11_late_capture", constant: 16512, terms: &[(135, 192), (196, 256), (197, 32), (198, 32), (199, 128), (200, 32), (201, 128), (202, 32), (241, 192)] },
        Form { name: "T11_ordinary_seed", constant: 10368, terms: &[(138, 4)] },
        Form { name: "T12", constant: 531730, terms: &[(44, 128), (45, 128), (47, 256), (51, 4096), (58, 1), (59, 1), (77, 32), (78, 32), (86, 256), (87, 4096), (96, 4), (98, 32), (112, 8), (118, 32), (125, 128), (126, 32), (135, 384), (139, 1152), (166, 2048), (172, 8), (179, 672), (184, 32), (195, 32), (196, 704), (197, 128), (198, 112), (199, 384), (200, 112), (201, 352), (202, 112), (203, 128), (212, 32), (216, 4096), (228, 768), (241, 768), (242, 17920), (243, 41919)] },
        Form { name: "T13", constant: 250566, terms: &[(9, 52), (10, 52), (28, 256), (29, 2048), (30, 2048), (38, 512), (39, 256), (41, 128), (42, 64), (61, 5), (64, 2048), (65, 512), (66, 256), (67, 256), (68, 256), (69, 256), (70, 256), (71, 256), (72, 256), (73, 256), (74, 3584), (77, 32), (79, 32), (80, 32), (81, 32), (82, 96), (83, 32), (84, 32), (109, 512), (119, 64), (120, 64), (121, 32), (129, 256), (131, 21248), (132, 10496), (133, 10496), (141, 256), (142, 512), (143, 256), (144, 4), (161, 4096), (166, 2048), (178, 1), (185, 1), (186, 2), (187, 1), (188, 1), (189, 2), (190, 1), (215, 20800), (220, 1792), (226, 1344), (228, 192), (229, 1), (230, 1), (231, 1), (232, 1), (233, 1), (234, 1), (235, 139584), (236, 75584), (237, 50976)] },
        Form { name: "T14", constant: 2183568, terms: &[(56, 4096), (60, 1), (76, 32), (89, 4096), (91, 32), (94, 4096), (106, 4096), (108, 4096), (115, 32), (127, 262144), (155, 4096), (156, 4096), (157, 4096), (158, 8192), (160, 4096), (183, 2048), (210, 32), (235, 4608), (240, 4096)] },
        Form { name: "T15", constant: 4096, terms: &[(54, 1), (90, 512), (99, 4096), (107, 2), (128, 64), (146, 32), (169, 1), (170, 1)] },
        Form { name: "T16_P1", constant: 110858314, terms: &[(8, 71696), (22, 4096), (145, 1), (154, 2115), (180, 2115), (208, 4352), (222, 93306)] },
        Form { name: "T16_P2", constant: 1121806665, terms: &[(8, 175860), (22, 4096), (145, 1), (154, 2115), (180, 2115), (208, 4352), (222, 522480)] },
        Form { name: "T16_P3", constant: 428916421, terms: &[(8, 169252), (22, 4096), (145, 1), (154, 2115), (180, 2115), (208, 4384), (222, 269584)] },
        Form { name: "T16_P4", constant: 184773892, terms: &[(8, 138614), (22, 4096), (145, 1), (154, 2115), (180, 2115), (208, 4096), (222, 138394)] },
        Form { name: "T16_moving", constant: 189303281, terms: &[] },
        Form { name: "T17_V1", constant: 271422963, terms: &[(8, 61278), (208, 288), (222, 153055)] },
        Form { name: "T17_V2_clone", constant: 122167538, terms: &[(8, 64583), (222, 79858)] },
        Form { name: "T17_V2_hash", constant: 1059200230, terms: &[(8, 101829), (208, 256), (222, 463944)] },
        Form { name: "T17_V3", constant: 10771580, terms: &[(8, 28452), (208, 384), (222, 27392)] },
        Form { name: "T17_V4", constant: 6920061, terms: &[(2, 4189), (3, 39), (8, 8037), (20, 83625), (21, 1024), (23, 12288), (181, 4096), (208, 51309), (222, 8460), (223, 8192)] },
        Form { name: "T17_V5", constant: 124603799, terms: &[(2, 4189), (3, 39), (8, 66698), (20, 57880), (21, 1024), (23, 12288), (181, 4096), (208, 51309), (222, 79858), (223, 8192)] },
        Form { name: "T17_V6", constant: 106446543, terms: &[(2, 4189), (3, 39), (8, 109536), (20, 57880), (21, 1024), (23, 12288), (181, 4096), (208, 51853), (222, 295945), (223, 8192)] },
        Form { name: "T17_moving_invocation", constant: 1179864, terms: &[] },
        Form { name: "T17_moving_publication", constant: 189303281, terms: &[] },
        Form { name: "T17_output", constant: 0, terms: &[(181, 4096), (221, 1)] },
        Form { name: "T19", constant: 8192, terms: &[(219, 1)] },
        Form { name: "T25_I1", constant: 1157082476, terms: &[(2, 2296), (4, 424), (8, 163110), (27, 16384), (62, 4), (113, 8), (147, 1), (148, 32), (149, 32), (150, 32), (152, 128), (175, 2115), (208, 2499), (217, 128), (222, 544711)] },
        Form { name: "T25_I2", constant: 428699838, terms: &[(2, 2296), (4, 424), (8, 112938), (27, 16384), (62, 4), (113, 8), (147, 1), (148, 32), (149, 32), (150, 32), (152, 128), (175, 2115), (208, 2563), (217, 128), (222, 184736)] },
        Form { name: "T25_I3", constant: 217461894, terms: &[(2, 2296), (4, 424), (8, 104507), (27, 16384), (62, 4), (113, 8), (147, 1), (148, 32), (149, 32), (150, 32), (152, 128), (175, 2115), (208, 2307), (217, 128), (222, 145340)] },
        Form { name: "T25_S1", constant: 20350012, terms: &[(8, 39320), (21, 384), (22, 400), (25, 192), (26, 32), (27, 16384), (31, 64), (37, 64), (49, 256), (53, 32), (55, 16784), (62, 4), (75, 256), (97, 37248), (101, 256), (102, 256), (103, 64), (104, 32), (105, 3088), (110, 64), (113, 12), (124, 128), (130, 256), (134, 32), (135, 192), (137, 192), (147, 1), (148, 32), (149, 32), (150, 32), (151, 256), (152, 128), (167, 2144), (193, 32), (205, 12832), (206, 32), (208, 2272), (211, 256), (217, 192), (222, 65536), (223, 14154), (228, 640), (243, 256)] },
        Form { name: "T25_S2", constant: 5594299, terms: &[(21, 384), (22, 320), (25, 128), (26, 32), (37, 64), (49, 64), (53, 32), (55, 16784), (101, 128), (102, 128), (103, 32), (105, 3088), (130, 128), (134, 32), (135, 192), (137, 192), (205, 8192), (208, 2080), (223, 13386), (228, 192)] },
        Form { name: "T25_S3", constant: 78439670, terms: &[(1, 424), (4, 353), (7, 424), (8, 2034), (12, 353), (13, 353), (92, 2115), (159, 2048), (182, 2115), (222, 1152)] },
        Form { name: "T25_S4", constant: 367697553, terms: &[(8, 384010), (11, 78), (159, 2048), (182, 2115), (208, 512), (222, 1051510), (228, 192)] },
        Form { name: "T25_S5", constant: 70217356, terms: &[(8, 13718), (100, 1), (159, 2048), (182, 2115), (222, 9036)] },
        Form { name: "T25_carried_case", constant: 45678294, terms: &[(8, 7369), (159, 2048), (182, 2115), (222, 4806)] },
        Form { name: "T25_moving", constant: 189079500, terms: &[] },
        Form { name: "TAV_W", constant: 1570041862, terms: &[] },
        Form { name: "TAV_X", constant: 1440401002, terms: &[] },
        Form { name: "TXT_moving", constant: 2599962, terms: &[] },
    ];
    pub(crate) const F_BODY: usize = 0;
    pub(crate) const F_HELPER_MOVING: usize = 1;
    pub(crate) const F_INVOC: usize = 2;
    pub(crate) const F_NOTICE: usize = 3;
    pub(crate) const F_NOTICE_MOVING: usize = 4;
    pub(crate) const F_O_BASE_DENSE: usize = 5;
    pub(crate) const F_O_BASE_SPARSE: usize = 6;
    pub(crate) const F_STAGED: usize = 7;
    pub(crate) const F_STATICS: usize = 8;
    pub(crate) const F_SUCC: usize = 9;
    pub(crate) const F_T07_MOVING: usize = 10;
    pub(crate) const F_T11: usize = 11;
    pub(crate) const F_T11_LATE_CAPTURE: usize = 12;
    pub(crate) const F_T11_ORDINARY_SEED: usize = 13;
    pub(crate) const F_T12: usize = 14;
    pub(crate) const F_T13: usize = 15;
    pub(crate) const F_T14: usize = 16;
    pub(crate) const F_T15: usize = 17;
    pub(crate) const F_T16_P1: usize = 18;
    pub(crate) const F_T16_P2: usize = 19;
    pub(crate) const F_T16_P3: usize = 20;
    pub(crate) const F_T16_P4: usize = 21;
    pub(crate) const F_T16_MOVING: usize = 22;
    pub(crate) const F_T17_V1: usize = 23;
    pub(crate) const F_T17_V2_CLONE: usize = 24;
    pub(crate) const F_T17_V2_HASH: usize = 25;
    pub(crate) const F_T17_V3: usize = 26;
    pub(crate) const F_T17_V4: usize = 27;
    pub(crate) const F_T17_V5: usize = 28;
    pub(crate) const F_T17_V6: usize = 29;
    pub(crate) const F_T17_MOVING_INVOCATION: usize = 30;
    pub(crate) const F_T17_MOVING_PUBLICATION: usize = 31;
    pub(crate) const F_T17_OUTPUT: usize = 32;
    pub(crate) const F_T19: usize = 33;
    pub(crate) const F_T25_I1: usize = 34;
    pub(crate) const F_T25_I2: usize = 35;
    pub(crate) const F_T25_I3: usize = 36;
    pub(crate) const F_T25_S1: usize = 37;
    pub(crate) const F_T25_S2: usize = 38;
    pub(crate) const F_T25_S3: usize = 39;
    pub(crate) const F_T25_S4: usize = 40;
    pub(crate) const F_T25_S5: usize = 41;
    pub(crate) const F_T25_CARRIED_CASE: usize = 42;
    pub(crate) const F_T25_MOVING: usize = 43;
    pub(crate) const F_TAV_W: usize = 44;
    pub(crate) const F_TAV_X: usize = 45;
    pub(crate) const F_TXT_MOVING: usize = 46;
    /// A form's value at the given atom values, in checked arithmetic.
    pub(crate) const fn form(f: &Form, v: &[u64; ATOMS]) -> Option<u64> {
        let mut total = f.constant;
        let mut i = 0;
        while i < f.terms.len() {
            let (a, c) = f.terms[i];
            let Some(x) = v[a].checked_mul(c) else { return None };
            let Some(t) = total.checked_add(x) else { return None };
            total = t;
            i += 1;
        }
        Some(total)
    }
    pub(crate) const fn add(a: Option<u64>, b: Option<u64>) -> Option<u64> {
        match (a, b) {
            (Some(a), Some(b)) => a.checked_add(b),
            _ => None,
        }
    }
    pub(crate) const fn max(a: Option<u64>, b: Option<u64>) -> Option<u64> {
        match (a, b) {
            (Some(a), Some(b)) => Some(if a > b { a } else { b }),
            _ => None,
        }
    }
    /// T12_T15 (sum of its terms, taken in the build).
    pub(crate) const fn t12_t15(v: &[u64; ATOMS]) -> Option<u64> {
        add(add(add(form(&FORMS[14], v), form(&FORMS[15], v)), form(&FORMS[16], v)), form(&FORMS[17], v))
    }
    /// T16 (maximum of its terms, taken in the build).
    pub(crate) const fn t16(v: &[u64; ATOMS]) -> Option<u64> {
        max(max(max(form(&FORMS[18], v), form(&FORMS[19], v)), form(&FORMS[20], v)), form(&FORMS[21], v))
    }
    /// T17 (sum of its terms, taken in the build).
    pub(crate) const fn t17(v: &[u64; ATOMS]) -> Option<u64> {
        add(max(max(max(max(max(max(form(&FORMS[23], v), form(&FORMS[24], v)), form(&FORMS[25], v)), form(&FORMS[26], v)), form(&FORMS[27], v)), form(&FORMS[28], v)), form(&FORMS[29], v)), form(&FORMS[32], v))
    }
    /// T25 (sum of its terms, taken in the build).
    pub(crate) const fn t25(v: &[u64; ATOMS]) -> Option<u64> {
        add(form(&FORMS[42], v), max(max(max(max(max(max(max(form(&FORMS[37], v), form(&FORMS[38], v)), form(&FORMS[39], v)), form(&FORMS[40], v)), form(&FORMS[41], v)), form(&FORMS[34], v)), form(&FORMS[35], v)), form(&FORMS[36], v)))
    }
    pub(crate) const PHASES: usize = 7;
    pub(crate) const PHASE_NAMES: [&str; PHASES] = [
        "W1 ordinary span (W1 not yet run)",
        "W2 G-B, G-C and the W1 phases (T12-T15) with the N1 reserve",
        "W3 publication (T16) with the staged copy",
        "W4 precommit validation (T17) with the successor and the invocation Value",
        "W5 transfer and Direct completion (T18, T19)",
        "X1 ordinary span with T25 (selected finalization)",
        "X2 X completion: retained receipt + reserve + Direct completion",
    ];
    /// The phases for sparse mode: requested bytes, and the largest moving extra (E_mov, without R).
    pub(crate) const fn phases_sparse(v: &[u64; ATOMS]) -> [(Option<u64>, Option<u64>); PHASES] {
        [
            (add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[10], v))), // W1
            (add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[4], v))), // W2
            (add(add(add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[7], v)), t16(v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[22], v))), // W3
            (add(add(add(add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), form(&FORMS[2], v)), t17(v)), max(max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[31], v)), form(&FORMS[30], v))), // W4
            (add(add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), max(form(&FORMS[46], v), form(&FORMS[1], v))), // W5
            (add(add(add(add(add(form(&FORMS[6], v), t25(v)), form(&FORMS[45], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[43], v), form(&FORMS[46], v)), form(&FORMS[1], v))), // X1
            (add(add(add(add(add(add(form(&FORMS[6], v), form(&FORMS[0], v)), form(&FORMS[45], v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[4], v))), // X2
        ]
    }
    /// The phases for dense mode: requested bytes, and the largest moving extra (E_mov, without R).
    pub(crate) const fn phases_dense(v: &[u64; ATOMS]) -> [(Option<u64>, Option<u64>); PHASES] {
        [
            (add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[10], v))), // W1
            (add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[4], v))), // W2
            (add(add(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[7], v)), t16(v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[22], v))), // W3
            (add(add(add(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), form(&FORMS[2], v)), t17(v)), max(max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[31], v)), form(&FORMS[30], v))), // W4
            (add(add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[44], v)), t12_t15(v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), form(&FORMS[9], v)), max(form(&FORMS[46], v), form(&FORMS[1], v))), // W5
            (add(add(add(add(add(form(&FORMS[5], v), t25(v)), form(&FORMS[45], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[43], v), form(&FORMS[46], v)), form(&FORMS[1], v))), // X1
            (add(add(add(add(add(add(form(&FORMS[5], v), form(&FORMS[0], v)), form(&FORMS[45], v)), form(&FORMS[3], v)), form(&FORMS[11], v)), form(&FORMS[33], v)), form(&FORMS[8], v)), max(max(form(&FORMS[46], v), form(&FORMS[1], v)), form(&FORMS[4], v))), // X2
        ]
    }
    /// The admission maximum without R: max over the phases of requested + moving.
    pub(crate) const fn maximum(phases: &[(Option<u64>, Option<u64>); PHASES]) -> Option<(u64, usize)> {
        let mut best: Option<(u64, usize)> = None;
        let mut i = 0;
        while i < PHASES {
            let Some(e) = add(phases[i].0, phases[i].1) else { return None };
            best = match best {
                Some((b, j)) if b >= e => Some((b, j)),
                _ => Some((e, i)),
            };
            i += 1;
        }
        best
    }
    /// The count of Estimate atoms (the profile is complete only when it is zero).
    pub(crate) const ESTIMATES: usize = {
        let mut n = 0;
        let mut i = 0;
        while i < ATOMS {
            if matches!(ATOM_BINDINGS[i], Binding::Estimate) {
                n += 1;
            }
            i += 1;
        }
        n
    };
    pub(crate) const SPARSE: Option<(u64, usize)> = maximum(&phases_sparse(&ATOM_VALUES));
    pub(crate) const DENSE: Option<(u64, usize)> = maximum(&phases_dense(&ATOM_VALUES));
    /// The Python chain's own evaluation (ASSUMED strides), for the transcription check.
    #[cfg(test)]
    pub(crate) const PYTHON_CHECK: [(u64, &str); 2] = [(3437874885, "W3 publication (T16) with the staged copy"), (3457585333, "W3 publication (T16) with the staged copy")];
}
// ---- END GENERATED PROFILE ----

/// S-1: `E_mov,max + R ≤ M`, in checked arithmetic. Pure.
pub(super) fn bound_admits(maximum: u64, reserved_stack: u64, threshold: u64) -> Result<u64, BoundRefusal> {
    let required = maximum.checked_add(reserved_stack).ok_or(BoundRefusal::Overflow)?;
    if required <= threshold {
        Ok(required)
    } else {
        Err(BoundRefusal::Exceeds { required, threshold })
    }
}
/// S-1 at admission (RV89 G6 S-3): the registered profile's bound for the invocation's
/// mode, `E_mov,max(mode) + R ≤ threshold`, with R always the constant reserved stack
/// (never the `cfg(test)` override, which only sizes the witness thread). `admit` prices
/// its bound through this function alone. Pure.
pub(super) fn admission_bound(maximum: Result<u64, BoundRefusal>, threshold: u64) -> Result<u64, BoundRefusal> {
    bound_admits(maximum?, RESERVED_STACK_BYTES as u64, threshold)
}
/// The required bytes a bound verdict states, admitted or exceeded (`None` when it was
/// not computed: Unpriced or Overflow). Pure.
pub(super) fn bound_required(verdict: &Result<u64, BoundRefusal>) -> Option<u64> {
    match *verdict {
        Ok(required) | Err(BoundRefusal::Exceeds { required, .. }) => Some(required),
        Err(_) => None,
    }
}

/// The admission law's private record in the report (not serialized; the public
/// report fields are unchanged). It keeps every fact and the first failing clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct AdmissionLaw {
    pub(super) raw_text: RawTextFacts,
    pub(super) nested: NestedTypedFacts,
    /// The first failing clause in DOMAIN.md order, then the bound.
    pub(super) refusal: Option<AdmissionRefusal>,
    /// The first failing request-level clause (D1.2–D1.11), whatever the caller
    /// and build: whether this request is inside D1.
    pub(super) domain: Option<AdmissionRefusal>,
    /// The registered profile admission selected; `None` on every refusal.
    registered: Option<usize>,
    /// The bound's required bytes, `E_mov,max(mode) + R`, when admission evaluated it
    /// (RV89 G6 S-3); `None` when an earlier clause refused or the maximum was unpriced.
    pub(super) required: Option<u64>,
}

// ---- G-B and G-C (API_G4.md §1–§2) -----------------------------------------
// Each gate reads borrowed lengths and capacities of the live owners, without
// allocating, and compares each against its cap-priced bound. A fact above its
// bound means the derivation missed something: the gate refuses to the ordinary
// path. The gates cross-check the derivation; they are not the memory bound.

/// G-B facts: the live ordinary owners at the late old-source capture, borrowed,
/// and the observer's own capture so far (RV84 S-6(c)). B1 SA: G-B reads the case's own
/// loads from `case`, and the running total Σ_{i≤k} l_i from the capture's
/// `late_loads_total` (ST's seam, added immediately before G-B at case k).
pub(super) struct LateFacts<'a> {
    /// Borrowed for the gate's site; no G-B fact reads it (API_G4.md §2).
    #[allow(dead_code)]
    pub(super) model: &'a crate::PreviewModel,
    pub(super) built: &'a crate::BuiltModel,
    pub(super) materials: &'a [crate::MaterialInput],
    pub(super) case: &'a crate::PreviewLoadCase,
    pub(super) restrained: &'a [usize],
    pub(super) springs: &'a [crate::SpringEntry],
    pub(super) capture: &'a crate::retained_product::ProductCapture,
}
/// G-C facts: the complete ordinary owner and the observer, including its late
/// capture (RV84 S-6(c)), borrowed.
pub(super) struct CompleteFacts<'a> {
    pub(super) ordinary: &'a crate::MechanicsEnvelope,
    pub(super) capture: &'a crate::retained_product::ProductCapture,
    /// B1 seam (PLAN_v2 §2.1; RV107 SF-4): the typed request's `model.load_cases.len()`,
    /// which `permitted_run` reads before the request moves into the observed run. T-3
    /// (e)'s requested count: `OrdinarySolveNotAttempted` reads it (B1 SA).
    pub(super) requested_cases: usize,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhaseGate {
    Late,
    Complete,
}
/// The facts the gates read (API_G4.md §2), G-B's then G-C's, in table order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum PhaseFact {
    BuiltNodes,
    BuiltMembers,
    BuiltFrameElements,
    BuiltSupports,
    CaseLoads,
    /// B1 SA: the running total of the requested cases' loads at G-B (Σ_{i≤k} l_i).
    CaseLoadsTotal,
    Restrained,
    Springs,
    Materials,
    LateObservationBytes,
    EnvelopeResults,
    EnvelopeResultCapacity,
    EnvelopeResultTextBytes,
    EnvelopeDiagnostics,
    EnvelopeDiagnosticCapacity,
    EnvelopeDiagnosticTextBytes,
    EnvelopeMaxStringBytes,
    DiagnosticIdMaxBytes,
    ContractEvidenceStatus,
    ContractEvidenceArrayElements,
    ContractEvidenceObjects,
    ContractEvidenceEntries,
    ContractEvidenceStringBytes,
    ContractEvidenceKeyBytes,
    SourceBlockRecovery,
    ObservationBytes,
    OrdinarySeedBytes,
    RetainedErrorTextBytes,
    /// G6 (ROOT, C1:64, ROUTING:98): 1 when the ordinary route returned without
    /// attempting a requested case's solve (B1 SA: T-3 (e), over every requested case);
    /// its bound is 0, so G-C declines W1 there.
    OrdinarySolveNotAttempted,
}
/// A gate's refusal: the gate, the first fact above its bound, the observed
/// value (saturated at `u64::MAX` when a sum overflows) and the bound.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) struct PhaseRefusal {
    pub(super) gate: PhaseGate,
    pub(super) fact: PhaseFact,
    pub(super) observed: u64,
    pub(super) cap: u64,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhaseObservation {
    pub(super) fact: PhaseFact,
    pub(super) observed: u64,
}
pub(super) const LATE_FACTS: usize = 10;
pub(super) const COMPLETE_FACTS: usize = 19;
/// The bound for each fact, in `late_observations`/`complete_observations` order.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhaseCaps {
    pub(super) late: [u64; LATE_FACTS],
    pub(super) complete: [u64; COMPLETE_FACTS],
}
/// A checked sum of capacities, saturated at `u64::MAX` (which exceeds every
/// bound) instead of wrapping.
#[derive(Clone, Copy)]
struct Bytes(Option<usize>);
impl Bytes {
    const ZERO: Self = Self(Some(0));
    fn add(self, value: usize) -> Self {
        Self(self.0.and_then(|sum| sum.checked_add(value)))
    }
    fn times(value: usize, stride: usize) -> Self {
        Self(value.checked_mul(stride))
    }
    fn plus(self, other: Self) -> Self {
        match other.0 {
            Some(value) => self.add(value),
            None => Self(None),
        }
    }
    fn get(self) -> u64 {
        self.0.and_then(|sum| u64::try_from(sum).ok()).unwrap_or(u64::MAX)
    }
}
fn count(value: usize) -> u64 {
    u64::try_from(value).unwrap_or(u64::MAX)
}
fn strings<'a>(texts: impl IntoIterator<Item = &'a String>) -> Bytes {
    texts.into_iter().fold(Bytes::ZERO, |sum, s| sum.add(s.capacity()))
}
fn string_vec(texts: &Vec<String>) -> Bytes {
    Bytes::times(texts.capacity(), std::mem::size_of::<String>()).plus(strings(texts))
}
fn result_text(row: &crate::ResultItem) -> Bytes {
    let mut sum = strings([&row.id, &row.kind, &row.unit, &row.entity_ref]).plus(string_vec(&row.source_result_refs));
    if let Some(b) = &row.basis_ref {
        sum = sum.plus(strings([&b.ref_type, &b.ref_id]));
    }
    if let Some(m) = &row.metadata {
        sum = sum.plus(strings([&m.component, &m.coordinate_system, &m.location, &m.basis, &m.sign_convention]));
    }
    sum
}
/// RV87 N-3: the longest single string a row or diagnostic owns (it sizes the hash
/// route's per-string temporary and parser scratch: L_PUB).
fn longest_string(e: &crate::MechanicsEnvelope) -> usize {
    let row = |r: &crate::ResultItem| {
        let mut n = [&r.id, &r.kind, &r.unit, &r.entity_ref].iter().map(|s| s.len()).max().unwrap_or(0);
        n = r.source_result_refs.iter().map(String::len).fold(n, usize::max);
        if let Some(b) = &r.basis_ref {
            n = n.max(b.ref_type.len()).max(b.ref_id.len());
        }
        if let Some(m) = &r.metadata {
            n = [&m.component, &m.coordinate_system, &m.location, &m.basis, &m.sign_convention].iter().map(|s| s.len()).fold(n, usize::max);
        }
        n
    };
    let diagnostic = |d: &crate::Diagnostic| {
        let n = [&d.id, &d.code, &d.severity, &d.message].iter().map(|s| s.len()).max().unwrap_or(0);
        d.affected_refs.iter().map(String::len).fold(n.max(d.source.as_ref().map_or(0, String::len)), usize::max)
    };
    e.results.iter().map(row).chain(e.diagnostics.iter().map(diagnostic)).max().unwrap_or(0)
}
fn diagnostic_text(d: &crate::Diagnostic) -> Bytes {
    strings([&d.id, &d.code, &d.severity, &d.message]).plus(strings(&d.source)).plus(string_vec(&d.affected_refs))
}
/// The capture's own capacity bytes so far: the adapter's `RustCapacityBytes`
/// tally, which every capture allocation enters (observation, support and
/// prepared arrays and texts). At G-B it excludes the late old-source capture;
/// at G-C it includes it, so the late capture is measured after it is made
/// (RV84 S-6(c)) inside G-C's `ObservationBytes`. A separate T11.P1 fact would
/// need the G-B tally kept by the observer: a retained_product.rs change
/// outside G5's fence.
fn capture_bytes(capture: &crate::retained_product::ProductCapture) -> u64 {
    capture.adapter.counts.get()[crate::retained_product::AdapterEvent::RustCapacityBytes as usize]
}
/// C-N4: the text owned by `error`, `observable_error` and `g5a_error`, over every requested
/// case's slot (B1 SA after SP's T-2; ROOT's note after R3′): the case in the capture's own
/// fields and, at c ≥ 2, the earlier cases' parked slots (`parked_cases()`). Its bound is
/// C·(3m + 1)·Text(err), one retained error set per case (I82's assumption; phase 4 checks it).
fn retained_error_text(capture: &crate::retained_product::ProductCapture) -> Bytes {
    use crate::retained_product::CaptureError;
    let text = |e: &Option<CaptureError>| match e {
        Some(CaptureError::Association(s)) => s.capacity(),
        _ => 0,
    };
    // `G5aFailure` holds only `&'static str` and integer facts.
    let own = Bytes::ZERO.add(text(&capture.error)).add(text(&capture.observable_error));
    capture.parked_cases().iter().fold(own, |sum, slot| sum.add(text(&slot.error)).add(text(&slot.observable_error)))
}
pub(super) fn late_observations(f: &LateFacts<'_>) -> [PhaseObservation; LATE_FACTS] {
    use PhaseFact as P;
    let o = |fact, observed| PhaseObservation { fact, observed };
    [
        o(P::BuiltNodes, count(f.built.nodes.len())),
        o(P::BuiltMembers, count(f.built.pipes.len())),
        o(P::BuiltFrameElements, count(f.built.frame_elements.len())),
        o(P::BuiltSupports, count(f.built.supports.len())),
        o(P::CaseLoads, count(f.case.primitive_loads.len())),
        o(P::CaseLoadsTotal, count(f.capture.late_loads_total)),
        o(P::Restrained, count(f.restrained.len())),
        o(P::Springs, count(f.springs.len())),
        o(P::Materials, count(f.materials.len())),
        o(P::LateObservationBytes, capture_bytes(f.capture)),
    ]
}
pub(super) fn complete_observations(f: &CompleteFacts<'_>) -> [PhaseObservation; COMPLETE_FACTS] {
    use PhaseFact as P;
    let o = |fact, observed| PhaseObservation { fact, observed };
    let e = f.ordinary;
    let evidence = e.contract_evidence.as_ref().map(borrowed_value_census);
    let evidence_text = e.contract_evidence.as_ref().map(raw_text_census);
    let evidence_complete = evidence.map_or(true, |c| c.status == CensusStatus::Complete)
        && evidence_text.map_or(true, |t| t.status == CensusStatus::Complete);
    let fact = |read: fn(&BorrowedValueFacts) -> usize| evidence.map_or(0, |c| count(read(&c)));
    let seeds = f.capture.ordinary.iter().fold(
        Bytes::times(f.capture.ordinary.capacity(), std::mem::size_of::<crate::retained_product::OrdinarySeed>()),
        |sum, seed| sum.add(seed.case.capacity()).plus(strings(&seed.d5_diagnostic_ref)),
    );
    [
        o(P::EnvelopeResults, count(e.results.len())),
        o(P::EnvelopeResultCapacity, count(e.results.capacity())),
        o(P::EnvelopeResultTextBytes, e.results.iter().fold(Bytes::ZERO, |sum, r| sum.plus(result_text(r))).get()),
        o(P::EnvelopeDiagnostics, count(e.diagnostics.len())),
        o(P::EnvelopeDiagnosticCapacity, count(e.diagnostics.capacity())),
        o(P::EnvelopeDiagnosticTextBytes, e.diagnostics.iter().fold(Bytes::ZERO, |sum, d| sum.plus(diagnostic_text(d))).get()),
        o(P::EnvelopeMaxStringBytes, count(longest_string(e).max(evidence_text.map_or(0, |t| t.max_string_bytes.max(t.max_key_bytes))))),
        o(P::DiagnosticIdMaxBytes, count(e.diagnostics.iter().map(|d| d.id.len()).max().unwrap_or(0))),
        o(P::ContractEvidenceStatus, u64::from(!evidence_complete)),
        o(P::ContractEvidenceArrayElements, fact(|c| c.array_capacity_elements)),
        o(P::ContractEvidenceObjects, fact(|c| c.objects)),
        o(P::ContractEvidenceEntries, fact(|c| c.object_entries)),
        o(P::ContractEvidenceStringBytes, fact(|c| c.string_capacity_bytes)),
        o(P::ContractEvidenceKeyBytes, fact(|c| c.key_capacity_bytes)),
        o(P::SourceBlockRecovery, u64::from(e.source_block_recovery.is_some())),
        o(P::ObservationBytes, capture_bytes(f.capture)),
        o(P::OrdinarySeedBytes, seeds.get()),
        o(P::RetainedErrorTextBytes, retained_error_text(f.capture).get()),
        o(P::OrdinarySolveNotAttempted, u64::from(!ordinary_solve_attempted(f.capture, f.requested_cases))),
    ]
}
/// G6 (ROOT's ruling on D1's blocked ordinary runs; C1:64 declines W1 before execution,
/// ROUTING:98 allows a notice only for W1 work that ran): whether the ordinary route
/// attempted the case's solve. The observer's own G-b record decides it, never the
/// envelope's status (`blocked_envelope` is also returned after an attempted solve):
/// `OrdinarySeed::initial` is set exactly at the attempt (`solve_load_case_observed`,
/// lib.rs `solve_load_case_observed`: `ordinary_initial_failure` for a structural failure or a
/// deferred-basis formation refusal, the F1b attempt that routes to W2) or at the
/// published report of a successful solve (`ordinary_report` in lib.rs, the only exit of
/// an `Ok` attempt inside D1, whose nonlinear supports D1.6 excludes). Every return
/// before the attempt (validation's `blocked_envelope`s in
/// `run_linear_static_preview_observed`; the load-input, ledger and reduction exits of
/// `solve_load_case_observed`, :3918, :4011, :4042, :4098) leaves no seed or a seed
/// with `initial` None. B1 SA (T-3 (e); DESIGN_v2 §1.2, RV105 N-1): the fact counts the
/// `requested` cases (`CompleteFacts::requested_cases`): it holds exactly when at least one
/// case is requested, the capture has one seed per requested case, and every seed's
/// `initial` is set. A run that blocks at case k < c − 1 leaves the later cases unseeded,
/// so it is not attempted, although every seed it left is. Allocates nothing.
pub(super) fn ordinary_solve_attempted(capture: &crate::retained_product::ProductCapture, requested: usize) -> bool {
    requested >= 1 && capture.ordinary.len() == requested && capture.ordinary.iter().all(|seed| seed.initial.is_some())
}
/// The first observation above its bound. Pure.
pub(super) fn check_phase<const N: usize>(gate: PhaseGate, observations: &[PhaseObservation; N], caps: &[u64; N]) -> Result<(), PhaseRefusal> {
    match observations.iter().zip(caps).find(|(o, cap)| o.observed > **cap) {
        Some((o, cap)) => Err(PhaseRefusal { gate, fact: o.fact, observed: o.observed, cap: *cap }),
        None => Ok(()),
    }
}
/// The text atoms the gates use, from the generated profile (the T08 closure at l <= 128 on
/// this code basis; RV84 C-N1 and RV87 N-3 for the longest strings).
pub(super) mod text_atoms {
    use super::profile as p;
    /// D_env: the envelope's own diagnostics.
    pub(crate) const D_ENV: u64 = p::TEXT_D_ENV;
    /// Text(diag_env): the envelope diagnostics' text bytes.
    pub(crate) const DIAG_ENV: u64 = p::TEXT_TEXT_DIAG_ENV;
    /// Text(row): one result row's text bytes.
    pub(crate) const ROW: u64 = p::TEXT_TEXT_ROW;
    /// Text(err): one retained error text.
    pub(crate) const ERR: u64 = p::TEXT_TEXT_ERR;
    /// L_PUB: the longest envelope string (the final integrity message, RV84 C-N1).
    pub(crate) const L_PUB: u64 = p::L_PUB;
    /// L_DIAGID: the longest reached `diagnostic:` id template (RV87 N-3).
    pub(crate) const L_DIAGID: u64 = p::L_DIAGID;
}
/// A profile form's in-build value; 0 (a bound every positive fact exceeds, so the gate
/// refuses, fail-closed) if its checked arithmetic overflows.
pub(super) const fn profile_bytes(index: usize) -> u64 {
    checked_or_zero(profile::form(&profile::FORMS[index], &profile::ATOM_VALUES))
}
/// A checked profile value, or 0 (fail-closed as a gate cap) when its arithmetic overflowed.
pub(super) const fn checked_or_zero(bytes: Option<u64>) -> u64 {
    match bytes {
        Some(bytes) => bytes,
        None => 0,
    }
}
/// The capacity of a Vec built by `h` pushes from empty (RawVec's doubling from 4).
pub(super) const fn push_capacity(h: u64) -> u64 {
    if h == 0 {
        return 0;
    }
    let mut c = 4;
    while c < h {
        c *= 2;
    }
    c
}
/// P_final ≤ 7n + 51m + 8g + 3 (DOMAIN.md §2, derived): one case's result rows.
pub(super) const P_FINAL: u64 = (7 * caps::NODES + 51 * caps::MEMBERS + 8 * caps::SUPPORTS + 3) as u64;
/// The gate bounds. Count bounds are D1's caps; text bounds are 2× the G4 text
/// atoms (exact-capacity copies at most double: API_G4.md §2); the preview tree
/// bounds are ordinary_caps.py's PREVIEW facts at the caps. The byte bounds (T11, T11
/// without its late capture, T11.4) are the generated profile's in-build forms.
/// B1 SA (option S3; I82 STUDY §4.3; PLAN_v2 §2.3), at C = `LOAD_CASES`: G-B bounds each
/// case's loads by l and their running total by L; G-C bounds the result rows, their
/// capacity and text by C·P_final (RV107 N-13), the contract-evidence facts by C times
/// the per-case preview facts, and the retained error text by C·(3m + 1)·Text(err) (I82's
/// assumption, checked against SP's producer in phase 4). The diagnostic, string and byte
/// bounds read the profile's text atoms and forms, which SQ regenerates at C = 3.
pub(super) const fn phase_caps() -> PhaseCaps {
    use caps::*;
    let (n, m, g, c) = (NODES as u64, MEMBERS as u64, SUPPORTS as u64, LOAD_CASES as u64);
    let k = if 6 * n < RESTRAINTS as u64 { 6 * n } else { RESTRAINTS as u64 };
    PhaseCaps {
        late: [n, m, m, g, LOADS as u64, TOTAL_LOADS as u64, k, SPRINGS as u64, 2 * MATERIALS as u64,
            profile_bytes(profile::F_T11).saturating_sub(profile_bytes(profile::F_T11_LATE_CAPTURE))],
        complete: [
            c * P_FINAL,
            push_capacity(c * P_FINAL),
            2 * c * P_FINAL * text_atoms::ROW,
            text_atoms::D_ENV,
            push_capacity(text_atoms::D_ENV),
            2 * text_atoms::DIAG_ENV,
            text_atoms::L_PUB,
            text_atoms::L_DIAGID,
            0,
            c * (3 * m + 2 * g),
            c * (3 + m + g),
            c * (9 + 15 * m + 2 * g),
            c * (m * (128 + 1024 + 3 * 120) + (2 * m + g) * 128 + g * (128 + 64)),
            c * ((9 + 15 * m + 2 * g) * 40),
            0,
            profile_bytes(profile::F_T11),
            profile_bytes(profile::F_T11_ORDINARY_SEED),
            c * (3 * m + 1) * text_atoms::ERR,
            0,
        ],
    }
}

// ---- U3's budgets (TRANSFER_COMPLETION.md §3) -------------------------------

/// The budgets U3 meets (B-1 to B-10). Structural budgets are counts; byte
/// budgets are the generated profile's in-build forms.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) struct PhaseBudgets {
    /// B-1 (RV84 S-7): ordinary runs per invocation.
    pub(super) ordinary_runs: u32,
    /// B-2: the staged typed copy, live from staging to the end of serialization.
    pub(super) staged_copy_bytes: u64,
    /// B-3: the one successor Value, moved and never cloned.
    pub(super) successor_bytes: u64,
    /// B-4: the precommit invocation Value, dropped right after `validate`.
    pub(super) precommit_invocation_bytes: u64,
    /// B-5: the precommit reader's peak, and its process-lifetime statics.
    pub(super) precommit_reader_bytes: u64,
    pub(super) reader_statics_bytes: u64,
    /// B-6: the N1 notice reserve, made before W1 starts (B1 SA: one per case in A, so
    /// at most C reserves; I82 STUDY §4.3).
    pub(super) notice_reserve_bytes: u64,
    /// B-7: fallible allocations after the first mutation of a published owner.
    pub(super) fallible_allocations_after_first_mutation: u32,
    /// B-8: the reserved-stack thread's heap, and its stack R.
    pub(super) thread_heap_bytes: u64,
    pub(super) reserved_stack_bytes: u64,
    /// B-9 (RV82 N9): parses per invocation.
    pub(super) parses_per_invocation: u32,
    /// B-10: new allocations by a `W1Fallback` value.
    pub(super) fallback_new_allocations: u32,
}
/// B-6 for grant 1b's exact reservation (`ReservedNotice::reserve`): one
/// diagnostic slot; the id (`format!`, at most twice its 30 + id + 12 bytes);
/// the code, severity and source; the 196-byte message reservation; one case-id
/// reference (its Vec slot and its String).
const NOTICE_RESERVE_BYTES: u64 = {
    use std::mem::size_of;
    let text = caps::TEXT_BYTES;
    (size_of::<crate::Diagnostic>() + 2 * (30 + text + 12) + 30 + 4 + 20 + 196 + size_of::<String>() + text) as u64
};
static PHASE_BUDGETS: PhaseBudgets = PhaseBudgets {
    ordinary_runs: 1,
    staged_copy_bytes: profile_bytes(profile::F_STAGED),
    successor_bytes: profile_bytes(profile::F_SUCC),
    precommit_invocation_bytes: profile_bytes(profile::F_INVOC),
    precommit_reader_bytes: checked_or_zero(profile::t17(&profile::ATOM_VALUES)),
    reader_statics_bytes: profile_bytes(profile::F_STATICS),
    notice_reserve_bytes: NOTICE_RESERVE_BYTES * caps::LOAD_CASES as u64,
    fallible_allocations_after_first_mutation: 0,
    thread_heap_bytes: (8 << 10) + std::mem::size_of::<crate::RetainedPreviewOutput>() as u64,
    reserved_stack_bytes: RESERVED_STACK_BYTES as u64,
    parses_per_invocation: 1,
    fallback_new_allocations: 0,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RetainedCaller {
    Direct,
    Headless,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissingAdmissionTerm {
    BootstrapBuildAndStack,
    ObjectBackingAndContainerLaws,
    NestedTypedOwners,
    OrdinaryActiveAndSuffix,
    PreparationNativeProofAndPublication,
    CallerCompletion,
    MovingAndTemporaryOwners,
}
const UNKNOWN_TERMS: [MissingAdmissionTerm; 7] = [
    MissingAdmissionTerm::BootstrapBuildAndStack,
    MissingAdmissionTerm::ObjectBackingAndContainerLaws,
    MissingAdmissionTerm::NestedTypedOwners,
    MissingAdmissionTerm::OrdinaryActiveAndSuffix,
    MissingAdmissionTerm::PreparationNativeProofAndPublication,
    MissingAdmissionTerm::CallerCompletion,
    MissingAdmissionTerm::MovingAndTemporaryOwners,
];
/// D-6's build status. `Missing` when no profile is registered (none since G6);
/// `Stale` when a profile is registered but this build does not match it;
/// `Registered` only for a build matching a registered profile.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileStatus {
    Missing,
    Stale,
    Registered,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AllowanceStatus {
    Unselected,
}

/// Borrowed actual caller roots, never a qualified label or caller byte total.
/// Cross-crate visibility is not authentication. Headless is refused at D1.0 (D-2), even in the registered build;
/// these roots only supply facts and cannot authorize an observer or solve.
pub struct RetainedHeadlessContext<'a> {
    payload: &'a Value,
    invocation: &'a Value,
    request_id: &'a String,
}
impl<'a> RetainedHeadlessContext<'a> {
    pub fn from_borrowed_roots(
        payload: &'a Value,
        invocation: &'a Value,
        request_id: &'a String,
    ) -> Self {
        Self {
            payload,
            invocation,
            request_id,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HeadlessRootFacts {
    pub payload: BorrowedValueFacts,
    pub invocation: BorrowedValueFacts,
    pub request_id: CapacityFact,
    /// Separate fact records are not automatically summed: aliases share owners.
    pub payload_and_invocation_alias: bool,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RetainedAdmissionReport {
    pub caller: RetainedCaller,
    pub raw: BorrowedValueFacts,
    pub typed: BorrowedRequestFacts,
    pub captured_digest: CapacityFact,
    pub captured_encoded_length: usize,
    pub headless: Option<HeadlessRootFacts>,
    pub profile: ProfileStatus,
    pub allowance: AllowanceStatus,
    /// U4 G5: the law's own facts and verdict (private; never serialized).
    law: AdmissionLaw,
}
impl RetainedAdmissionReport {
    pub fn required_unknown_terms(&self) -> &'static [MissingAdmissionTerm] {
        &UNKNOWN_TERMS
    }
    pub fn census_complete(&self) -> bool {
        self.raw.status == CensusStatus::Complete
            && self.typed.dof_upper.is_ok()
            && self
                .headless
                .map(|h| {
                    h.payload.status == CensusStatus::Complete
                        && h.invocation.status == CensusStatus::Complete
                })
                .unwrap_or(true)
    }
    /// The law's record: its census extensions and the first failing clause.
    #[cfg_attr(not(test), allow(dead_code))]
    pub(super) fn law(&self) -> &AdmissionLaw {
        &self.law
    }
}

/// Linear (U3 grant 1b): neither `Clone` nor `Copy`. The facade moves it onto the
/// reserved-stack thread and into the observer, which uses it for G-B and G-C.
pub(super) struct CapturePermit {
    _profile: &'static RegisteredProfile,
}
impl CapturePermit {
    /// R from STACK_PLAN.md §1 (the `cfg(test)` override in test builds).
    pub(super) fn reserved_stack_bytes(&self) -> usize {
        reserved_stack()
    }
    /// G-B, immediately before the late old-source capture.
    pub(super) fn check_late(&self, facts: &LateFacts<'_>) -> Result<(), PhaseRefusal> {
        check_phase(PhaseGate::Late, &late_observations(facts), &phase_caps().late)
    }
    /// G-C, after the complete ordinary owner returns.
    pub(super) fn check_complete(&self, facts: &CompleteFacts<'_>) -> Result<(), PhaseRefusal> {
        check_phase(PhaseGate::Complete, &complete_observations(facts), &phase_caps().complete)
    }
    /// The budgets U3 meets (TRANSFER_COMPLETION.md §3).
    #[allow(dead_code)]
    pub(super) fn budgets(&self) -> &'static PhaseBudgets {
        &PHASE_BUDGETS
    }
}
fn admission(report: RetainedAdmissionReport) -> Result<CapturePermit, RetainedAdmissionReport> {
    // All counts and missing premises survive the refusal, including incomplete
    // traversal and overflow. A permit exists only for a registered profile that
    // the whole law selected: the one registered dev/test profile; any other build is Stale.
    match (report.law.refusal, report.law.registered.and_then(|index| REGISTERED_PROFILES.get(index))) {
        (None, Some(profile)) => Ok(CapturePermit { _profile: profile }),
        _ => Err(report),
    }
}
/// The law's order (DOMAIN.md §1, then S-1): D1.0, D1.1, the request-level
/// clauses D1.2–D1.11, then the bound for the selected profile. Pure: the build
/// status, the domain verdict and the bound are its inputs.
pub(super) fn law_order(
    caller: RetainedCaller,
    build: Result<usize, ProfileStatus>,
    domain: Result<(), AdmissionRefusal>,
    bound: impl FnOnce(usize) -> Result<u64, BoundRefusal>,
) -> Result<usize, AdmissionRefusal> {
    caller_clause(caller)?;
    let index = build.map_err(AdmissionRefusal::Profile)?;
    domain?;
    bound(index).map_err(AdmissionRefusal::Bound)?;
    Ok(index)
}
pub(super) enum Entry<'a> {
    Direct,
    Headless(RetainedHeadlessContext<'a>),
}
// The dispatch calls `admit` since U3; `assess` keeps its report-only form for
// the existing census tests.
#[cfg_attr(not(test), allow(dead_code))]
pub(super) fn assess(
    capture: &CapturedInvocation,
    request: &LinearStaticPreviewRequest,
    entry: Entry<'_>,
) -> RetainedAdmissionReport {
    match admit(capture, request, entry) {
        Err(report) => report,
        Ok((_permit, report)) => report,
    }
}
/// G-A (API_G4.md §1): the census, the D1 predicate (D1.0–D1.11; Headless is
/// refused under D-2), the D-6 build status and the cap-priced bound against M,
/// all allocation-free. A permit comes with the same report (RV85 S3), so a
/// permitted output keeps every fact. With no registered profile, every call
/// refuses at D1.1 and the report keeps the first failing domain clause too.
pub(super) fn admit(
    capture: &CapturedInvocation,
    request: &LinearStaticPreviewRequest,
    entry: Entry<'_>,
) -> Result<(CapturePermit, RetainedAdmissionReport), RetainedAdmissionReport> {
    let (caller, headless) = match entry {
        Entry::Direct => (RetainedCaller::Direct, None),
        Entry::Headless(c) => (
            RetainedCaller::Headless,
            Some(HeadlessRootFacts {
                payload: borrowed_value_census(c.payload),
                invocation: borrowed_value_census(c.invocation),
                request_id: CapacityFact::string(c.request_id),
                payload_and_invocation_alias: std::ptr::eq(c.payload, c.invocation),
            }),
        ),
    };
    let build = build_status();
    let mut report = RetainedAdmissionReport {
        caller,
        raw: borrowed_value_census(capture.borrowed_raw()),
        typed: borrowed_request_census(request),
        captured_digest: CapacityFact::string(capture.borrowed_digest()),
        captured_encoded_length: capture.encoded_length(),
        headless,
        profile: build.map_or_else(|status| status, |_| ProfileStatus::Registered),
        allowance: AllowanceStatus::Unselected,
        law: AdmissionLaw {
            raw_text: raw_text_census(capture.borrowed_raw()),
            nested: nested_typed_census(request),
            refusal: None,
            domain: None,
            registered: None,
            required: None,
        },
    };
    let domain = domain_clauses(
        &DomainFacts {
            raw: &report.raw,
            raw_text: &report.law.raw_text,
            typed: &report.typed,
            nested: &report.law.nested,
            headless: report.headless.as_ref(),
            digest: &report.captured_digest,
        },
        request,
    );
    let mode = capture.mode();
    let mut required = None;
    let verdict = law_order(caller, build, domain, |index| {
        let bound = admission_bound(cap_priced_maximum(mode), REGISTERED_PROFILES[index].threshold_bytes);
        required = bound_required(&bound);
        bound
    });
    report.law.required = required;
    report.law.domain = domain.err();
    match verdict {
        Ok(index) => report.law.registered = Some(index),
        Err(refusal) => report.law.refusal = Some(refusal),
    }
    admission(report).map(|permit| (permit, report))
}

#[cfg(test)]
#[path = "retained_memory_law_tests.rs"]
mod law_tests;
#[cfg(test)]
#[path = "retained_memory_witness_tests.rs"]
mod witness_tests;

#[cfg(test)]
pub(super) mod tests {
    use super::*;
    thread_local! { static DISPATCH_COUNT: std::cell::Cell<Option<usize>> = const { std::cell::Cell::new(None) }; }
    pub(crate) fn ordinary_dispatch_entered() {
        DISPATCH_COUNT.with(|c| {
            if let Some(n) = c.get() {
                c.set(Some(n + 1));
            }
        });
    }
    #[test]
    fn actual_retained_entry_dispatches_ordinary_once() {
        // G6: the counted dispatch is the unpermitted one; the milestone is made out of D1
        // (B1, PLAN_v2 §2.3 and RV107 SF-2: C + 1 load cases, D1.4) so no build admits it.
        // The permitted path's single ordinary run (U3's B-1) is not counted by this hook
        // (QUALIFICATION.md §7).
        let mut raw: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
        ))
        .unwrap();
        let case = raw["model"]["load_cases"][0].clone();
        for ordinal in 2..=caps::LOAD_CASES + 1 {
            let mut copy = case.clone();
            copy["id"] = Value::String(format!("case-{ordinal}"));
            raw["model"]["load_cases"].as_array_mut().unwrap().push(copy);
        }
        assert_eq!(raw["model"]["load_cases"].as_array().unwrap().len(), caps::LOAD_CASES + 1);
        for mode in [
            crate::PreviewSolverMode::SparseInteractive,
            crate::PreviewSolverMode::DenseScrutiny,
        ] {
            DISPATCH_COUNT.with(|c| c.set(Some(0)));
            let result =
                crate::run_linear_static_preview_value_with_retained_direct(raw.clone(), mode)
                    .unwrap();
            // G6: the qualified build admits the milestone (one dispatch, capture installed);
            // any other build is Stale. Either way the ordinary run is dispatched once.
            let expected = build_status().map_or_else(|status| status, |_| ProfileStatus::Registered);
            assert_eq!(result.admission().unwrap().profile, expected);
            assert_eq!(
                result.admission().unwrap().law().domain,
                Some(AdmissionRefusal::Family(D1Clause::Invocation, FamilyFact::LoadCases)),
                "C + 1 cases: outside D1 at D1.4"
            );
            DISPATCH_COUNT.with(|c| {
                assert_eq!(c.get(), Some(1));
                c.set(None);
            });
        }
    }

    #[test]
    fn prefix_depth_and_value_limits_are_honest() {
        let mut deep = Value::Null;
        for _ in 0..DEPTH_LIMIT + 2 {
            deep = Value::Array(vec![deep]);
        }
        let f = borrowed_value_census(&deep);
        assert_eq!(f.status, CensusStatus::DepthLimit);
        assert_eq!(f.maximum_depth, DEPTH_LIMIT);
        let wide = Value::Array(vec![Value::Null; VALUE_LIMIT]);
        let f = borrowed_value_census(&wide);
        assert_eq!(f.status, CensusStatus::ValueLimit);
        assert_eq!(f.values, VALUE_LIMIT);
        assert_eq!(f.array_elements, VALUE_LIMIT);
    }
    #[test]
    fn arithmetic_refusal_keeps_prefix_and_never_wraps() {
        let mut prior = usize::MAX;
        assert_eq!(add(&mut prior, 1), Err(CensusStatus::ArithmeticOverflow));
        assert_eq!(prior, usize::MAX);
        assert_eq!(
            checked_dof(usize::MAX),
            Err(CensusStatus::ArithmeticOverflow)
        );
    }
    #[test]
    fn object_keys_and_array_spare_capacity_are_observed() {
        let mut key = String::with_capacity(73);
        key.push('k');
        let mut text = String::with_capacity(91);
        text.push('v');
        let mut array = Vec::with_capacity(31);
        array.push(Value::String(text));
        let mut map = serde_json::Map::new();
        map.insert(key, Value::Array(array));
        let f = borrowed_value_census(&Value::Object(map));
        assert_eq!(f.status, CensusStatus::Complete);
        assert_eq!(f.values, 3);
        assert_eq!(f.array_elements, 1);
        assert!(f.array_capacity_elements >= 31);
        assert!(f.string_capacity_bytes >= 91);
        assert!(f.key_capacity_bytes >= 73);
        assert_eq!(f.string_bytes, 1);
    }
    #[test]
    fn missing_stale_overflow_and_unknowns_cannot_mint_a_permit() {
        let raw: Value = serde_json::from_str(include_str!(
            "../../../fixtures/product_preview/rf_skew_t_cant_off_122_r1e-04.request.json"
        ))
        .unwrap();
        let (request, capture) =
            CapturedInvocation::parse(raw, crate::PreviewSolverMode::SparseInteractive).unwrap();
        let base = assess(&capture, &request, Entry::Direct);
        for profile in [ProfileStatus::Missing, ProfileStatus::Stale] {
            for status in [
                CensusStatus::Complete,
                CensusStatus::DepthLimit,
                CensusStatus::ValueLimit,
                CensusStatus::ArithmeticOverflow,
            ] {
                let mut report = base;
                report.profile = profile;
                report.raw.status = status;
                // G6: the law records D1.1's refusal for a Missing/Stale build (a profile now exists).
                report.law.refusal = Some(AdmissionRefusal::Profile(profile));
                let refusal = match admission(report) {
                    Err(r) => r,
                    Ok(_) => panic!("no profile can authorize execution"),
                };
                assert_eq!(refusal, report);
                assert_eq!(refusal.allowance, AllowanceStatus::Unselected);
                assert_eq!(refusal.required_unknown_terms().len(), 7);
            }
        }
    }
}
