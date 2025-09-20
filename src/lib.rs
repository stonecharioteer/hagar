
/**
 * Struct to manage bluetooth connections
 */
pub struct BluetoothManager {

}

impl BluetoothManager {
    pub fn scan(&self) {
        unimplemented!()
    }
}

#![cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn scan_bluetooth() {
        let bluetooth = BluetoothManager {};
        bluetooth.scan();
        println!("Scanning for bluetooth devices")
    }
}
