use hal::{ledc::{channel::{Channel, ChannelIFace}, LowSpeed}, gpio::{Gpio3, Output, PushPull, Gpio5, Gpio4, OutputPin}};



pub struct LedComponent<'b, RED: OutputPin, GREEN: OutputPin, BLUE: OutputPin> {
    red: Channel<'b,LowSpeed,RED>,
    green: Channel<'b,LowSpeed,GREEN>,
    blue: Channel<'b,LowSpeed,BLUE>,
}

impl <'b, RED: OutputPin, GREEN: OutputPin, BLUE: OutputPin>LedComponent<'b, RED, GREEN, BLUE> {
    pub fn new(red: Channel<'b,LowSpeed,RED>,green: Channel<'b,LowSpeed,GREEN>, blue: Channel<'b,LowSpeed,BLUE>)->Self {
        LedComponent { red, green, blue }
    }

    pub fn set_color(&mut self, red: u8, green: u8, blue: u8) {
        self.red.set_duty(red).unwrap();
        self.green.set_duty(green).unwrap();
        self.blue.set_duty(blue).unwrap();
    }

}