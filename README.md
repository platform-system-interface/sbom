# SWID - Software ID Utilities

## About

This is a collection of packages/libraries and utilities dealing with [SBoMs](
https://blogs.gnome.org/hughsie/2022/03/10/firmware-software-bill-of-materials/)
in platform initialization firmware, such as [coreboot](https://coreboot.org).

There is a [Python implementation](https://github.com/hughsie/python-uswid/) as
well, though not suitable for every environment.
So here are [Rust](https://www.rust-lang.org/) and [Go](https://go.dev/). :-)

Note that this is work in progress. Contributions are welcome!

## Go

```sh
go run ./cmds/goswid path/to/firmware.bin
```

<details>
  <summary>Example output</summary>

```
magic 53424f4dd6ba2eaca3e67a52aaee3baf at: 2673e0
uswid:
  Version: 2
  Header size: 24
  Payload size: 1977
swid.SoftwareIdentity{
    XMLName:          xml.Name{},
    CoSWIDExtension:  swid.CoSWIDExtension{},
    GlobalAttributes: swid.GlobalAttributes{Lang:"en-US"},
    TagID:            swid.TagID{
        val: uuid.UUID{0xa9, 0x3, 0x2c, 0x9d, 0x2a, 0xaa, 0x5a, 0x25, 0xa0, 0xe6, 0x6d, 0x86, 0x5b, 0x24, 0xe6, 0xd2},
    },
    TagVersion:      0,
    Corpus:          false,
    Patch:           false,
    Supplemental:    false,
    SoftwareName:    "coreboot",
    SoftwareVersion: "bd34cca50aba130364f362618881693c0478a4a6",
    VersionScheme:   &swid.VersionScheme{
        val: int64(3),
    },
    Media:         "",
    SoftwareMetas: &swid.SoftwareMetas{
        {
            SoftwareMetaExtension:   swid.SoftwareMetaExtension{},
            GlobalAttributes:        swid.GlobalAttributes{},
            ActivationStatus:        "",
            ChannelType:             "",
            ColloquialVersion:       "58a040842be9ff7b6c6df4519c7da44476d7c95f",
            Description:             "",
            Edition:                 "",
            EntitlementDataRequired: (*bool)(nil),
            EntitlementKey:          "",
            Generator:               (*swid.TagID)(nil),
            PersistentID:            "org.coreboot.rocks",
            Product:                 "",
            ProductFamily:           "",
            Revision:                "",
            Summary:                 "coreboot is a project to develop open source boot firmware for various architectures",
            UnspscCode:              "",
            UnspscVersion:           "",
        },
    },
    Entities: {
        {
            EntityExtension:  swid.EntityExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            EntityName:       "9elements",
            RegID:            "9elements.com",
            Roles:            swid.Roles{
                val: {
                    int64(1),
                },
            },
            Thumbprint: (*swid.HashEntry)(nil),
        },
    },
    Links: &swid.Links{
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/Apache-2.0.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/BSD-3-Clause.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/CC-BY-4.0.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/CC-BY-SA-3.0.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/GPL-2.0-only.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/GPL-2.0-or-later.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/GPL-3.0-only.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/GPL-3.0-or-later.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/ISC.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/MIT.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "https://spdx.org/licenses/X11.html",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-2),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "swid:9579af2b-39d8-59f1-ac5a-5b1fd4c03bd0",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(10),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "swid:e5a249ad-04bb-5b63-a587-ceb7b0e331c9",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(10),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "swid:23edb84c-5d68-544e-b389-8a67f6c80247",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(10),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "swid:23edb84c-5d68-544e-b389-8a67f6c80247",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(10),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
        {
            LinkExtension:    swid.LinkExtension{},
            GlobalAttributes: swid.GlobalAttributes{},
            Artifact:         "",
            Href:             "swid:8e0d0fd3-1116-50ad-ba5f-599c8117c42b",
            Media:            "",
            Ownership:        (*swid.Ownership)(nil),
            Rel:              swid.Rel{
                val: int64(-1),
            },
            MediaType: "",
            Use:       (*swid.Use)(nil),
        },
    },
    Payload:  (*swid.Payload)(nil),
    Evidence: (*swid.Evidence)(nil),
}
```
</details>

## Rust

```sh
cargo run path/to/firmware.bin
```

<details>
  <summary>Example output</summary>

```
../sbom/firmware.bin
SBOM found at 2673e0
Magic: SBOM ([53, 42, 4f, 4d, d6, ba, 2e, ac, a3, e6, 7a, 52, aa, ee, 3b, af])
  Version: 2
  Header size: 24
  Payload size: 1977
Payload start/end: [a9, 0f, 65, 65][32, 2e, 33, 37]
```
</details>
