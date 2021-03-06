error_chain! {
    foreign_links {
        HidError(::hid::Error);
        UartError(::cp211x_uart::Error);
    }
    errors {
        OutOfRange {
            description("Argument is out of range")
            display("argument is out of range")
        }
        CommandWrite(cmd: &'static str) {
            description("Command writing error")
            display("can't write command '{}' to DMM", cmd)
        }
        CommandError {
            description("Command execution error")
            display("can't execute command due to error")
        }
        WaitTimeout {
            description("Wait timeout is exceed")
            display("did not receive message from DMM, timeout is exceed")
        }
        InvalidDateTime(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) {
            description("Invalid date and time")
            display("invalid date and/or time ({}/{}/{} {}:{}:{})", year, month, day, hour, minute, second)
        }
        RecordNameTooLong(name: String) {
            description("Record name length exceeds 10 characters")
            display("record name length exceeds 10 characters: '{}'", name)
        }
        InvalidRecordName(name: String) {
            description("Invalid record name (printable ASCII characters are allowed only)")
            display("invalid record name (printable ASCII characters are allowed only): '{}'", name)
        }
        RecordIntervalIsOutOfRange(interval: u16) {
            description("Record interval is out of range (1..3600 second(s))")
            display("record interval is out of range (1..3600 second(s)): {}", interval)
        }
        RecordDurationIsOutOfRange(duration: u32) {
            description("Record duration is out of range (1..143999 minute(s))")
            display("record interval is out of range (1..143999 minute(s)): {}", duration)
        }
        UnknownReplyCode(code: u16) {
            description("Unknown reply code")
            display("unknown reply code (0x{:04X})", code)
        }
        UnknownMessageFormat(format: u8) {
            description("Unknown message format")
            display("unknown message format (0x{:02X})", format)
        }
        UnknownMeasurementKind(kind: u8) {
            description("Unknown measurement message kind")
            display("unknown measurement message kind (0x{:02X})", kind)
        }
        UnknownMeasurementMode(mode: u16) {
            description("Unknown measurement message mode")
            display("unknown measurement message mode (0x{:04X})", mode)
        }
        UnknownMeasurementRange(range: u8) {
            description("Unknown measurement range")
            display("unknown measurement range (0x{:02X})", range)
        }
        UnknownMeasurementUnit(unit: String) {
            description("Unknown measurement unit")
            display("unknown measurement unit '{}' ({:?})", unit, unit.as_bytes())
        }
    }
}
