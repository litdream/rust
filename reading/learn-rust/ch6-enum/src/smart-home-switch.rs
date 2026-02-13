enum DeviceCommand {
    Power(bool),             // toggle the power switch, store state in enum directly.  (It's common in Rust).
    SetBrightness(u8),       // Brightness.
    SetColor { r:u8, g:u8, b:u8 },   // We support fancy light-bulb. lol.   struct(r,g,b)
}

impl DeviceCommand {
    fn execute(&self) {
        match self {
            DeviceCommand::Power(true) => {
		println!("System: Booting up...");
	    }
            DeviceCommand::Power(false) => {
		println!("System: Shutting down...");
	    }
            DeviceCommand::SetBrightness(level) => {
                println!("Setting brightness to {}%.", level);
            }
            DeviceCommand::SetColor { r, g, b } => {
                println!("Changing color to RGB({}, {}, {}).", r, g, b);
            }
        }
    }

    // Power Switch toggle.
    fn toggle_power(&mut self) {
        if let DeviceCommand::Power(current_status) = self {
            *current_status = !*current_status;
            println!("Power toggled to: {}", current_status);
        } else {
            println!("Cannot toggle power: Device is currently in a different mode.");
        }
    }
}

fn main() {
    // Start with Power Off
    let mut my_lamp = DeviceCommand::Power(false);

    // toggle demo
    my_lamp.execute(); 
    my_lamp.toggle_power();
    my_lamp.execute();  

    // Light Orange light.
    my_lamp = DeviceCommand::SetColor { r: 255, g: 213, b: 128 };
    my_lamp.execute();

    // Light Brightness
    my_lamp = DeviceCommand::SetBrightness(35);
    my_lamp.execute();
}
