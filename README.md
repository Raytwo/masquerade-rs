# Masquerade-rs

Persona 5 Scramble (JP) file replacement plugin for Nintendo Switch.

## Prerequisites

* A Nintendo Switch capable of running a Custom Firmware
* The latest release of [Atmosphere CFW](https://github.com/Atmosphere-NX/Atmosphere/releases) by [SciresM](https://github.com/SciresM)
* (OPTIONAL) A TCP client of your choice if you want runtime logs.

## Setup

1. Download the latest [release](https://github.com/Raytwo/masquerade-rs/releases) of Masquerade-rs and extract the content of the ``sd`` directory at the root of your SD card.
2. Edit your system settings (atmosphere/config/system_settings.ini) and make sure to edit the ``ease_nro_restriction`` line so that it is toggled on. (``ease_nro_restriction = u8!0x1``)

## Usage

Place the files you edited in ``sd:/Masquerade/forge`` on your SD card and give them a name representing the FileID of the one you are replacing, WITHOUT EXTENSION.

If you wish to replace the file 0 of LINKDATA, then the file should be ``sd:/Masquerade/forge/0``.

NOTE: Filenames are allowed, as long as the FileID is the first thing in the name and the separation is a ``-`` symbol.  
Example: ``sd:/Masquerade/forge/0-msgdata.bin``

## Extras

- Copyright on screenshots has been removed
- Message logging is available using a TCP client (listen on port 6969 before running the game)

## Credits
* [jam1garner](https://github.com/jam1garner) - [cargo-skyline](https://github.com/jam1garner/cargo-skyline), [skyline-rs](https://github.com/ultimate-research/skyline-rs), [skyline-rs-template](https://github.com/ultimate-research/skyline-rs-template), support and using me as a guinea pig
* [shadowninja108](https://github.com/shadowninja108) and the Skyline contributors - [Skyline](https://github.com/shadowninja108/Skyline)
* [kolakcc](https://github.com/kolakcc) - Forge caching implementation, KTGL reverse engineering collaborator