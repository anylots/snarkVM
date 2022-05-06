// Copyright (C) 2019-2022 Aleo Systems Inc.
// This file is part of the snarkVM library.

// The snarkVM library is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// The snarkVM library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with the snarkVM library. If not, see <https://www.gnu.org/licenses/>.

use super::Hash;
use crate::{
    function::{parsers::*, Instruction, Opcode, Operation, Registers},
    Program,
    Value,
};
use snarkvm_circuits::{algorithms::Pedersen1024, Hash as CircuitHash, Parser, ParserResult};
use snarkvm_utilities::FromBytes;

use nom::combinator::map;
use snarkvm_circuits::{Literal, ToBits};
use std::io::{Read, Result as IoResult};

/// Performs a Pedersen hash taking a 512-bit value as input.
pub type HashPed1024<P> = Hash<P, Pedersen1024<<P as Program>::Aleo>>;

impl<P: Program> Opcode for HashPed1024<P> {
    /// Returns the opcode as a string.
    #[inline]
    fn opcode() -> &'static str {
        "hash.ped1024"
    }
}

impl<P: Program> Parser for HashPed1024<P> {
    type Environment = P::Environment;

    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        map(UnaryOperation::parse, |operation| Self {
            operation,
            hasher: Pedersen1024::<P::Environment>::setup("PedersenCircuit0"),
        })(string)
    }
}

impl<P: Program> FromBytes for HashPed1024<P> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self {
            operation: UnaryOperation::read_le(&mut reader)?,
            hasher: Pedersen1024::<P::Environment>::setup("PedersenCircuit0"),
        })
    }
}

#[allow(clippy::from_over_into)]
impl<P: Program> Into<Instruction<P>> for HashPed1024<P> {
    /// Converts the operation into an instruction.
    fn into(self) -> Instruction<P> {
        Instruction::HashPed1024(self)
    }
}

impl<P: Program> Operation<P> for HashPed1024<P> {
    /// Evaluates the operation.
    #[inline]
    fn evaluate(&self, registers: &Registers<P>) {
        impl_pedersen_evaluate!(self, registers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_instruction_halts, test_modes, Identifier, Process, Register};

    type P = Process;

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<P>::parse("hash.ped1024 r0 into r1;").unwrap();
        assert!(matches!(instruction, Instruction::HashPed1024(_)));
    }

    test_modes!(
        address,
        HashPed1024,
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah",
        "7481826612753713410217729925917351756348444555753582640939800395433217644869field"
    );
    test_modes!(
        bool,
        HashPed1024,
        "true",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        field,
        HashPed1024,
        "1field",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        group,
        HashPed1024,
        "2group",
        "564546604862166251269187547407669874296117017437472033102698766525356841251field"
    );
    test_modes!(
        i8,
        HashPed1024,
        "1i8",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        i16,
        HashPed1024,
        "1i16",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        i32,
        HashPed1024,
        "1i32",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        i64,
        HashPed1024,
        "1i64",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        i128,
        HashPed1024,
        "1i128",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        u8,
        HashPed1024,
        "1u8",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        u16,
        HashPed1024,
        "1u16",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        u32,
        HashPed1024,
        "1u32",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        u64,
        HashPed1024,
        "1u64",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        u128,
        HashPed1024,
        "1u128",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        scalar,
        HashPed1024,
        "1scalar",
        "6122249396247477588925765696834100286827340493907798245233656838221917119242field"
    );
    test_modes!(
        string,
        HashPed1024,
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\"",
        "6527352455101447058676936043301538275522692367742743342479030477370441210344field"
    );

    test_instruction_halts!(
        string_halts,
        HashPed1024,
        "The Pedersen hash input cannot exceed 1024 bits.",
        "\"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\""
    );

    #[test]
    fn test_composite() {
        let first = Value::<P>::Composite(Identifier::from_str("message"), vec![
            Literal::from_str("true.public"),
            Literal::from_str("false.private"),
        ]);

        let registers = Registers::<P>::default();
        registers.define(&Register::from_str("r0"));
        registers.define(&Register::from_str("r1"));
        registers.assign(&Register::from_str("r0"), first);

        HashPed1024::from_str("r0 into r1").evaluate(&registers);

        let value = registers.load(&Register::from_str("r1"));
        let expected = Value::<P>::from_str(
            "6122249396247477588925765696834100286827340493907798245233656838221917119242field.private",
        );
        assert_eq!(expected, value);
    }

    #[test]
    #[should_panic(expected = "The Pedersen hash input cannot exceed 1024 bits.")]
    fn test_composite_halts() {
        let first = Value::<P>::Composite(Identifier::from_str("message"), vec![
            Literal::from_str("1field.public"),
            Literal::from_str("2field.private"),
            Literal::from_str("3field.private"),
            Literal::from_str("4field.private"),
            Literal::from_str("5field.private"),
        ]);

        let registers = Registers::<P>::default();
        registers.define(&Register::from_str("r0"));
        registers.define(&Register::from_str("r1"));
        registers.assign(&Register::from_str("r0"), first);

        HashPed1024::from_str("r0 into r1").evaluate(&registers);
    }
}
