#include <beacon/gps.h>
#include "gps.h"

/*
   This sample code tracks satellite elevations using TinyGPSCustom objects.

   Satellite numbers and elevations are not normally tracked by TinyGPSPlus, but
   by using TinyGPSCustom we get around this.

   It requires the use of SoftwareSerial and assumes that you have a
   4800-baud serial GPS device hooked up on pins 4(RX) and 3(TX).
*/
static const uint32_t GPSBaud = 4800;

static const int MAX_SATELLITES = 40;
static const int PAGE_LENGTH = 40;

// The TinyGPSPlus object
TinyGPSPlus gps;

// The serial connection to the GPS device, using GPS_RX, GPS_TX

// struct ring_buffer;

// class HardwareSerial : public Stream
// {
// 	private:
// 		ring_buffer *_rx_buffer;
// 		ring_buffer *_tx_buffer;
// 		uint8_t uartOffset;
// 		uint16_t rxPinMode;
// 		uint16_t txPinMode;
// 		uint8_t rxPin;
// 		uint8_t txPin;
// 		uint8_t lock;
// 	public:
// 		HardwareSerial(ring_buffer *rx_buffer, ring_buffer *tx_buffer, uint8_t uartOffset, uint16_t rxPinMode, uint16_t txPinMode, uint8_t rxPin, uint8_t txPin)


ring_buffer rx_buffer;
ring_buffer tx_buffer;

HardwareSerial hs(&rx_buffer, &tx_buffer, DEBUG_UART_MODULE_OFFSET, 0, 0, GPS_RX, GPS_TX);


TinyGPSCustom totalGPGSVMessages(gps, "GPGSV", 1);  // $GPGSV sentence, first element
TinyGPSCustom messageNumber(gps, "GPGSV", 2);	    // $GPGSV sentence, second element
TinyGPSCustom satNumber[4];			    // to be initialized later
TinyGPSCustom elevation[4];
bool anyChanges = false;
unsigned long linecount = 0;

struct
{
    int elevation;
    bool active;
} sats[MAX_SATELLITES];

void setupGps() {
    hs.begin(GPSBaud);

    Serial.println(F("SatElevTracker.ino"));
    Serial.println(F("Displays GPS satellite elevations as they change"));
    Serial.print(F("Testing TinyGPSPlus library v. "));
    Serial.println(TinyGPSPlus::libraryVersion());
    Serial.println(F("by Mikal Hart"));
    Serial.println();

    // Initialize all the uninitialized TinyGPSCustom objects
    for (int i = 0; i < 4; ++i) {
        satNumber[i].begin(gps, "GPGSV", 4 + 4 * i);  // offsets 4, 8, 12, 16
        elevation[i].begin(gps, "GPGSV", 5 + 4 * i);  // offsets 5, 9, 13, 17
    }
}

void gpsLoop() {
    // Dispatch incoming characters
    if (hs.available() > 0) {
	gps.encode(hs.read());

	if (totalGPGSVMessages.isUpdated()) {
	    for (int i = 0; i < 4; ++i) {
		int no = atoi(satNumber[i].value());
		if (no >= 1 && no <= MAX_SATELLITES) {
		    int elev = atoi(elevation[i].value());
		    sats[no - 1].active = true;
		    if (sats[no - 1].elevation != elev) {
			sats[no - 1].elevation = elev;
			anyChanges = true;
		    }
		}
	    }

	    int totalMessages = atoi(totalGPGSVMessages.value());
	    int currentMessage = atoi(messageNumber.value());
	    if (totalMessages == currentMessage && anyChanges) {
		if (linecount++ % PAGE_LENGTH == 0)
		    printHeader();
		TimePrint();
		for (int i = 0; i < MAX_SATELLITES; ++i) {
		    Serial.print(F(" "));
		    if (sats[i].active)
			IntPrint(sats[i].elevation, 2);
		    else
			Serial.print(F("   "));
		    sats[i].active = false;
		}
		Serial.println();
		anyChanges = false;
	    }
	}
    }
}

void IntPrint(int n, int len) {
    int digs = n < 0 ? 2 : 1;
    for (int i = 10; i <= abs(n); i *= 10)
	++digs;
    while (digs++ < len)
	Serial.print(F(" "));
    Serial.print(n);
    Serial.print(F(" "));
}

void TimePrint() {
    if (gps.time.isValid()) {
	if (gps.time.hour() < 10)
	    Serial.print(F("0"));
	Serial.print(gps.time.hour());
	Serial.print(F(":"));
	if (gps.time.minute() < 10)
	    Serial.print(F("0"));
	Serial.print(gps.time.minute());
	Serial.print(F(":"));
	if (gps.time.second() < 10)
	    Serial.print(F("0"));
	Serial.print(gps.time.second());
	Serial.print(F(" "));
    } else {
	Serial.print(F("(unknown)"));
    }
}

void printHeader() {
    Serial.println();
    Serial.print(F("Time     "));
    for (int i = 0; i < MAX_SATELLITES; ++i) {
	Serial.print(F(" "));
	IntPrint(i + 1, 2);
    }
    Serial.println();
    Serial.print(F("---------"));
    for (int i = 0; i < MAX_SATELLITES; ++i)
	Serial.print(F("----"));
    Serial.println();
}
