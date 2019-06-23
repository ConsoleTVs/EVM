mod tests {
    extern crate evm;
    use evm::opcodes::Opcode;
    use evm::values::Value;

    // The program instructions to add two numbers and print it.
    static CODE: &[Opcode] = &[
        Opcode::Load(0, 0), // Load to register 0, the value from constant 0.
        Opcode::Load(1, 1), // Load to register 1 the value from constant 1.
        Opcode::Mul(0, 0, 1), // Perform a multiplication to register 1 of register 0 and register 1.
        Opcode::Exit // Terminate the interpretation.
    ];

    static REGISTERS: [Value; 2] = [
        Value::VInteger(0),
        Value::VInteger(0),
    ];

    #[test]
    fn integer_multiplication() {
        // The constant pool.
        let constants = &[
            Value::VInteger(5), // 5 (integer).
            Value::VInteger(10), // 10 (integer).
        ];
        // The registers to use (They can be default-initialized).
        let registers = &mut REGISTERS.clone();
        // Interpret the result.
        let result = evm::interpret(CODE, constants, registers);
        // Assert if the result is success.
        assert_eq!(result, evm::EVMResult::Success);
        // Assert the registers.
        assert_eq!(registers[0], evm::values::Value::VInteger(50));
        assert_eq!(registers[1], evm::values::Value::VInteger(10));
    }

    #[test]
    fn float_multiplication() {
        // The constant pool.
        let constants = &[
            Value::VFloat(5.0), // 5.0 (integer).
            Value::VFloat(10.0), // 10.0 (integer).
        ];
        // The registers to use (They can be default-initialized).
        let registers = &mut REGISTERS.clone();
        // Interpret the result.
        let result = evm::interpret(CODE, constants, registers);
        // Assert if the result is success.
        assert_eq!(result, evm::EVMResult::Success);
        // Assert the registers.
        assert_eq!(registers[0], evm::values::Value::VFloat(50.0));
        assert_eq!(registers[1], evm::values::Value::VFloat(10.0));
    }

    #[test]
    fn bool_multiplication() {
        // The constant pool.
        let constants = &[
            Value::VBoolean(true), // true (bool).
            Value::VBoolean(true), // true (bool).
        ];
        // The registers to use (They can be default-initialized).
        let registers = &mut REGISTERS.clone();
        // Interpret the result.
        let result = evm::interpret(CODE, constants, registers);
        // Assert if the result is success.
        assert_eq!(result, evm::EVMResult::Success);
        // Assert the registers.
        assert_eq!(registers[0], evm::values::Value::VInteger(1));
        assert_eq!(registers[1], evm::values::Value::VBoolean(true));
    }

    #[test]
    fn string_multiplication() {
        // The constant pool.
        let constants = &[
            Value::VString(String::from("Erik")), // "Hello, " (string).
            Value::VInteger(2), // "World" (string).
        ];
        // The registers to use (They can be default-initialized).
        let registers = &mut REGISTERS.clone();
        // Interpret the result.
        let result = evm::interpret(CODE, constants, registers);
        // Assert if the result is success.
        assert_eq!(result, evm::EVMResult::Success);
        // Assert the registers.
        assert_eq!(registers[0], evm::values::Value::VString(String::from("ErikErik")));
        assert_eq!(registers[1], evm::values::Value::VInteger(2));
    }

}
