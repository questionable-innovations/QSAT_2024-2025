#pragma once
#include <Arduino.h>

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

const int LoRa_SPI_SCK = P1_1;
const int LoRa_SPI_MOSI = P1_2;
const int LoRa_SPI_MISO = P1_3;
const int LoRa_Reset = P2_7;
const int LoRa_IRQ = P2_6;
const int LoRa_DIO1 = P2_5;


const int ESP32Cam_OneWire = P2_4;

const int LSM6DSMTR_SCL = P4_7;
const int LSM6DSMTR_SDA = P4_6;

const int GPS_TX = P4_3;
const int GPS_RX = P4_2;
const int GPS_Enable = P2_3;

const int CamTrigServo_Trigger = P2_1;
const int CamTrigServo_Reload = P2_0;

const int ESP32Cam_Backup = P1_7;

const int CurrentSense = P5_3;
const int LightSense = P5_2;
const int Buzzer = P5_1;
const int BattVoltageSense = P5_0;

const int StatusLED1 = P3_3;
const int StatusLED2 = P3_2;
const int StatusLED3 = P3_1;

const int BattManage_5VEnable = P3_0;
