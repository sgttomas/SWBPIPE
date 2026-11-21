//! The content-source traits of the export decision interface (DEL-12-02's native caller
//! interface), name for name: [`SourceObject`], [`SourceList`], [`Peek`], [`Member`],
//! [`WalkStopped`] and [`MAX_SOURCE_NESTING`], with the promises their comments state.
//!
//! This module is replaced by the dependency on `open_pipe_stress_export_decision` when that
//! crate lands: its definitions are copied here unchanged, and nothing else of that crate is.
//! Its comments name items of that crate (`Content`, `RuleError`, the crate's "Content given
//! by a source") that are not here.

use std::borrow::Cow;

use serde_json::Value;

/// The greatest depth of source nesting a rule walks, counting the root [`Content::Object`]
/// as 1 and each source object or source list handed over inside another as one more. A
/// source nested deeper is refused ([`RuleError::ContentSourceInconsistent`]). An ordinary
/// value handed over by a source does not count toward it, whatever its own depth.
pub const MAX_SOURCE_NESTING: usize = 64;

/// A walk that did not reach its end: its visitor stopped it, or its source or the source's
/// writer could not go on. Nothing a stopped walk staged is exposed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WalkStopped;

/// An object whose members its source gives: small members by name, and every member in
/// order by a walk the source itself drives. The rule reads members by name and visits them
/// in order; it reads nothing else of a source.
///
/// # What a source promises
///
/// - [`member_names`](SourceObject::member_names) gives every member name, each once, and
///   [`peek`](SourceObject::peek) answers for exactly those names. Both write nothing, stage
///   nothing, and give the same answer each time they are asked.
/// - A member the source holds as a value is peeked as [`Peek::Value`] and walked as
///   [`Member::Value`], with an equal value. A member it gives only by walk is peeked as
///   [`Peek::Object`] and walked as [`Member::Object`], or peeked as [`Peek::List`] and
///   walked as [`Member::List`]. No other pairing.
/// - A member named `provenance` is held as a value, whatever its kind. It is the one member
///   a rule reads into before it visits an object's members.
/// - [`walk`](SourceObject::walk) hands the visitor every member once, in ascending UTF-16
///   code unit order of its name (the order of canonical JSON). It returns `Ok` only when it
///   has handed over every member, no call of the visitor returned `Err`, and every source
///   object or list it handed over had been walked to its end exactly once when the
///   visitor's call for it returned. Otherwise it stops and returns `Err`.
/// - What it stages as it walks is what it shows, as the crate's "Content given by a source"
///   states.
/// - Source objects and lists nest to at most [`MAX_SOURCE_NESTING`].
///
/// # What a visitor owes a source
///
/// - It walks every [`Member::Object`] and [`Member::List`] it is handed to its end, once,
///   inside the call that handed it over, before it returns `Ok` from that call. The walk is
///   the visitor's act; the source checks it, as promised above.
/// - It passes on every `Err` it receives from a walk, and visits nothing further.
/// - It walks the root once.
///
/// # What a rule checks of a source
///
/// From what the traits give it, a rule checks that the names are distinct, that the walk
/// hands over exactly those names in ascending order, that each member it peeked is walked
/// in the pairing above with an equal value, that a name it asked for and was refused is
/// not a member, that `provenance` is held as a value, and the depth of nesting. It refuses
/// a source that fails one ([`RuleError::ContentSourceInconsistent`]). It cannot check that
/// a list handed over every entry, or what a source staged: those rest on the producer.
pub trait SourceObject {
    /// Every member name, each once, in any order.
    fn member_names(&self) -> Vec<String>;

    /// The member so named, without writing anything: its value where the source holds it
    /// as a value, its kind alone where the source gives it only by walk, and `None` exactly
    /// for a name that is not a member.
    fn peek(&self, name: &str) -> Option<Peek<'_>>;

    /// Hands `visit` every member once, in order.
    fn walk(
        &self,
        visit: &mut dyn FnMut(&str, Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped>;
}

/// A list whose entries its source gives one at a time, by a walk the source drives. An
/// entry lives for its call: a source may build it, hand it over and drop it.
///
/// A source promises that [`walk`](SourceList::walk) hands over every entry once, in index
/// order, and returns `Ok` only when it has handed over every entry, no call of the visitor
/// returned `Err`, and every source object or list it handed over had been walked to its end
/// exactly once when the visitor's call for it returned; that what it stages as it walks is
/// what it shows; and the bound on nesting. A visitor owes it what it owes a
/// [`SourceObject`]. A rule cannot check a list's completeness.
pub trait SourceList {
    /// Hands `visit` every entry once, in index order.
    fn walk(
        &self,
        visit: &mut dyn FnMut(Member<'_>) -> Result<(), WalkStopped>,
    ) -> Result<(), WalkStopped>;
}

/// What [`SourceObject::peek`] answers for a member.
#[derive(Debug, Clone, PartialEq)]
pub enum Peek<'a> {
    /// The member's value.
    Value(Cow<'a, Value>),
    /// An object the source gives only by walk.
    Object,
    /// A list the source gives only by walk.
    List,
}

/// A member or a list entry as a walk hands it over.
#[derive(Clone, Copy)]
pub enum Member<'a> {
    Value(&'a Value),
    Object(&'a dyn SourceObject),
    List(&'a dyn SourceList),
}
