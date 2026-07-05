# M0064 Feature Reports

## Get Feature Reports

### `0x2d` - Get FW Infos

```mermaid
---
title: "Report Structure"
---
packet
+8:  "Report ID"
+8:  "Report length (in bytes)"
+32: "Unknown"
+88: "Firmware version"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x2d` (`45`) |
| Report length | The number of remaining bytes in the report. | Potentially `0x00` (`0`) to `0x78` (`120`) |
| Unknown | The purpose has not been discovered. | |
| Firmware version | The current firmware version. | Apparently an 11-character UTF-8 string. |

Example: `2d 0f 5c e8 ff 9a 30 31 2e 30 30 2e 30 30 2e 30 31 00`

> **WARNING**
>
> This feature report appears in the SDK but is unused by the application. See feature report `0x2e` for the one used by the application.

This seems to contain a very low firmware version, perhaps the lowest supported or factory original version.

#### Postprocessing

The SDK splits the 15 bytes into a 4-byte and 11-byte pair. The firmware version number appears to be in the 11-byte part.

<table>
    <tr>
        <th>Byte</th>
        <td>5c</td>
        <td>e8</td>
        <td>ff</td>
        <td>9a</td>
        <td>30</td>
        <td>31</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>2e</td>
        <td>30</td>
        <td>31</td>
        <td>00</td>
    </tr>
    <tr>
        <th>UTF-8</th>
        <td>\</td>
        <td>�</td>
        <td>�</td>
        <td>�</td>
        <td>0</td>
        <td>1</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td>.</td>
        <td>0</td>
        <td>1</td>
        <td></td>
    </tr>
</table>

The 4-byte part seems to be interpreted differently. I have not found any use for it in the rest of the application so it may be unused.

### `0x2e` - Get FW Infos

```mermaid
---
title: "Report Structure"
---
packet
+8:  "Report ID"
+8:  "Report length (in bytes)"
+32: "Unknown"
+88: "Firmware version"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x2e` (`46`) |
| Report length | The number of remaining bytes in the report. | Potentially `0` to `0x78` (`120`) |
| Unknown | The purpose has not been discovered. | |
| Firmware version | The current firmware version. | An 11-character UTF-8 string. |

Example: `2e 0f 12 54 4c 2f 30 32 2e 30 30 2e 30 34 2e 30 30 00`

> **IMPORTANT**
>
> This is the feature report that is consistently used by the application to get the module's firmware version.

This seems to contain the current firmware version.

#### Postprocessing

The SDK splits the 15 bytes into a 4-byte and 11-byte pair. The firmware version number appears to be in the 11-byte part.

<table>
    <tr>
        <th>Byte</th>
        <td>12</td>
        <td>54</td>
        <td>4c</td>
        <td>2f</td>
        <td>30</td>
        <td>32</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>2e</td>
        <td>30</td>
        <td>34</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>00</td>
    </tr>
    <tr>
        <th>UTF-8</th>
        <td></td>
        <td>T</td>
        <td>L</td>
        <td>/</td>
        <td>0</td>
        <td>2</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td>.</td>
        <td>0</td>
        <td>4</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td></td>
    </tr>
</table>

The 4-byte part seems to be interpreted differently. I have not found any use for it in the rest of the application so it may be unused.

### `0x2f` - Get FW Infos

```mermaid
---
title: "Report Structure"
---
packet
+8:  "Report ID"
+8:  "Report length (in bytes)"
+32: "Unknown"
+88: "Firmware version"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x2f` (`47`) |
| Report length | The number of remaining bytes in the report. | Potentially `0` to `0x78` (`120`) |
| Unknown | The purpose has not been discovered. | |
| Firmware version | The current firmware version. | An 11-character UTF-8 string. |

Example: `2f 0f 12 54 4c 2f 30 32 2e 30 30 2e 30 34 2e 30 30 00`

> **WARNING**
>
> This feature report appears in the SDK but is unused by the application. See feature report `0x2e` for the one used by the application.

This seems to contain the same or higher version than `0x2d` or `0x2e`. This is possibly the firmware version that has been uploaded to/staged on the device.

#### Postprocessing

The SDK splits the 15 bytes into a 4-byte and 11-byte pair. The firmware version number appears to be in the 11-byte part.

