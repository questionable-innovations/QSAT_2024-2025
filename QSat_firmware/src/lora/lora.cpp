#include <definitions/pins.h>
#include <SPI.h>
#include <LoRa.h>

const long frequency = 915E6;

void lora_setup()
{
    LoRa.setPins(csPin, LoRa_Reset, LoRa_IRQ);
};

void lora_tx(char *data, int len)
{
    LoRa.beginPacket();
    LoRa.write(data, len);
    LoRa.endPacket();
};