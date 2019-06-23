mod tests {
    extern crate evm;
    use evm::opcodes::Opcode;
    use evm::values::Value;

    // The program instructions to add two numbers and print it.
    static CODE: &[Opcode] = &[
        Opcode::Load(0, 0), // Load to register 0, the value from constant 0.
        Opcode::Load(1, 1), // Load to register 1 the value from constant 1.
        Opcode::Add(0, 0, 1), // Perform an addition to register 1 of register 0 and register 1.
        Opcode::Print(0), // Print the value of the register 0.
        Opcode::Exit // Terminate the interpretation.
    ];

    static REGISTERS: [Value; 2] = [
        Value::VInteger(5), // 5 (integer).
        Value::VInteger(10), // 10 (integer).
    ];

    #[test]
    fn integer_addition() {
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
        assert_eq!(registers[0], evm::values::Value::VInteger(15));
        assert_eq!(registers[1], evm::values::Value::VInteger(10));
    }

    #[test]
    fn float_addition() {
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
        assert_eq!(registers[0], evm::values::Value::VFloat(15.0));
        assert_eq!(registers[1], evm::values::Value::VFloat(10.0));
    }

    #[test]
    fn bool_addition() {
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
        assert_eq!(registers[0], evm::values::Value::VInteger(2));
        assert_eq!(registers[1], evm::values::Value::VBoolean(true));
    }

    #[test]
    fn string_addition() {
        // The constant pool.
        let constants = &[
            Value::VString(String::from("Hello, ")), // "Hello, " (string).
            Value::VString(String::from("World")), // "World" (string).
        ];
        // The registers to use (They can be default-initialized).
        let registers = &mut REGISTERS.clone();
        // Interpret the result.
        let result = evm::interpret(CODE, constants, registers);
        // Assert if the result is success.
        assert_eq!(result, evm::EVMResult::Success);
        // Assert the registers.
        assert_eq!(registers[0], evm::values::Value::VString(String::from("Hello, World")));
        assert_eq!(registers[1], evm::values::Value::VString(String::from("World")));
    }

}