<table>
    <tr>
        <th>Byte</th>
        <td>12</td>
        <td>54</td>
        <td>4c</td>
        <td>2f</td>
        <td>30</td>
        <td>32</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>2e</td>
        <td>30</td>
        <td>34</td>
        <td>2e</td>
        <td>30</td>
        <td>30</td>
        <td>00</td>
    </tr>
    <tr>
        <th>UTF-8</th>
        <td></td>
        <td>T</td>
        <td>L</td>
        <td>/</td>
        <td>0</td>
        <td>2</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td>.</td>
        <td>0</td>
        <td>4</td>
        <td>.</td>
        <td>0</td>
        <td>0</td>
        <td></td>
    </tr>
</table>

The 4-byte part seems to be interpreted differently. I have not found any use for it in the rest of the application so it may be unused.

### `0x30` - Get Serial Number

```mermaid
---
title: "Report Structure"
---
packet
+8:   "Report ID"
+8:   "Report length (in bytes)"
+232: "Serial number"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x30` (`48`) |
| Report length | The number of remaining bytes in the report. | Potentially `0` to `0x1d` (`29`) |
| Serial number | The current serial number. | A UTF-8 encoded string with up to 29 bytes. |

Example: `30 14 4d 48 42 53 30 31 41 41 34 36 31 32 34 32 34 30 30 31 38 35`

> **NOTE**
>
> The SDK checks whether the serial number is less than or equal to `0x1d` (`29`). It is unclear at this point if that includes the C-style `null` terminator at the end of the string. The byte buffer is size `0x20` (`32`); if the `null` terminator is included then we would fill `0x1f` (`31`) bytes, while if it was not included we would fill all `0x20` (`32`) bytes.
>
> It's possible the last byte is potentially unused or an implicit `null` terminator. In practice, the serial numbers seem to only be about 20 characters.

#### Postprocessing

The serial number is a C-style UTF-8 string (with a `null` terminator) encoded as bytes.

<table>
    <tr>
        <th>Byte</th>
        <td>4d</td>
        <td>48</td>
        <td>42</td>
        <td>53</td>
        <td>30</td>
        <td>31</td>
        <td>41</td>
        <td>41</td>
        <td>34</td>
        <td>36</td>
        <td>31</td>
        <td>32</td>
        <td>34</td>
        <td>32</td>
        <td>34</td>
        <td>30</td>
        <td>30</td>
        <td>31</td>
        <td>38</td>
        <td>35</td>
    </tr>
    <tr>
        <th>UTF-8</th>
        <td>M</td>
        <td>H</td>
        <td>B</td>
        <td>S</td>
        <td>0</td>
        <td>1</td>
        <td>A</td>
        <td>A</td>
        <td>4</td>
        <td>6</td>
        <td>1</td>
        <td>2</td>
        <td>4</td>
        <td>2</td>
        <td>4</td>
        <td>0</td>
        <td>0</td>
        <td>1</td>
        <td>8</td>
        <td>5</td>
    </tr>
</table>

The first part of the serial number appears to also contain the model number. See [model numbers](../model_numbers.md) for information on significant bits of the model number.

### `0x31` - Get Device Infos

This feature appears in the SDK but is unused by the application and appears to return an empty response. It is possibly an unused/legacy endpoint.

Example: `31 14 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00`

### `0x32` - Get LED Brightness

```mermaid
---
title: "Report Structure"
---
packet
+8: "Report ID"
+8: "Report length (in bytes)"
+8: "Brightness"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x32` (`50`) |
| Report length | The number of remaining bytes in the report. | Always `0x01` (`1`) |
| Brightness | The current LED brightness. | Integers in the range `[0x00, 0x64]` (`[0, 100]`) |

Example: `32 01 64`

### `0x33` - Get Slider Value

This feature report appears in the SDK but does not appear to generate a response. It is possibly unused by this module.

### `0x34` - Get LED Mode

```mermaid
---
title: "Report Structure"
---
packet
+8: "Report ID"
+8: "Report length (in bytes)"
+8: "Mode"
```

| Element | Description | Acceptable Values |
| --- | --- | --- |
| Report ID | The ID of the report. | Always `0x34` (`52`) |
| Report length | The number of remaining bytes in the report. | Always `0x01` (`1`) |
| Mode | The current LED mode. | Any [lighting mode](../../lighting_modes.md) |

Example: `34 01 01`

### `0x35` - Get Device Direction

This feature report appears in the SDK but does not appear to generate a response. It is possibly unused by this module.

### `0x36` - Get Config Info

This feature report appears in the SDK but the report structure is unknown.

Example: `36 10 7b 87 df e4 b0 00 00 00 34 18 cc 97 b0 00 00 00`

## Send Feature Reports

There are no known send feature reports for this module.
