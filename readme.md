# Esp32 s3 rust led toggle esp-hal latest
> wvv 20250113

```bash
   Compiling rust-esp v0.1.0 (D:\rust-esp)
    Finished `dev` profile [optimized + debuginfo] target(s) in 0.47s
     Running `espflash flash --monitor --chip esp32s3 target\xtensa-esp32s3-none-elf\debug\rust-esp`
[2026-01-13T03:16:24Z INFO ] Serial port: 'COM11'
[2026-01-13T03:16:24Z INFO ] Connecting...
[2026-01-13T03:16:24Z INFO ] Using flash stub
Chip type:         esp32s3 (revision v0.2)
Crystal frequency: 40 MHz
Flash size:        16MB
Features:          WiFi, BLE, Embedded Flash
MAC address:       10:20:ba:4f:f0:4c
App/part. size:    89,104/16,384,000 bytes, 0.54%
[00:00:00] [========================================]      14/14      0x0      Skipped! (checksum matches)                                                                                             
[00:00:00] [========================================]       1/1       0x8000   Skipped! (checksum matches)                                                                                             
[00:00:00] [========================================]      24/24      0x10000  Verifying... OK!                                                                                                        
[2026-01-13T03:16:26Z INFO ] Flashing has completed!
Commands:
    CTRL+R    Reset chip
    CTRL+C    Exit

ESP-ROM:esp32s3-20210327
Build:Mar 27 2021
rst:0x15 (USB_UART_CHIP_RESET),boot:0x8 (SPI_FAST_FLASH_BOOT)
Saved PC:0x40378f09
SPIWP:0xee
mode:DIO, clock div:2
load:0x3fce2820,len:0x158c
load:0x403c8700,len:0xd24
load:0x403cb700,len:0x2f34
entry 0x403c8924
I (27) boot: ESP-IDF v5.5.1-838-gd66ebb86d2e 2nd stage bootloader
I (27) boot: compile time Nov 26 2025 12:27:56
I (27) boot: Multicore bootloader
I (29) boot: chip revision: v0.2
I (31) boot: efuse block revision: v1.3
I (35) boot.esp32s3: Boot SPI Speed : 40MHz
I (39) boot.esp32s3: SPI Mode       : DIO
I (43) boot.esp32s3: SPI Flash Size : 16MB
I (47) boot: Enabling RNG early entropy source...
I (51) boot: Partition Table:
I (54) boot: ## Label            Usage          Type ST Offset   Length
I (60) boot:  0 nvs              WiFi data        01 02 00009000 00006000
I (66) boot:  1 phy_init         RF data          01 01 0000f000 00001000
I (73) boot:  2 factory          factory app      00 00 00010000 00fa0000
I (80) boot: End of partition table
I (83) esp_image: segment 0: paddr=00010020 vaddr=3c000020 size=0568ch ( 22156) map
I (96) esp_image: segment 1: paddr=000156b4 vaddr=3fc8919c size=007c0h (  1984) load
I (98) esp_image: segment 2: paddr=00015e7c vaddr=40378000 size=0119ch (  4508) load
I (106) esp_image: segment 3: paddr=00017020 vaddr=00000000 size=08ff8h ( 36856) 
I (121) esp_image: segment 4: paddr=00020020 vaddr=42010020 size=05bc8h ( 23496) map
I (128) boot: Loaded app from partition at offset 0x10000
I (128) boot: Disabling RNG early entropy source...
LED toggled
LED toggled
LED toggled
LED toggled
LED toggled
LED toggled
LED toggled
LED toggled
```
