use msp430fr2x5x_hal::{
    gpio::{
        Alternate1, Alternate3, Floating, Input, Output, Pin, Pin0, Pin1, Pin2, Pin3, Pin4, Pin5,
        Pin6, Pin7, Pullup,
    },
    pac::{E_USCI_A0, E_USCI_A1, E_USCI_B0, E_USCI_B1, P1, P2, P3, P4, P5, P6},
    serial::{Rx, Tx},
    spi::SpiBus,
};

// GPS UART Interface
pub type GpsEusci = E_USCI_A1;
pub type GpsTx = Tx<E_USCI_A1>;
pub type GpsRx = Rx<E_USCI_A1>;
pub type GpsTxPin = Pin<P4, Pin3, Alternate1<Input<Floating>>>;
pub type GpsRxPin = Pin<P4, Pin2, Alternate1<Input<Floating>>>;
pub type GpsEnPin = Pin<P2, Pin3, Output>;

// LoRa Radio SPI Interface
pub type LoraEusci = E_USCI_B0;
pub type LoraSpi = SpiBus<E_USCI_B0>;
pub type LoraMosiPin = Pin<P1, Pin2, Alternate1<Input<Floating>>>;
pub type LoraSclkPin = Pin<P1, Pin1, Alternate1<Input<Floating>>>;
pub type LoraMisoPin = Pin<P1, Pin3, Alternate1<Input<Floating>>>;
pub type LoraResetPin = Pin<P2, Pin7, Output>;
pub type LoraIrqPin = Pin<P2, Pin6, Input<Floating>>;
pub type LoraDio1Pin = Pin<P2, Pin5, Input<Floating>>;
pub type LoraCSPin = Pin<P6, Pin5, Output>;

// Status LEDs
pub type RedLedPin = Pin<P3, Pin3, Output>; // Status LED1
pub type BlueLedPin = Pin<P3, Pin2, Output>; // Status LED2
pub type GreenLedPin = Pin<P3, Pin1, Output>; // Status LED3

// Accelerometer I2C
pub type AccelEusci = E_USCI_B1;
pub type AccelSclPin = Pin<P4, Pin7, Alternate1<Input<Floating>>>;
pub type AccelSdaPin = Pin<P4, Pin6, Alternate1<Input<Floating>>>;

// Camera/Servo Control
pub type CamTrigServoPin = Pin<P2, Pin1, Output>; // May need TimerOutput<TB1, CCR2> for PWM
pub type ShutterServoPin = Pin<P2, Pin0, Output>; // May need TimerOutput<TB1, CCR1> for PWM
pub type EspCamPin = Pin<P2, Pin4, Output>; // OneWire communication with ESP32 Cam
pub type EspCamAltPin = Pin<P1, Pin7, Alternate1<Input<Floating>>>; // Backup serial for ESP32 Cam (UCA0TXD)

// Sensors
pub type CurrentSensePin = Pin<P5, Pin3, Alternate3<Input<Floating>>>; // A11
pub type LightSensePin = Pin<P5, Pin2, Alternate3<Input<Floating>>>; // A10
pub type BuzzerPin = Pin<P5, Pin1, Output>; // May need TimerOutput<TB2, CCR2> for PWM
pub type HalfVbatPin = Pin<P5, Pin0, Alternate3<Input<Floating>>>; // A8

// Power Management
pub type Enable5vPin = Pin<P3, Pin0, Output>; // 5V Enable for battery management

// Debug interface (if needed)
pub type DebugEusci = E_USCI_A0;
pub type DebugTxPin = Pin<P1, Pin7, Alternate1<Input<Floating>>>;

// 1	P1.2	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	SPI Master Out
// 2	P1.1	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	SPI Clock
// 8	P2.7	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	Lora Reset
// 9	P2.6	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	LoRa IRQ (DIO0)
// 10	P2.5	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	LoRa Configurable DIO1 (RadioLib wants it?)
// 11	P2.4	I/O	LVCMOS	DVCC	OFF	TRUE	ESP32 Cam	Hopefully will be used for OneWire commication with the ESPCam
// 12	P4.7	I/O	LVCMOS	DVCC	OFF	TRUE	Accelerometer	LSM6DSMTR I2C SCL
// 13	P4.6	I/O	LVCMOS	DVCC	OFF	TRUE	Accelerometer	LSM6DSMTR I2C SDA
// 23	P4.3	I/O	LVCMOS	DVCC	OFF	TRUE	GPS Module	GPS TX
// 24	P4.2	I/O	LVCMOS	DVCC	OFF	TRUE	GPS Module	GPS RX
// 27	P2.3	I/O	LVCMOS	DVCC	OFF	TRUE	GPS Module	GPS Enable
// 29	P2.1(RD)	I/O	LVCMOS	DVCC	OFF	TRUE	CamTrigServo	Send trigger servo PWM signal
// 30	P2.0	I/O	LVCMOS	DVCC	OFF	TRUE	CamTrigServo	Shutter reload servo PWM signal
// 31	P1.7	I/O	LVCMOS	DVCC	OFF	FALSE	JTAG, ESP32 Cam	Dual Use - It's a backup for the ESPCam com. If the DIY OneWire protocol doesn't work, we'll jank together the serial logs with communication to the ESPCam.
// 40	P5.3	I/O	LVCMOS	DVCC	OFF	TRUE	Current Sense	Analog current sense for servo controlling the shutter
// 41	P5.2	I/O	LVCMOS	DVCC	OFF	TRUE	LightSense	Read the photoresiste to detect leaving the capsual
// 42	P5.1	I/O	LVCMOS	DVCC	OFF	TRUE	Buzzer	PWM output for the buzzer - useful for finding capsual after rocket launch
// 43	P5.0	I/O	LVCMOS	DVCC	OFF	TRUE	Batt Manage	Default Sense Design
// 44	P3.3	I/O	LVCMOS	DVCC	OFF	TRUE	Status LED	LED1
// 45	P3.2	I/O	LVCMOS	DVCC	OFF	TRUE	Status LED	LED2
// 46	P3.1	I/O	LVCMOS	DVCC	OFF	TRUE	Status LED	LED3
// 47	P3.0	I/O	LVCMOS	DVCC	OFF	TRUE	Batt Manage	5V Enable
// 48	P1.3	I/O	LVCMOS	DVCC	OFF	TRUE	LoRa	SPI Master In
