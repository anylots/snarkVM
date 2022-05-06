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
use snarkvm_circuits::{algorithms::Poseidon8, Hash as CircuitHash, Parser, ParserResult};
use snarkvm_utilities::FromBytes;

use nom::combinator::map;
use snarkvm_circuits::{Field, Literal, ToFields};
use std::io::{Read, Result as IoResult};

/// Performs a Poseidon hash with an input rate of 8.
pub type HashPsd8<P> = Hash<P, Poseidon8<<P as Program>::Aleo>>;

impl<P: Program> Opcode for HashPsd8<P> {
    /// Returns the opcode as a string.
    #[inline]
    fn opcode() -> &'static str {
        "hash.psd8"
    }
}

impl<P: Program> Parser for HashPsd8<P> {
    type Environment = P::Environment;

    #[inline]
    fn parse(string: &str) -> ParserResult<Self> {
        map(UnaryOperation::parse, |operation| Self { operation, hasher: Poseidon8::<P::Environment>::new() })(string)
    }
}

impl<P: Program> FromBytes for HashPsd8<P> {
    fn read_le<R: Read>(mut reader: R) -> IoResult<Self> {
        Ok(Self { operation: UnaryOperation::read_le(&mut reader)?, hasher: Poseidon8::<P::Environment>::new() })
    }
}

#[allow(clippy::from_over_into)]
impl<P: Program> Into<Instruction<P>> for HashPsd8<P> {
    /// Converts the operation into an instruction.
    fn into(self) -> Instruction<P> {
        Instruction::HashPsd8(self)
    }
}

impl<P: Program> Operation<P> for HashPsd8<P> {
    /// Evaluates the operation.
    #[inline]
    fn evaluate(&self, registers: &Registers<P>) {
        impl_poseidon_evaluate!(self, registers);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{test_instruction_halts, test_modes, Identifier, Process, Register};

    type P = Process;

    #[test]
    fn test_parse() {
        let (_, instruction) = Instruction::<P>::parse("hash.psd8 r0 into r1;").unwrap();
        assert!(matches!(instruction, Instruction::HashPsd8(_)));
    }

    test_modes!(
        field,
        HashPsd8,
        "1field",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        i8,
        HashPsd8,
        "1i8",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        i16,
        HashPsd8,
        "1i16",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        i32,
        HashPsd8,
        "1i32",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        i64,
        HashPsd8,
        "1i64",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        i128,
        HashPsd8,
        "1i128",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        u8,
        HashPsd8,
        "1u8",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        u16,
        HashPsd8,
        "1u16",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        u32,
        HashPsd8,
        "1u32",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        u64,
        HashPsd8,
        "1u64",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        u128,
        HashPsd8,
        "1u128",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        scalar,
        HashPsd8,
        "1scalar",
        "3999071741215241790607111275574824668617854796802626587041088136954841194555field"
    );
    test_modes!(
        string,
        HashPsd8,
        "\"aaaaaaaa\"",
        "4020837770720319542691472472080405581209506316726251354702740114046129734437field"
    );

    test_instruction_halts!(bool_halts, HashPsd8, "Invalid 'hash.psd8' instruction", "true");
    test_instruction_halts!(
        address_halts,
        HashPsd8,
        "Invalid 'hash.psd8' instruction",
        "aleo1d5hg2z3ma00382pngntdp68e74zv54jdxy249qhaujhks9c72yrs33ddah"
    );
    test_instruction_halts!(group_halts, HashPsd8, "Invalid 'hash.psd8' instruction", "2group");

    #[test]
    fn test_composite() {
        let first = Value::<P>::Composite(Identifier::from_str("message"), vec![
            Literal::from_str("1field.public"),
            Literal::from_str("2field.private"),
        ]);

        let registers = Registers::<P>::default();
        registers.define(&Register::from_str("r0"));
        registers.define(&Register::from_str("r1"));
        registers.assign(&Register::from_str("r0"), first);

        HashPsd8::from_str("r0 into r1").evaluate(&registers);

        let value = registers.load(&Register::from_str("r1"));
        let expected = Value::<P>::from_str(
            "2132636093982099992808836832692348220698310395516022520468979890154979376079field.private",
        );
        assert_eq!(expected, value);
    }
}
