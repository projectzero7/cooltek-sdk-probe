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

* [M003K](https://dev.pms.masterctrl.coolermaster.com/api/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/stable)
* [M005S](https://dev.pms.masterctrl.coolermaster.com/api/devices/6ceb80f4-15f0-5892-b987-f0159817bd21/firmwares/stable)
* [M0064](https://dev.pms.masterctrl.coolermaster.com/api/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/stable)
* [M032LE](https://dev.pms.masterctrl.coolermaster.com/api/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/stable)
* [M4315](https://dev.pms.masterctrl.coolermaster.com/api/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/stable)

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

#### M003K

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/1.0.0.0.sys)
* [1.0.0.1](https://dev.pms.masterctrl.coolermaster.com/assets/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/1.0.0.1.sys)
* [1.0.2.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/1.0.2.0.sys)
* [1.0.4.1](https://dev.pms.masterctrl.coolermaster.com/assets/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/1.0.4.1.sys)
* [1.0.5.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/1f7b995d-ee2f-5705-a07f-381a8ffcd2ac/firmwares/1.0.5.0.sys)

#### M005S

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/6ceb80f4-15f0-5892-b987-f0159817bd21/firmwares/1.0.0.0.sys)
* [1.0.2.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/6ceb80f4-15f0-5892-b987-f0159817bd21/firmwares/1.0.2.0.sys)
* [1.0.4.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/6ceb80f4-15f0-5892-b987-f0159817bd21/firmwares/1.0.4.0.sys)
* [1.0.5.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/6ceb80f4-15f0-5892-b987-f0159817bd21/firmwares/1.0.5.0.sys)

#### M0064

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/1.0.0.0.sys)
* [2.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.0.0.sys)
* [2.0.1.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.1.0.sys)
* [2.0.3.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.3.0.sys)
* [2.0.4.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/ebf5c18b-0386-5dba-ba01-498ffdae2908/firmwares/2.0.4.0.sys)

#### M032LE

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/1.0.0.0.sys)
* [1.0.0.1](https://dev.pms.masterctrl.coolermaster.com/assets/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/1.0.0.1.sys)
* [1.0.1.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/1.0.1.0.sys)
* [1.0.2.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/1.0.2.0.sys)
* [1.0.3.1](https://dev.pms.masterctrl.coolermaster.com/assets/devices/669a0142-86f2-556e-8206-18a1bb3e0af9/firmwares/1.0.3.1.sys)

#### M4315

* [1.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/1.0.0.0.sys)
* [1.0.0.1](https://dev.pms.masterctrl.coolermaster.com/assets/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/1.0.0.1.sys)
* [1.0.3.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/1.0.3.0.sys)
* [2.0.0.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/2.0.0.0.sys)
* [2.0.1.0](https://dev.pms.masterctrl.coolermaster.com/assets/devices/c9ffd9c7-9679-549a-b2be-a9530de33ebd/firmwares/2.0.1.0.sys)
