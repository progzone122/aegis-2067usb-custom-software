# aegis-2067-usb-custom-software
Open source custom software for AULA Aegis 2067usb (a.k.a F2067) keyboard

## USB Protocol
The USB keyboard protocol was reverse engineered, [protocol documentation can be found here](https://github.com/progzone122/aegis-2067usb-protocol)

## Usage guide
```bash
aegis2067usb [COMMAND]
```
#### Commands:
| Command                  | Description                                               |
|--------------------------|-----------------------------------------------------------|
| help                     | Print this message or the help of the given subcommand(s) |
| animation **[ID/ NAME]** | Configuration of animation effect                         |
| brightness **[0..5]**    | Configuration of LED brightness                           |
| speed **[0..2]**         | Configuration of speed animation effect                   |

##### Examples:
```bash
aegis2067usb --animation 17
```
```bash
aegis2067usb --brightness 3
```
```bash
aegis2067usb --speed 2
```

## Build guide
- ### Udev rules
  A user must have write access on the /dev/bus/usb/XXX/YYY nodes to successfully open a device. Use udev rules to configure these permissions.

  #### File "/etc/udev/rules.d/70-plugdev-usb.rules:"
  ```text
  SUBSYSTEM=="usb", MODE="0660", GROUP="plugdev"
  ```

  After adding the rule, reload udev rules
  ```bash
  sudo udevadm control --reload
  ```

- ### Build package
  - #### CLI version:
  ```bash
  cargo run --package aegis2067usb
  ```
  - #### GUI version:
  ```bash
  cargo run --package gui
  ```

## RoadMap:

- ### Macro Settings
  | Function name | API | CLI | GUI |
  |---------------|-----|-----|-----|
  | Macro key     | -   | -   | -   |
  | Single key    | -   | -   | -   |
  | Media key     | -   | -   | -   |
  | Function key  | -   | -   | -   |
- ### LED Settings
  | Function name     | API | CLI | GUI |
  |-------------------|-----|-----|-----|
  | Animation effects | +   | +   | -   |
  | Animation speed   | +   | +   | -   |
  | LED Brightness    | +   | +   | -   |
- ### Other functions
  | Function name               | API | CLI | GUI |
  |-----------------------------|-----|-----|-----|
  | Reset default configuration | +   | +   | -   |