<div align="center">

![Star a repo](https://dudochkin-victor.github.io/assets/ux-charts/logo-wide.svg)
# UX Charts

[![API Docs][docrs-badge]][docrs-url]
[![Crates.io][crates-badge]][crates-url]
[![MPL-2.0 licensed][license-badge]][license-url]
[![Gitter chat][gitter-badge]][gitter-url]
[![Rustc Version 1.45+][rust-badge]][rust-url]
[![loc][loc-badge]][loc-url]
</div>

[docrs-badge]: https://img.shields.io/docsrs/ux-charts?style=flat-square
[docrs-url]: https://docs.rs/ux-charts/
[crates-badge]: https://img.shields.io/crates/v/ux-charts.svg?style=flat-square
[crates-url]: https://crates.io/crates/ux-charts
[license-badge]: https://img.shields.io/badge/license-MPL--2.0-blue.svg?style=flat-square
[license-url]: https://github.com/angular-rust/ux-charts/blob/master/LICENSE
[gitter-badge]: https://img.shields.io/gitter/room/angular_rust/angular_rust.svg?style=flat-square
[gitter-url]: https://gitter.im/angular_rust/community
[rust-badge]: https://img.shields.io/badge/rustc-1.45-lightgrey.svg?style=flat-square
[rust-url]: https://blog.rust-lang.org/2020/07/16/Rust-1.45.0.html
[loc-badge]: https://tokei.rs/b1/github/angular-rust/ux-charts?category=code
[loc-url]: https://github.com/angular-rust/ux-charts

**UX Charts** is a drawing library designed for clean charts. UX Charts supports various types of backend including GTK / Cairo and HTML5 Canvas. UX Charts are designed with the concept - `one code for all`. UX Charts uses the [UX Dataflow](https://github.com/angular-rust/ux-dataflow) library as the data source and the [UX Animate](https://github.com/angular-rust/ux-animate) library as the canvas implementation.

**UX Charts** is part of the Angular Rust framework.

**Angular Rust** is a high productivity, `platform-agnostic` frontend framework for the [Rust language](https://www.rust-lang.org/). It now supports desktop and web development. Angular Rust currently uses GTK for desktop development and WebAssembly for web development. We are planning to add support for mobile development.

## Gallery 

<img src="https://dudochkin-victor.github.io/assets/ux-charts/linechart-gtk.png" width="802" />
<img src="https://dudochkin-victor.github.io/assets/ux-charts/linechart-web.png" width="802" />

<img src="https://dudochkin-victor.github.io/assets/ux-charts/barchart-gtk.png" width="401" /><img src="https://dudochkin-victor.github.io/assets/ux-charts/gaugechart-gtk.png" width="401"/>

<img src="https://dudochkin-victor.github.io/assets/ux-charts/piechart-gtk.png" width="401" /><img src="https://dudochkin-victor.github.io/assets/ux-charts/radarchart-gtk.png" width="401" />



## Features

- [x] Various charts: barchart, linechart, piechart, gaugechart, and radarchart.
- [ ] Animation support
- [ ] Gradient fills
- [ ] User interaction

> The unimplemented features depend on `User-Experience` during the development of the [UX Animate](https://github.com/angular-rust/ux-animate) crate. So far, we have implemented basic features to visualize your data streams.

## Quick Start

Install UX Charts:

	cargo add ux-charts

## Learn More

* [Manual, Docs, etc](https://angular-rust.github.io/)
* [Samples](https://github.com/angular-rust/ux-samples)
* [Apps using Angular Rust](https://github.com/angular-rust/ux-charts/wiki/Apps-in-the-Wild)
* [Articles Featuring Angular Rust](https://github.com/angular-rust/ux-charts/wiki/Articles)

## Community

 [![](https://img.shields.io/badge/Facebook-1877F2?style=for-the-badge&logo=facebook&logoColor=white)](https://www.facebook.com/groups/angular.rust) 
 [![](https://img.shields.io/badge/Stack_Overflow-FE7A16?style=for-the-badge&logo=stack-overflow&logoColor=white)](https://stackoverflow.com/questions/tagged/angular-rust) 
 [![](https://img.shields.io/badge/YouTube-FF0000?style=for-the-badge&logo=youtube&logoColor=white)](https://www.youtube.com/channel/UCBJTkSl_JWShuolUy4JksTQ) 
 [![](https://img.shields.io/badge/Medium-12100E?style=for-the-badge&logo=medium&logoColor=white)](https://medium.com/@angular.rust) 
 [![](https://img.shields.io/gitter/room/angular_rust/angular_rust?style=for-the-badge)](https://gitter.im/angular_rust/community)


## Contributing

We believe the wider community can create better code. The first tool for improving the community is to tell the developers about the project by giving it a star. More stars - more members.

 ![Star a repo](https://dudochkin-victor.github.io/assets/star-me-wide.svg)
 
Angular Rust is a community effort and we welcome all kinds of contributions, big or small, from developers of all backgrounds. We want the Angular Rust community to be a fun and friendly place, so please review our [Code of Conduct](CODE_OF_CONDUCT.md) to learn what behavior will not be tolerated.

### New to Angular Rust?

Start learning about the framework by helping us improve our [documentation](https://angular-rust.github.io/). Pull requests which improve test coverage are also very welcome.

### Looking for inspiration?

Check out the community curated list of awesome things related to Angular Rust / WebAssembly at [awesome-angular-rust](https://github.com/angular-rust/awesome-angular-rust).

### Confused about something?

Feel free to drop into our [Gitter chatroom](https://gitter.im/angular_rust/community) or open a [new "Question" issue](https://github.com/angular-rust/ux-charts/issues/new/choose) to get help from contributors. Often questions lead to improvements to the ergonomics of the framework, better documentation, and even new features!

### Ready to dive into the code?

After reviewing the [Contributing Code Guidelines](CONTRIBUTING.md), check out the ["Good First Issues"](https://github.com/angular-rust/ux-charts/issues?q=is%3Aopen+is%3Aissue+label%3A%22good+first+issue%22) (they are eager for attention!). Once you find one that interests you, feel free to assign yourself to an issue and don't hesitate to reach out for guidance, the issues vary in complexity.

### Let's help each other!

Come help us on the [issues that matter that the most](https://github.com/angular-rust/ux-charts/labels/%3Adollar%3A%20Funded%20on%20Issuehunt) and receive a small cash reward for your troubles. We use [Issuehunt](https://issuehunt.io/r/angular-rust/ux-charts/) to fund issues from our Open Collective funds. If you really care about an issue, you can choose to add funds yourself! 

### Found a bug?

Please [report all bugs!](https://github.com/angular-rust/ux-charts/issues/new/choose) We are happy to help support developers fix the bugs they find if they are interested and have the time.

## Todo
- [ ] Documentation