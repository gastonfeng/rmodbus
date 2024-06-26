//! MODBUS Constants

// MODBUS Functions
pub const MODBUS_GET_COILS: u8 = 1;
pub const MODBUS_GET_DISCRETES: u8 = 2;
pub const MODBUS_GET_HOLDINGS: u8 = 3;
pub const MODBUS_GET_INPUTS: u8 = 4;
pub const MODBUS_SET_COIL: u8 = 5;
pub const MODBUS_SET_HOLDING: u8 = 6;
pub const MODBUS_SET_COILS_BULK: u8 = 15;
pub const MODBUS_SET_HOLDINGS_BULK: u8 = 16;
pub const MODBUS_READ_SLAVE_ID: u8 = 17;

pub const MODBUS_READ_FILE_RECORD: u8 = 20;
pub const MODBUS_WRITE_FILE_RECORD: u8 = 21;
pub const MODBUS_MASK_WRITE_HOLDING: u8 = 22;
pub const MODBUS_GET_SET_HOLDINGS_BULK: u8 = 23;
pub const MODBUS_MASK_WRITE_HOLDING_BULK: u8 = 24;
pub const MODBUS_READ_DEVICE_ID: u8 = 43;
pub const MODBUS_PROTOBUF_FUNCTION: u8 = 0x6F;
// MODBUS Errors
pub const MODBUS_ERROR_ILLEGAL_FUNCTION: u8 = 1;
pub const MODBUS_ERROR_ILLEGAL_DATA_ADDRESS: u8 = 2;
pub const MODBUS_ERROR_ILLEGAL_DATA_VALUE: u8 = 3;
