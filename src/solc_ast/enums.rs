//! Enum types used across the Solidity AST.
//!
//! These mirror the enums defined in the official solc source at
//! `libsolidity/ast/ASTEnums.h` and serialized by `ASTJsonExporter.cpp`.

use std::fmt;

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

impl fmt::Display for ContractKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Contract => write!(f, "contract"),
            Self::Interface => write!(f, "interface"),
            Self::Library => write!(f, "library"),
        }
    }
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

impl fmt::Display for Visibility {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, ""),
            Self::Private => write!(f, "private"),
            Self::Internal => write!(f, "internal"),
            Self::Public => write!(f, "public"),
            Self::External => write!(f, "external"),
        }
    }
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

impl fmt::Display for StateMutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Pure => write!(f, "pure"),
            Self::View => write!(f, "view"),
            Self::Nonpayable => write!(f, "nonpayable"),
            Self::Payable => write!(f, "payable"),
        }
    }
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

impl fmt::Display for Mutability {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Mutable => write!(f, "mutable"),
            Self::Immutable => write!(f, "immutable"),
            Self::Constant => write!(f, "constant"),
            Self::Transient => write!(f, "transient"),
        }
    }
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

impl fmt::Display for StorageLocation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Default => write!(f, "default"),
            Self::Storage => write!(f, "storage"),
            Self::Memory => write!(f, "memory"),
            Self::Calldata => write!(f, "calldata"),
            Self::Transient => write!(f, "transient"),
        }
    }
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

impl fmt::Display for FunctionKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Function => write!(f, "function"),
            Self::Receive => write!(f, "receive"),
            Self::Constructor => write!(f, "constructor"),
            Self::Fallback => write!(f, "fallback"),
            Self::FreeFunction => write!(f, "function"),
        }
    }
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
