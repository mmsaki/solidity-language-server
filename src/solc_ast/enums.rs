//! Enum types used across the Solidity AST.
//!
//! These mirror the enums defined in the official solc source at
//! `libsolidity/ast/ASTEnums.h` and serialized by `ASTJsonExporter.cpp`.

use serde::{Deserialize, Serialize};

// ── Contract ───────────────────────────────────────────────────────────────

/// The kind of a contract declaration.
///
/// Matches `ContractKind` in `ASTEnums.h`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum ContractKind {
    Contract,
    Interface,
    Library,
}

// ── Visibility ─────────────────────────────────────────────────────────────

/// Visibility of a declaration.
///
/// Matches `Visibility` in `ASTEnums.h`, serialized as lowercase strings.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Default,
    Private,
    Internal,
    Public,
    External,
}

// ── State mutability ───────────────────────────────────────────────────────

/// How a function or variable can mutate EVM state.
///
/// Matches `StateMutability` in `ASTEnums.h`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StateMutability {
    Pure,
    View,
    Nonpayable,
    Payable,
}

// ── Variable mutability ────────────────────────────────────────────────────

/// Mutability of a variable declaration.
///
/// Serialized by `ASTJsonExporter::visit(VariableDeclaration)`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Mutability {
    Mutable,
    Immutable,
    Constant,
    /// Transient storage (EIP-1153, solc 0.8.28+).
    Transient,
}

// ── Storage location ───────────────────────────────────────────────────────

/// Storage location of a variable.
///
/// `"default"` means the compiler decides (state variables, value types).
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum StorageLocation {
    Default,
    Storage,
    Memory,
    Calldata,
    /// Transient storage (EIP-1153, solc 0.8.28+).
    Transient,
}

// ── Function kind ──────────────────────────────────────────────────────────

/// The kind of a function definition.
///
/// Note: `freeFunction` is serialized with camelCase by `ASTJsonExporter`.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionKind {
    Function,
    Receive,
    Constructor,
    Fallback,
    FreeFunction,
}

// ── Function call kind ─────────────────────────────────────────────────────

/// The kind of a function call expression.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum FunctionCallKind {
    FunctionCall,
    TypeConversion,
    StructConstructorCall,
}

// ── Literal kind ───────────────────────────────────────────────────────────

/// The kind of a literal value.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum LiteralKind {
    Bool,
    Number,
    String,
    HexString,
    UnicodeString,
}

// ── Modifier invocation kind ───────────────────────────────────────────────

/// Distinguishes modifier calls from base constructor specifiers.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum ModifierInvocationKind {
    ModifierInvocation,
    BaseConstructorSpecifier,
}

// ── Yul literal kind ───────────────────────────────────────────────────────

/// The kind of a Yul literal value.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "camelCase")]
pub enum YulLiteralKind {
    Number,
    String,
    Bool,
}
