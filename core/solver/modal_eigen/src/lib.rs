//! Code-neutral in-repo dense eigen core for F1 modal analysis.
//!
//! This crate implements the dense path of DEL-04-09's governing design
//! (`execution/PKG-04/DEL-04-09/Work/modal-analysis.md`): §3.2 (Cholesky
//! reduction, Householder tridiagonalization, implicit QL), the cluster
//! definition of §3.4 check 5 with the reported-set rule of §1.4, the two
//! shape normalizations and the participation quantities of §1.5, the
//! M-orthogonality measure of §3.4 check 3, the finite-mode count of §3.2
//! item 5, and the Sturm counts of §3.2 item 7 and §3.4 check 4 over
//! `sparse_direct`'s LDLᵀ.
//!
//! Determinism posture: single-threaded, fixed operation order, no randomized
//! tie-breaking, Rust standard library and in-repo path crates only. Only
//! IEEE binary64 `+ - * /`, `sqrt` and `mul_add` are used, each correctly
//! rounded, so results do not depend on a platform math library. The crate contains open
//! numerical-mechanics routines only. It does not encode design code
//! compliance checks, protected standards content, or private project data.

pub mod clusters;
pub mod dd;
pub mod dense;
pub mod matrix;
pub mod modal_quantities;
pub mod orthogonality;
pub mod sturm;

pub use matrix::{MatrixError, SymmetricMatrix};
