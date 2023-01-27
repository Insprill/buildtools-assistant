[![Contributors][contributors-shield]][contributors-url]
[![Forks][forks-shield]][forks-url]
[![Stargazers][stars-shield]][stars-url]
[![Issues][issues-shield]][issues-url]
[![Apache 2.0 License][license-shield]][license-url]




<h1 align="center">BuildTools Assistant</h1>
<p align="center">
  A cross-platform CLI tool to easily run Spigot's BuildTools for one or more versions simultaneously, without the Java version mess.
  <br />
  <br />
  <a href="https://github.com/Insprill/buildtools-assistant/issues">Report Bugs</a>
  ·
  <a href="https://github.com/Insprill/buildtools-assistant/issues">Request Features</a>
</p>




<!-- TABLE OF CONTENTS -->
<details>
  <summary><h2 style="display: inline-block">Table of Contents</h2></summary>
  <ol>
    <li><a href="#about-the-project">About The Project</a></li>
    <li><a href="#usage">Usage</a></li>
    <li><a href="#compiling">Compiling</a></li>
    <li><a href="#contributing">Contributing</a></li>
    <li><a href="#license">License</a></li>
  </ol>
</details>




<!-- USAGE -->

## Usage

To use BuildTools Assistant, you can either build it locally, or download the [latest prebuilt binary][latest-actions-build]. You can then run it with the `--help` flag to see the proper usage.

<details>
  <summary>Example for all versions between 1.8 and 1.19.3</summary>
  <code>./buildtools-assistant 1.19.3 1.19.2 1.19.1 1.19 1.18.2 1.18.1 1.18 1.17.1 1.17 1.16.5 1.16.4 1.16.3 1.16.2 1.16.1 1.16 1.15.2 1.15.1 1.15 1.14.4 1.14.3 1.14.2 1.14.1 1.14 1.13.2 1.13.1 1.13 1.12.2 1.12.1 1.12 1.11.2 1.11.1 1.11 1.10.2 1.10 1.9.4 1.9.2 1.9.2 1.9 1.8.8 1.8.7 1.8.6 1.8.5 1.8.4 1.8.3 1.8</code>
</details




<!-- Compiling -->

## Compiling

To compile BuildTools Assistant, you'll need [Rust](https://www.rust-lang.org/tools/install) 1.67 or newer.  
Clone this repo, then run `cargo build --release` from your terminal.  
You can find the compiled program in the `target/release` directory.  




<!-- CONTRIBUTING -->

## Contributing

Contributions are what make the open source community such an amazing place to learn, inspire, and create.  
Any contributions you make are **greatly appreciated**!  
If you're new to contributing to open-source projects, you can follow [this](https://docs.github.com/en/get-started/quickstart/contributing-to-projects) guide.




<!-- LICENSE -->

## License

Distributed under the Apache 2.0 License. See [`LICENSE`][license-url] for more information.




<!-- MARKDOWN LINKS & IMAGES -->
<!-- https://www.markdownguide.org/basic-syntax/#reference-style-links -->

[contributors-shield]: https://img.shields.io/github/contributors/Insprill/buildtools-assistant.svg?style=for-the-badge
[contributors-url]: https://github.com/Insprill/buildtools-assistant/graphs/contributors
[forks-shield]: https://img.shields.io/github/forks/Insprill/buildtools-assistant.svg?style=for-the-badge
[forks-url]: https://github.com/Insprill/buildtools-assistant/network/members
[stars-shield]: https://img.shields.io/github/stars/Insprill/buildtools-assistant.svg?style=for-the-badge
[stars-url]: https://github.com/Insprill/buildtools-assistant/stargazers
[issues-shield]: https://img.shields.io/github/issues/Insprill/buildtools-assistant.svg?style=for-the-badge
[issues-url]: https://github.com/Insprill/buildtools-assistant/issues
[license-shield]: https://img.shields.io/github/license/Insprill/buildtools-assistant.svg?style=for-the-badge
[license-url]: https://github.com/Insprill/buildtools-assistant/blob/master/LICENSE
[latest-actions-build]: https://nightly.link/Insprill/buildtools-assistant/workflows/build/master
