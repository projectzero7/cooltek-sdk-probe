# Firmware

Each module uses the same process to check for and download new firmware versions.

The MasterHUB application uses `https://dev.pms.masterctrl.coolermaster.com` as the base URL for firmware-related requests.

## Getting the latest firmware version

The latest firmware check URL for each module uses the same template: `/api/devices/<uuid>/firmwares/stable`, where `<uuid>` is the modules UUID. See [metadata](./metadata.md) for module UUIDs.

These URLs are unauthenticated endpoints that returns the latest stable firmware version and URI as JSON:

```json
{
  "version": "<version>",
  "uri": "/assets/devices/<uuid>/firmwares/<version>.sys"
}
```

Unknown UUIDs appear to return an `HTTP 200` with an empty JSON body.

> **IMPORTANT**
>
> The `uri` in the JSON payload is a _relative_ URI, not an absolute URI.

### Latest Firmware Versions

* [M0046](https://dev.pms.masterctrl.coolermaster.com/api/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/stable)

## Downloading firmware versions

The firmware download URL for each module uses the same template: `/assets/devices/<uuid>/firmwares/<version>.sys`, where `<uuid>` is the module's UUID and `<version>` is the version of the firmware to download. See [metadata](./metadata.md) for module UUIDs.

Unknown UUIDs and versions here return an `HTTP 403` with the following XML:

```xml
<Error>
  <Code>AccessDenied</Code>
  <Message>Access Denied</Message>
</Error>
```

### Known Firmware Versions

#### M0046

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/1.0.0.0.sys)
* [2.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.0.0.sys)
* [2.0.1.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.1.0.sys)
* [2.0.3.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.3.0.sys)
* [2.0.4.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.4.0.sys)
